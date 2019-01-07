use crate::message;

impl message::MessageStatic for NetOk {
    const MESSAGE_ID: &'static str = "NetOk";
}
impl message::MessageStatic for NetErr {
    const MESSAGE_ID: &'static str = "NetErr";
}
