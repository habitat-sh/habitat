use crate::message;

impl message::MessageStatic for ApplicationEnvironment {
    const MESSAGE_ID: &'static str = "ApplicationEnvironment";
}
impl message::MessageStatic for PackageIdent {
    const MESSAGE_ID: &'static str = "PackageIdent";
}
impl message::MessageStatic for ChannelIdent {
    const MESSAGE_ID: &'static str = "ChannelIdent";
}
impl message::MessageStatic for ProcessStatus {
    const MESSAGE_ID: &'static str = "ProcessStatus";
}
impl message::MessageStatic for ServiceBind {
    const MESSAGE_ID: &'static str = "ServiceBind";
}
impl message::MessageStatic for ServiceCfg {
    const MESSAGE_ID: &'static str = "ServiceCfg";
}
impl message::MessageStatic for ServiceGroup {
    const MESSAGE_ID: &'static str = "ServiceGroup";
}
impl message::MessageStatic for ServiceStatus {
    const MESSAGE_ID: &'static str = "ServiceStatus";
}
impl message::MessageStatic for HealthCheckInterval {
    const MESSAGE_ID: &'static str = "HealthCheckInterval";
}
