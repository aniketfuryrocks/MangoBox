use actix::Message as ActixMessage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ActixMessage)]
#[rtype(result = "()")]
pub struct WsMessage {
    pub ty: MessageType,
    pub data: String,
}

impl WsMessage {
    pub fn msg<T: ToString>(data: T) -> Self {
        WsMessage {
            ty: MessageType::Msg,
            data: data.to_string(),
        }
    }

    pub fn err<T: ToString>(data: T) -> Self {
        WsMessage {
            ty: MessageType::Err,
            data: data.to_string(),
        }
    }

    pub fn info<T: ToString>(data: T) -> Self {
        WsMessage {
            ty: MessageType::Info,
            data: data.to_string(),
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
