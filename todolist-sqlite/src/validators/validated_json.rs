use axum::{
    Json,
    extract::{FromRequest, Request},
};
use validator::Validate;

use crate::model::api_exception::ApiException;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + Validate,
    S: Send + Sync
{
    type Rejection = ApiException;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| ApiException::BadRequest(e.to_string()))?;

        value.validate()
            .map_err(|e| ApiException::BadRequest(e.to_string()))?;

        Ok(ValidatedJson(value))
    }
}
