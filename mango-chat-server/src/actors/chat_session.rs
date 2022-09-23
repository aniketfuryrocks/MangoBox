use crate::messages::{
    server::{ClientMessage, Connect, Disconnect, JoinRoom, Leave},
    session::{
        wsmessage::{MessageType, WsMessage},
        Message,
    },
};
use crate::{
    actors::chat_server::ChatServer,
    models::{RoomId, SessionId},
};
use actix::{clock::Instant, ActorContext, Handler, Running, StreamHandler};
use actix::{Actor, Addr, AsyncContext};
use actix_web_actors::ws::{self, WebsocketContext};

/// actor to every unique connection
pub struct WsChatSession {
    pub id: SessionId,
    pub room: Option<RoomId>,
    pub chat_server: Addr<ChatServer>,
    pub hb: Instant,
}

impl WsChatSession {
    pub fn new(addr: Addr<ChatServer>, id: SessionId) -> Self {
        WsChatSession {
            id,
            room: None,
            hb: Instant::now(),
            chat_server: addr,
        }
    }
}

impl Actor for WsChatSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let chat_server = self.chat_server.clone();
        let id = self.id.clone();

        ctx.spawn(actix::fut::wrap_future::<_, Self>(async move {
            chat_server
                .send(Connect {
                    id,
                    addr: addr.recipient(),
                })
                .await
                .unwrap();
        }));

        // start sending heartbeats
        ctx.run_interval(crate::HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > crate::CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");
                // notify chat server
                act.chat_server.do_send(Disconnect {
                    session: act.id.clone(),
                });
                // stop actor
                ctx.stop();
                // don't try to send a ping
                return;
            }
            ctx.ping(b"");
        });
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.chat_server.do_send(Disconnect {
            session: self.id.clone(),
        });
        Running::Stop
    }
}

/// handler for ws::Message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // send a pong back with the same message
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => self.hb = Instant::now(),
            Ok(ws::Message::Text(text)) => {
                match serde_json::from_str::<WsMessage>(&text) {
                    Ok(ws_message) => ctx.notify(ws_message),
                    Err(err) => ctx.text(WsMessage::err(&err.to_string()).to_string()),
                };
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl Handler<WsMessage> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        log::info!("{}", msg.to_string());
        let data = msg.data.to_string();
        match msg.ty {
            MessageType::Join => self.join(data, ctx),
            MessageType::Msg => self.msg(serde_json::Value::String(data), ctx),
            MessageType::Leave => self.leave(ctx),
            _ => ctx.text(msg.to_string()),
        }
    }
}

impl Handler<Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(WsMessage::msg(&msg.msg).to_string());
    }
}

impl WsChatSession {
    fn msg(&mut self, data: serde_json::Value, ctx: &mut WebsocketContext<Self>) {
        let room = if let Some(room) = &self.room {
            room.to_owned()
        } else {
            ctx.text(WsMessage::err("Join A Room First").to_string());
            return;
        };

        self.chat_server.do_send(ClientMessage {
            room: room.clone(),
            session: self.id.clone(),
            msg: data.to_string(),
        });
        self.room = Some(room);
        ctx.text(WsMessage::info("Room Joined").to_string());
    }

    fn join(&mut self, room_id: RoomId, ctx: &mut WebsocketContext<Self>) {
        self.chat_server.do_send(JoinRoom {
            room: room_id.clone(),
            session: self.id.clone(),
        });
        self.room = Some(room_id);
        ctx.text(WsMessage::info("Room Joined").to_string());
    }

    fn leave(&mut self, ctx: &mut WebsocketContext<Self>) {
        self.chat_server.do_send(Leave {
            session: self.id.clone(),
        });
        self.room = None;
        ctx.text(WsMessage::info("Room leaved").to_string())
    }
}
