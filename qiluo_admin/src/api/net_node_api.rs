use super::web_path::{WebPath, WebPathType};
use crate::service::net::s_net_node::*;
use axum::routing::{get, post};
use axum::Router;

pub fn router_node() -> WebPath {
    WebPath::new().nest(
        "/net/node",
        WebPath::new()
            .route(
                "/register",
                WebPathType::Post,
                Some("Register Edge Node"),
                post(register_node),
            )
            .route(
                "/list",
                WebPathType::Get,
                Some("List Edge Nodes"),
                get(list_nodes),
            )
            .route(
                "/delete",
                WebPathType::Post,
                Some("Delete Edge Node"),
                post(delete_node),
            ),
    )
}

pub fn router_node_public() -> Router {
    Router::new().route("/net/node/heartbeat", post(node_heartbeat))
}
