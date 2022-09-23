mod actors;
mod messages;
mod models;

use crate::actors::chat_session::WsChatSession;
use crate::{actors::chat_server::ChatServer, models::AppState};
use actix::Actor;
use actix_web::{web, App, HttpRequest, HttpServer};
use actix_web::{Error, HttpResponse};
use actix_web_actors::ws;
use simplelog::*;

use std::time::Duration;

/// How often heartbeat pings are sent
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub const SERVER_ADDR: &str = "0.0.0.0:8080";

#[actix_web::get("/chat")]
async fn web_socket_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let state: &AppState = req.app_data().unwrap();
    log::info!("connection requested /chat");
    ws::start(
        WsChatSession::new(state.chat.clone(), "12313123".to_string()),
        &req,
        stream,
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let chat = ChatServer::new().start();
    HttpServer::new(move || {
        App::new()
            .app_data(AppState { chat: chat.clone() })
            .service(web_socket_route)
    })
    .disable_signals()
    .bind(SERVER_ADDR)?
    .run()
    .await
}
