mod auth;
mod service;

use crate::{auth::DeviceAuthenticator, service::App};
use drogue_cloud_endpoint_common::{
    auth::AuthConfig,
    command::{Commands, KafkaCommandSource, KafkaCommandSourceConfig},
    sender::DownstreamSender,
    sink::KafkaSink,
};
use drogue_cloud_mqtt_common::server::{build, MqttServerOptions, TlsConfig};
use drogue_cloud_service_api::kafka::KafkaClientConfig;
use drogue_cloud_service_common::{
    defaults,
    health::{HealthServer, HealthServerConfig},
};
use futures::TryFutureExt;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub disable_tls: bool,
    #[serde(default)]
    pub cert_bundle_file: Option<String>,
    #[serde(default)]
    pub key_file: Option<String>,

    #[serde(default)]
    pub mqtt: MqttServerOptions,

    #[serde(default)]
    pub health: Option<HealthServerConfig>,

    pub auth: AuthConfig,

    pub command_source_kafka: KafkaCommandSourceConfig,

    pub kafka_downstream_config: KafkaClientConfig,
    pub kafka_command_config: KafkaClientConfig,

    pub instance: String,

    #[serde(default = "defaults::check_kafka_topic_ready")]
    pub check_kafka_topic_ready: bool,
}

impl TlsConfig for Config {
    fn is_disabled(&self) -> bool {
        self.disable_tls
    }

    #[cfg(feature = "rustls")]
    fn verifier_rustls(&self) -> std::sync::Arc<dyn rust_tls::server::ClientCertVerifier> {
        // This seems dangerous, as we simply accept all client certificates. However,
        // we validate them later during the "connect" packet validation.
        std::sync::Arc::new(auth::AcceptAllClientCertVerifier)
    }

    fn key_file(&self) -> Option<&str> {
        self.key_file.as_deref()
    }

    fn cert_bundle_file(&self) -> Option<&str> {
        self.cert_bundle_file.as_deref()
    }
}

pub async fn run(config: Config) -> anyhow::Result<()> {
    let commands = Commands::new();

    let app = App {
        downstream: DownstreamSender::new(
            KafkaSink::from_config(
                config.kafka_downstream_config.clone(),
                config.check_kafka_topic_ready,
            )?,
            config.instance.clone(),
        )?,

        authenticator: DeviceAuthenticator(
            drogue_cloud_endpoint_common::auth::DeviceAuthenticator::new(config.auth.clone())
                .await?,
        ),
        commands: commands.clone(),
    };

    let srv = build(config.mqtt.clone(), app, &config)?.run();

    log::info!("Starting web server");

    // command source

    let command_source = KafkaCommandSource::new(
        commands,
        config.kafka_command_config,
        config.command_source_kafka,
    )?;

    // run
    if let Some(health) = config.health {
        // health server
        let health = HealthServer::new(health, vec![Box::new(command_source)]);
        futures::try_join!(health.run_ntex(), srv.err_into(),)?;
    } else {
        futures::try_join!(srv)?;
    }

    // exiting

    Ok(())
}
