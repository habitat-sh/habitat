use message;

impl message::MessageStatic for NetProgress {
    const MESSAGE_ID: &'static str = "NetProgress";
}
impl message::MessageStatic for Handshake {
    const MESSAGE_ID: &'static str = "Handshake";
}
impl message::MessageStatic for ServiceBindList {
    const MESSAGE_ID: &'static str = "ServiceBindList";
}
impl message::MessageStatic for SupDepart {
    const MESSAGE_ID: &'static str = "SupDepart";
}
impl message::MessageStatic for SvcFilePut {
    const MESSAGE_ID: &'static str = "SvcFilePut";
}
impl message::MessageStatic for SvcGetDefaultCfg {
    const MESSAGE_ID: &'static str = "SvcGetDefaultCfg";
}
impl message::MessageStatic for SvcValidateCfg {
    const MESSAGE_ID: &'static str = "SvcValidateCfg";
}
impl message::MessageStatic for SvcSetCfg {
    const MESSAGE_ID: &'static str = "SvcSetCfg";
}
impl message::MessageStatic for SvcLoad {
    const MESSAGE_ID: &'static str = "SvcLoad";
}
impl message::MessageStatic for SvcUnload {
    const MESSAGE_ID: &'static str = "SvcUnload";
}
impl message::MessageStatic for SvcStart {
    const MESSAGE_ID: &'static str = "SvcStart";
}
impl message::MessageStatic for SvcStop {
    const MESSAGE_ID: &'static str = "SvcStop";
}
impl message::MessageStatic for SvcStatus {
    const MESSAGE_ID: &'static str = "SvcStatus";
}
impl message::MessageStatic for ConsoleLine {
    const MESSAGE_ID: &'static str = "ConsoleLine";
}
