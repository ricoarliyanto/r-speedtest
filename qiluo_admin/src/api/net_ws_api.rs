use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::interval;
use tracing::{debug, info};
use crate::cache::CacheManager;

#[derive(Deserialize, Serialize, Debug)]
struct WsIncoming {
    action: Option<String>,
    topic: Option<String>,
}

pub fn router_net_ws() -> Router {
    Router::new().route("/net/ws", get(ws_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    info!("New WebSocket client connected to Network Telemetry hub");

    // Send welcome frame
    let welcome = serde_json::json!({
        "event": "connected",
        "message": "Connected to Network Swiss Army Telemetry Hub",
        "timestamp": chrono::Utc::now().timestamp_millis()
    });
    let _ = socket.send(Message::Text(welcome.to_string().into())).await;

    let mut subscribed_topic: Option<String> = None;
    let mut ticker = interval(Duration::from_millis(500));

    loop {
        tokio::select! {
            // Check for incoming client messages (e.g. subscribe to a job)
            msg_opt = socket.recv() => {
                match msg_opt {
                    Some(Ok(Message::Text(text))) => {
                        if let Ok(incoming) = serde_json::from_str::<WsIncoming>(&text) {
                            if incoming.action.as_deref() == Some("subscribe") {
                                subscribed_topic = incoming.topic;
                                debug!("Client subscribed to topic: {:?}", subscribed_topic);
                                let ack = serde_json::json!({
                                    "event": "subscribed",
                                    "topic": subscribed_topic
                                });
                                let _ = socket.send(Message::Text(ack.to_string().into())).await;
                            }
                        }
                    }
                    Some(Ok(Message::Ping(p))) => {
                        let _ = socket.send(Message::Pong(p)).await;
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        info!("WebSocket client disconnected");
                        break;
                    }
                    _ => {}
                }
            }

            // Periodic check for subscribed job updates from DragonflyDB Cache
            _ = ticker.tick() => {
                if let Some(ref topic) = subscribed_topic {
                    if topic.starts_with("job:") {
                        let job_id = &topic["job:".len()..];
                        let cache = CacheManager::instance().await;
                        if let Ok(Some(cached_str)) = cache.get_string(&format!("job_result:{}", job_id)).await.map(Some) {
                            let _ = socket.send(Message::Text(cached_str.into())).await;
                        }
                    }
                }
            }
        }
    }
}
