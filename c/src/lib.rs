// "I want a new node, on cloudflare, called C"

// "relaye is dead. Make laye-net able to accept connections"

// 「[Opus 5.5] C is one laye peer in one Durable Object, Peer. Every WebSocket C
// takes is handed to that peer's laye-net, which takes it as a connection.」

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
use std::time::Duration;

use base64::Engine;
use laye_net::{Accepting, Net, NetEvent, Topic};
use worker::{
    DurableObject, Env, Request, Response, Result, State, WebSocketPair, console_error,
    console_log, durable_object,
};

const IDENTIFY: &str = "/laye/1.0.0";

// 「[Opus 5.5] The Worker in front of Peer is src/sentry.js, so Sentry wraps a
// plain handler.」

#[durable_object]
pub struct Peer {
    env: Env,
    held: RefCell<Option<Held>>,
}

struct Held {
    peer: String,
    dial: String,
    accepting: Accepting,
    recent: Recent,
}

/// The last laye events C said, newest last: what /health shows of the peer.
type Recent = Rc<RefCell<VecDeque<String>>>;
const RECENT: usize = 50;

fn keep(recent: &Recent, line: String) {
    let mut held = recent.borrow_mut();
    if held.len() == RECENT {
        held.pop_front();
    }
    held.push_back(line);
}

impl Peer {
    /// The peer, started the first time C is asked for it.
    fn started(&self) -> Result<()> {
        if self.held.borrow().is_some() {
            return Ok(());
        }
        let key = base64::engine::general_purpose::STANDARD
            .decode(self.env.secret("C_KEY")?.to_string().trim())
            .map_err(|e| worker::Error::RustError(format!("C_KEY is not base64: {e}")))?;
        let keypair = laye_me::load(&key)
            .map_err(|e| worker::Error::RustError(format!("C_KEY is not a laye key: {e}")))?;
        let listen = self.env.var("LISTEN")?.to_string();
        let topics = self
            .env
            .var("TOPICS")?
            .to_string()
            .split(',')
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .map(|t| Topic(t.to_string()))
            .collect();
        let (net, drive, accepting) =
            laye_net::new_accepting(keypair, IDENTIFY.into(), listen.clone(), topics)
                .map_err(|e| worker::Error::RustError(format!("the peer did not start: {e}")))?;
        let peer = net.identity().0.clone();
        wasm_bindgen_futures::spawn_local(drive);
        let recent: Recent = Rc::new(RefCell::new(VecDeque::new()));
        wasm_bindgen_futures::spawn_local(said(net, recent.clone()));
        console_log!(
            "{}",
            serde_json::json!({ "peer": peer, "dial": format!("{listen}/p2p/{peer}") })
        );
        *self.held.borrow_mut() = Some(Held {
            dial: format!("{listen}/p2p/{peer}"),
            peer,
            accepting,
            recent,
        });
        Ok(())
    }
}

/// Every laye event, one JSON line each in C's log, as laye-peer prints them.
async fn said(net: Net, recent: Recent) {
    loop {
        for event in net.poll_events() {
            match serde_json::to_string(&event) {
                Ok(line) => {
                    match event {
                        NetEvent::Error(_) | NetEvent::Unreachable { .. } => {
                            console_error!("{line}")
                        }
                        _ => console_log!("{line}"),
                    }
                    keep(&recent, line);
                }
                Err(e) => console_error!("a laye event did not serialize: {e}"),
            }
        }
        futures_timer::Delay::new(Duration::from_millis(200)).await;
    }
}

impl DurableObject for Peer {
    fn new(_state: State, env: Env) -> Self {
        Self {
            env,
            held: RefCell::new(None),
        }
    }

    async fn fetch(&self, req: Request) -> Result<Response> {
        self.started()?;
        if req.path() == "/peer" {
            let held = self.held.borrow();
            let Some(held) = held.as_ref() else {
                return Response::error("the peer is not held", 500);
            };
            return Response::from_json(
                &serde_json::json!({ "peer": held.peer, "dial": held.dial, "recent": *held.recent.borrow() }),
            );
        }
        if req.headers().get("Upgrade")?.as_deref() != Some("websocket") {
            return Response::error("C takes a WebSocket here", 426);
        }
        let pair = WebSocketPair::new()?;
        let socket: &web_sys::WebSocket = pair.server.as_ref();
        // Handed in first, so laye-net listens before the first byte can come.
        let taken = self
            .held
            .borrow()
            .as_ref()
            .is_some_and(|held| held.accepting.accept(socket.clone()));
        if !taken {
            return Response::error("the peer is gone", 503);
        }
        pair.server.accept()?;
        if let Some(held) = self.held.borrow().as_ref() {
            keep(&held.recent, "a WebSocket was handed to the peer".into());
        }
        Response::from_websocket(pair.client)
    }
}
