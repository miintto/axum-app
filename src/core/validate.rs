use axum::extract::{Request, rejection::JsonRejection, FromRequest};

use crate::core::http::Http4xx;

pub struct Json<T>(pub T);

impl<S, T> FromRequest<S> for Json<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = Http4xx;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                match rejection {
                    JsonRejection::JsonDataError(_) => Err(Http4xx::InvalidParameter),
                    _ => Err(Http4xx::BadRequest),
                }
            }
        }
    }
}
