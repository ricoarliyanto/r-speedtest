use crate::common::result::ApiResponse;
use axum::{
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct VJson<T>(pub T);
 
impl<T, S> FromRequest<S> for VJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ServerError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(VJson(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),

    #[error(transparent)]
    MissingJsonContentType(#[from] axum::extract::rejection::MissingJsonContentType),

    #[error(transparent)]
    AxumQueryRejection(#[from] axum::extract::rejection::QueryRejection),

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                tracing::error!("{:?}", e); 
                ApiResponse::bad_request(e.to_string())
            }
            ServerError::AxumJsonRejection(e) => {
                tracing::error!("{:?}", e);
                ApiResponse::bad_request(e.to_string())
            }
            ServerError::MissingJsonContentType(e) => {
                tracing::error!("{:?}", e);
                ApiResponse::bad_request(e.to_string())
            }
            ServerError::AxumQueryRejection(e) => {
                tracing::error!("{:?}", e);
                ApiResponse::bad_request(e.to_string())
            }
            ServerError::AxumFormRejection(e) => {
                tracing::error!("{:?}", e);
                ApiResponse::bad_request(e.to_string())
            }
        }
        .into_response()
    }
}
