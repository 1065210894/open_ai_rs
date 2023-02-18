use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http;
use axum::http::request::Parts;
use axum::http::StatusCode;

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
            Some(auth_value) if !auth_value.contains("Firefox") => Ok(Self),
            _ => Err((
                StatusCode::UNAUTHORIZED,
                "Authentication failed".to_string(),
            )),
        }
    }
}
