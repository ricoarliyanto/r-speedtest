use super::web_path::{WebPath, WebPathType};
use crate::service::net::s_net_alert::*;
use axum::routing::post;

pub fn router_alert() -> WebPath {
    WebPath::new().nest(
        "/net/alert",
        WebPath::new().route(
            "/test",
            WebPathType::Post,
            Some("Test NOC Alert Dispatch"),
            post(test_alert),
        ),
    )
}
