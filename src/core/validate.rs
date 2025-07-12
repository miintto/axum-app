use axum::{Json, extract::{Request, rejection::JsonRejection, FromRequest}};

use crate::core::error::ApiError;

pub struct ValidJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidJson<T>
where
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                match rejection {
                    JsonRejection::JsonDataError(_) => Err(ApiError::InvalidParameter),
                    _ => Err(ApiError::BadRequest),
                }
            }
        }
    }
}
