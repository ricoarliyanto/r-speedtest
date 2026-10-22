use super::validatedjson::ServerError;
use axum::{
    extract::{Form, FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct VForm<T>(pub T);


impl<T, S> FromRequest<S> for VForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = ServerError;
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(VForm(value))
    }
}
