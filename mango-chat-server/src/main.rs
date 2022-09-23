mod actors;
mod messages;
mod models;

use crate::actors::chat_session::WsChatSession;
use crate::{actors::chat_server::ChatServer, models::AppState};
use actix::Actor;
use actix_web::{get, web, App, HttpRequest, HttpServer, Responder};
use actix_web_actors::ws;

use std::time::Duration;

/// How often heartbeat pings are sent
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub const SERVER_ADDR: &str = "0.0.0.0:8080";

#[get("/")]
pub async fn connect(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<AppState>,
) -> impl Responder {
    let chat = state.chat.clone();
    ws::start(
        WsChatSession::new(chat, "12313123".to_string()),
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let chat = ChatServer::new().start();
        App::new().app_data(AppState { chat }).service(connect)
    })
    .bind(SERVER_ADDR)?
    .run()
    .await
}
