use crate::command::{Command, CommandDispatcher};
use async_trait::async_trait;
use drogue_cloud_event_common::stream::{
    AutoAck, EventStream, EventStreamConfig, EventStreamError,
};
use drogue_cloud_service_api::{
    health::{HealthCheckError, HealthChecked},
    kafka::{KafkaClientConfig, KafkaConfig},
};
use futures::StreamExt;
use serde::Deserialize;
use std::sync::Arc;
use std::{
    convert::TryFrom,
    sync::atomic::{AtomicBool, Ordering},
};
use tokio::task::JoinHandle;

#[derive(Clone, Debug, Deserialize)]
pub struct KafkaCommandSourceConfig {
    pub topic: String,
    pub consumer_group: String,
}

pub struct KafkaCommandSource {
    handle: JoinHandle<()>,
    alive: Arc<AtomicBool>,
}

impl KafkaCommandSource {
    pub fn new<D>(
        dispatcher: D,
        kafka_client: KafkaClientConfig,
        config: KafkaCommandSourceConfig,
    ) -> Result<Self, EventStreamError>
    where
        D: CommandDispatcher + Send + Sync + 'static,
    {
        let mut source = EventStream::<AutoAck>::new(EventStreamConfig {
            kafka: KafkaConfig {
                topic: config.topic,
                client: kafka_client,
            },
            consumer_group: Some(config.consumer_group),
        })?;

        let alive = Arc::new(AtomicBool::new(true));
        let a = alive.clone();

        let handle = tokio::spawn(async move {
            while let Some(event) = source.next().await {
                log::debug!("Command event: {:?}", event);
                match event {
                    Ok(event) => match Command::try_from(event) {
                        Ok(command) => dispatcher.send(command).await,
                        Err(_) => {
                            log::info!("Failed to convert event to command");
                        }
                    },
                    Err(err) => {
                        log::info!("Failed to read next event: {}", err);
                    }
                }
            }
            log::info!("Exiting event loop!");
            a.store(false, Ordering::Relaxed);
        });

        Ok(Self { handle, alive })
    }
}

impl Drop for KafkaCommandSource {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

#[async_trait]
impl HealthChecked for KafkaCommandSource {
    async fn is_alive(&self) -> Result<(), HealthCheckError> {
        if self.alive.load(Ordering::Relaxed) {
            Ok(())
        } else {
            HealthCheckError::nok("Event loop is not alive")
        }
    }
}
