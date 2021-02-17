use crate::command::CommandWait;
use crate::downstream::HttpCommandSender;
use actix_web::{post, web, HttpResponse};
use drogue_cloud_endpoint_common::{
    auth::DeviceAuthenticator,
    downstream::{DownstreamSender, Publish},
    error::{EndpointError, HttpEndpointError},
    x509::ClientCertificateChain,
};
use drogue_cloud_service_api::auth;
use drogue_cloud_service_api::management::Device;
use drogue_ttn::http as ttn;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PublishOptions {
    pub tenant: Option<String>,
    pub device: Option<String>,

    pub model_id: Option<String>,
    pub ttd: Option<u64>,
}

#[post("")]
pub async fn publish(
    sender: web::Data<DownstreamSender>,
    auth: web::Data<DeviceAuthenticator>,
    web::Query(opts): web::Query<PublishOptions>,
    req: web::HttpRequest,
    body: web::Bytes,
    cert: Option<ClientCertificateChain>,
) -> Result<HttpResponse, HttpEndpointError> {
    let (application, device) = match auth
        .authenticate_http(
            opts.tenant,
            opts.device,
            req.headers().get(http::header::AUTHORIZATION),
            cert.map(|c| c.0),
        )
        .await
        .map_err(|err| HttpEndpointError(err.into()))?
        .outcome
    {
        auth::Outcome::Fail => return Err(HttpEndpointError(EndpointError::AuthenticationError)),
        auth::Outcome::Pass {
            application,
            device,
        } => (application, device),
    };

    let uplink: ttn::Uplink = serde_json::from_slice(&body).map_err(|err| {
        log::info!("Failed to decode payload: {}", err);
        EndpointError::InvalidFormat {
            source: Box::new(err),
        }
    })?;

    log::info!(
        "Application / Device properties: {:?} / {:?}",
        application,
        device
    );

    // eval model_id from query and function port mapping
    let model_id = eval_model_id(opts.model_id.as_ref().cloned(), &device, &uplink);

    let device_id = uplink.dev_id;

    log::info!("Device ID: {}, Model ID: {:?}", device_id, model_id);

    // FIXME: need to authorize device

    sender
        .publish_and_await(
            Publish {
                channel: uplink.port.to_string(),
                device_id,
                model_id,
                ..Default::default()
            },
            CommandWait::from_secs(opts.ttd),
            body,
        )
        .await
}

fn eval_model_id(
    model_id: Option<String>,
    device: &Device,
    uplink: &ttn::Uplink,
) -> Option<String> {
    model_id.or_else(|| {
        let fport = uplink.port.to_string();
        device.spec.get("lorawan").and_then(|spec| {
            spec["ports"][fport]["model_id"]
                .as_str()
                .map(|str| str.to_string())
        })
    })
}

#[cfg(test)]
mod test {

    use super::*;
    use chrono::Utc;
    use drogue_ttn::http::Metadata;
    use serde_json::{json, Map, Value};

    #[test]
    fn test_model_mapping() {
        let lorawan_spec = json!({
            "ports": {
             "1": { "model_id": "mod1",},
             "5": {"model_id": "mod5",},
            }
        });

        let device = device(Some(lorawan_spec));
        let uplink = default_uplink(5);

        let model_id = eval_model_id(None, &device, &uplink);

        assert_eq!(model_id, Some(String::from("mod5")));
    }

    #[test]
    fn test_model_no_mapping_1() {
        let device = device(None);
        let uplink = default_uplink(5);

        let model_id = eval_model_id(None, &device, &uplink);

        assert_eq!(model_id, None);
    }

    #[test]
    fn test_model_no_mapping_2() {
        let device = device(Some(json!({
            "ports": { "1": {"model_id": "mod1"}}
        })));
        let uplink = default_uplink(5);

        let model_id = eval_model_id(None, &device, &uplink);

        assert_eq!(model_id, None);
    }

    #[test]
    fn test_model_no_mapping_3() {
        let device = device(Some(json!({
            "ports": { "1": {"no_model_id": "mod1"}}
        })));
        let uplink = default_uplink(5);

        let model_id = eval_model_id(None, &device, &uplink);

        assert_eq!(model_id, None);
    }

    fn device(lorawan_spec: Option<Value>) -> Device {
        let mut spec = Map::new();
        if let Some(lorawan_spec) = lorawan_spec {
            spec.insert("lorawan".into(), lorawan_spec);
        }
        Device {
            metadata: Default::default(),
            spec,
            status: Default::default(),
        }
    }

    fn default_uplink(port: u16) -> ttn::Uplink {
        ttn::Uplink {
            app_id: "".to_string(),
            dev_id: "".to_string(),
            hardware_serial: "".to_string(),
            port,
            counter: 0,
            is_retry: false,
            confirmed: false,
            payload_raw: vec![],
            metadata: Metadata {
                time: Utc::now(),
                frequency: 0.0,
                modulation: "".to_string(),
                data_rate: None,
                bit_rate: None,
                coding_rate: "".to_string(),
                coordinates: None,
                gateways: vec![],
            },
            downlink_url: None,
        }
    }
}
