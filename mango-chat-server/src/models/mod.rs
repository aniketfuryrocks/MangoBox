use crate::actors::chat_server::ChatServer;
use actix::Addr;

pub type SessionId = String;
pub type RoomId = String;

pub struct AppState {
    pub chat: Addr<ChatServer>,
}
