use actix::Message as ActixMessage;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, ActixMessage)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub ty: MessageType,
    pub data: Value,
}

impl WsMessage {
    pub fn msg(msg: &str) -> Self {
        WsMessage {
            ty: MessageType::Msg,
            data: json!(msg),
        }
    }
    pub fn err(msg: &str) -> Self {
        WsMessage {
            ty: MessageType::Err,
            data: json!(msg),
        }
    }

    pub fn info(msg: &str) -> Self {
        WsMessage {
            ty: MessageType::Info,
            data: json!(msg),
        }
    }
}

impl ToString for WsMessage {
    fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Deserialize)]
pub enum MessageType {
    Join,
    Leave,
    Msg,
    Err,
    Info,
}
