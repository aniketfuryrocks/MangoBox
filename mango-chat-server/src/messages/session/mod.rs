pub mod wsmessage;
use actix::Message as ActixMessage;
use serde::Serialize;

use crate::models::SessionId;

#[derive(Serialize, ActixMessage)]
#[rtype(result = "()")]
pub struct Message {
    ///TODO: make this proper
    pub wallet_pk: Option<SessionId>,
    pub msg: String,
}
