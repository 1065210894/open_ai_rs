use axum::extract::{FromRequest, FromRequestParts};
use axum::http;
use axum::http::{Request, StatusCode};
use async_trait::async_trait;
use axum::http::request::Parts;

pub struct AuthenticationUser;

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticationUser
    where
        S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_agent = parts
            .headers
            .get(http::header::USER_AGENT)
            .and_then(|value| value.to_str().ok());

        match user_agent {
            Some(auth_value) if !auth_value.contains("Safari") => {
                Ok(Self)
            }
            _ => Err((StatusCode::UNAUTHORIZED, "Authentication failed".to_string())),
        }
    }
}