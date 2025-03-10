use crate::{
    backend::Token,
    data::{SharedDataDispatcher, SharedDataOps},
    examples::{data::ExampleData, note_local_certs},
    html_prop,
    utils::{shell_quote, shell_single_quote, url_encode},
};
use drogue_cloud_service_api::endpoints::Endpoints;
use patternfly_yew::*;
use yew::prelude::*;

#[derive(Clone, Debug, Properties, PartialEq, Eq)]
pub struct Props {
    pub endpoints: Endpoints,
    pub data: ExampleData,
    pub token: Token,
}

pub struct CommandAndControl {
    data_agent: SharedDataDispatcher<ExampleData>,
}

#[derive(Clone, Debug)]
pub enum Msg {
    SetDrgToken(bool),
    SetCommandEmptyMessage(bool),
    SetCommandName(String),
    SetCommandPayload(String),
}

impl Component for CommandAndControl {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            data_agent: SharedDataDispatcher::new(),
        }
    }

    fn update(&mut self, _: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::SetCommandEmptyMessage(cmd_empty_message) => self
                .data_agent
                .update(move |data| data.cmd_empty_message = cmd_empty_message),
            Self::Message::SetDrgToken(drg_token) => self
                .data_agent
                .update(move |data| data.drg_token = drg_token),
            Self::Message::SetCommandName(name) => {
                self.data_agent.update(|mut data| data.cmd_name = name)
            }
            Self::Message::SetCommandPayload(payload) => self
                .data_agent
                .update(|mut data| data.cmd_payload = payload),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut cards: Vec<_> = vec![html! {
            <Alert
                title="Command & control"
                r#type={Type::Info} inline=true
                >
                <Content>
                <p>
                    {r#"Command & control, also known as "cloud-to-device messaging", is used to send messages back to a device. In order to test this,
                    you will need simulate a device, connecting to the cloud, and at the same time, a cloud side application, which sends data to a device"#}
                </p>

                <p>
                    {"For this you will need to have two different terminals open at the same time."}
                </p>

                </Content>
            </Alert>
        }];

        let local_certs = ctx
            .props()
            .data
            .local_certs(ctx.props().endpoints.local_certs);

        if let Some(http) = &ctx.props().endpoints.http {
            let payload = match ctx.props().data.cmd_empty_message {
                true => "".into(),
                false => format!(
                    "echo {payload} | ",
                    payload = shell_single_quote(&ctx.props().data.payload)
                ),
            };
            let publish_http_cmd = format!(
                r#"{payload}http --auth {auth} {certs}POST {url}/v1/foo?ct=30"#,
                payload = payload,
                url = http.url,
                auth = shell_quote(format!(
                    "{device_id}@{app_id}:{password}",
                    app_id = ctx.props().data.app_id,
                    device_id = url_encode(&ctx.props().data.device_id),
                    password = &ctx.props().data.password,
                )),
                certs = local_certs
                    .then(|| "--verify build/certs/endpoints/root-cert.pem ")
                    .unwrap_or("")
            );
            cards.push(html!{
                <Card title={html_prop!({"Receive commands using HTTP long-polling"})}>
                    <div>
                        {r#"
                        A device can receive commands using HTTP long-polling, when it publishes data to the cloud. To do this, a device needs to inform the HTTP endpoint,
                        that it will wait for some seconds for the cloud to receive a command message, which then gets reported in the response of the publish message.
                        "#}
                    </div>
                    <div>
                        <Switch
                            checked={ctx.props().data.cmd_empty_message}
                            label="Send empty message" label_off="Send example payload"
                            on_change={ctx.link().callback(|data| Msg::SetCommandEmptyMessage(data))}
                            />
                    </div>
                    <Alert title="Hurry up!" inline=true>
                        {r#"
                        This example will wait 30 seconds for the cloud side to send a command. If you don't execute the "send command" step before this timeout
                        expires, the device will stop waiting and return with an empty response.
                        "#}
                    </Alert>
                    <Clipboard code=true readonly=true variant={ClipboardVariant::Expandable} value={publish_http_cmd}/>
                    {note_local_certs(local_certs)}
                </Card>
            });
        }

        if let Some(coap) = &ctx.props().endpoints.coap {
            let payload = match ctx.props().data.cmd_empty_message {
                true => "".into(),
                false => format!(
                    "echo {payload} | ",
                    payload = shell_single_quote(&ctx.props().data.payload)
                ),
            };
            let publish_http_cmd = format!(
                r#"{payload}coap POST -O '4209,Basic {auth}' {url}/v1/foo?ct=30"#,
                payload = payload,
                url = coap.url,
                auth = shell_quote(base64::encode_config(
                    format!(
                        "{device_id}@{app_id}:{password}",
                        app_id = ctx.props().data.app_id,
                        device_id = url_encode(&ctx.props().data.device_id),
                        password = &ctx.props().data.password,
                    ),
                    base64::STANDARD_NO_PAD
                )),
            );
            cards.push(html!{
                <Card title={html_prop!({"Receive commands using CoAP long-polling"})}>
                    <div>
                        {r#"
                        A device can receive commands using CoAP long-polling, when it publishes data to the cloud. To do this, a device needs to inform the CoAP endpoint,
                        that it will wait for some seconds for the cloud to receive a command message, which then gets reported in the response of the publish message.
                        "#}
                    </div>
                    <div>
                        <Switch
                            checked={ctx.props().data.cmd_empty_message}
                            label="Send empty message" label_off="Send example payload"
                            on_change={ctx.link().callback(|data| Msg::SetCommandEmptyMessage(data))}
                            />
                    </div>
                    <Alert title="Hurry up!" inline=true>
                        {r#"
                        This example will wait 30 seconds for the cloud side to send a command. If you don't execute the "send command" step before this timeout
                        expires, the device will stop waiting and return with an empty response.
                        "#}
                    </Alert>
                    <Clipboard code=true readonly=true variant={ClipboardVariant::Expandable} value={publish_http_cmd} />
                    {note_local_certs(local_certs)}
                </Card>
            });
        }

        if let Some(mqtt) = &ctx.props().endpoints.mqtt {
            let publish_mqtt_cmd = format!(
                r#"mqtt sub -h {host} -p {port} -u '{device_id}@{app_id}' -pw '{password}' -s {certs}-t command/inbox/#"#,
                host = mqtt.host,
                port = mqtt.port,
                app_id = &ctx.props().data.app_id,
                device_id = shell_quote(url_encode(&ctx.props().data.device_id)),
                password = shell_quote(&ctx.props().data.password),
                certs = local_certs
                    .then(|| "--cafile build/certs/endpoints/root-cert.pem ")
                    .unwrap_or("")
            );
            cards.push(html!{
                <Card title={html_prop!({"Receive commands using MQTT subscriptions"})}>
                    <div>
                        {"Using MQTT, you can simply subscribe to commands."}
                    </div>
                    <Clipboard code=true readonly=true variant={ClipboardVariant::Expandable} value={publish_mqtt_cmd}/>
                    {note_local_certs(local_certs)}
                </Card>
            });
        }

        if let Some(cmd) = &ctx.props().endpoints.command_url {
            let v = |value: &str| match value {
                "" => InputState::Error,
                _ => InputState::Default,
            };
            let token = match ctx.props().data.drg_token {
                true => "$(drg whoami -t)",
                false => ctx.props().token.access_token.as_str(),
            };
            let send_command_cmd = format!(
                r#"echo {payload} | http POST {url}/api/command/v1alpha1/apps/{app}/devices/{device} command=={cmd} "Authorization:Bearer {token}""#,
                payload = shell_single_quote(&ctx.props().data.cmd_payload),
                url = cmd,
                app = url_encode(&ctx.props().data.app_id),
                device = url_encode(&ctx.props().data.device_id),
                token = token,
                cmd = shell_quote(&ctx.props().data.cmd_name),
            );
            cards.push(html!{
                <Card title={html_prop!({"Send a command"})}>
                    <div>
                        {r#"
                        Once the device is waiting for commands, you can send one.
                        "#}
                    </div>
                    <Form>
                        <FormGroup label="Command name">
                            <TextInput
                                value={ctx.props().data.cmd_name.clone()}
                                required=true
                                onchange={ctx.link().callback(|name|Msg::SetCommandName(name))}
                                validator={Validator::from(v)}
                                />
                        </FormGroup>
                        <FormGroup label="Command payload">
                            <TextArea
                                value={ctx.props().data.cmd_payload.clone()}
                                onchange={ctx.link().callback(|payload|Msg::SetCommandPayload(payload))}
                                />
                        </FormGroup>
                        <FormGroup>
                            <Switch
                                checked={ctx.props().data.drg_token}
                                label="Use 'drg' to get the access token" label_off="Show current token in example"
                                on_change={ctx.link().callback(|data| Msg::SetDrgToken(data))}
                                />
                        </FormGroup>
                    </Form>
                    <Clipboard code=true readonly=true variant={ClipboardVariant::Expandable} value={send_command_cmd} />
                </Card>
            });
        }

        cards
            .iter()
            .map(|card| {
                html! {<StackItem> { card.clone() } </StackItem>}
            })
            .collect()
    }
}
