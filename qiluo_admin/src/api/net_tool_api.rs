use super::web_path::{WebPath, WebPathType};
use crate::service::net::s_net_tool::*;
use axum::routing::{get, post};

pub fn router_net() -> WebPath {
    WebPath::new().nest(
        "/net",
        WebPath::new()
            .nest(
                "/tool",
                WebPath::new()
                    .route(
                        "/list",
                        WebPathType::Get,
                        Some("List Network Tools"),
                        get(list_tools),
                    )
                    .route(
                        "/run",
                        WebPathType::Post,
                        Some("Run Diagnostic Tool"),
                        post(run_tool),
                    ),
            )
            .nest(
                "/job",
                WebPath::new()
                    .route(
                        "/list",
                        WebPathType::Get,
                        Some("List Diagnostic Jobs"),
                        get(list_jobs),
                    )
                    .route(
                        "/get",
                        WebPathType::Get,
                        Some("Get Job Details"),
                        get(get_job),
                    )
                    .route(
                        "/cancel",
                        WebPathType::Post,
                        Some("Cancel Job"),
                        post(cancel_job),
                    )
                    .route(
                        "/artifact",
                        WebPathType::Get,
                        Some("Download Job Artifact"),
                        get(download_artifact),
                    ),
            ),
    )
}
