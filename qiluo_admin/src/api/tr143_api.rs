use axum::{
    routing::{get, post},
    Router,
};
use crate::tools::tr143::{tr143_download_handler, tr143_upload_handler};

pub fn router_tr143_public() -> Router {
    Router::new()
        .nest(
            "/diagnostics/tr143",
            Router::new()
                .route("/download", get(tr143_download_handler))
                .route("/upload", post(tr143_upload_handler)),
        )
}
