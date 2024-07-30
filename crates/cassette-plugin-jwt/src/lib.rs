use actix_web::{http, HttpRequest};
use base64::Engine;
use serde::de::DeserializeOwned;

pub const HEADER_AUTHORIZATION: &str = "Authorization";

#[derive(Debug, ::thiserror::Error)]
pub enum Error {
    #[error("Authorization token has not found")]
    AuthorizationTokenNotFound,
    #[error("Failed to parse the Authorization token: {0}")]
    AuthorizationTokenParseError(http::header::ToStrError),
    #[error("Authorization token is not a Bearer token")]
    BearerTokenNotFound,
    #[error("Failed to decode the Bearer token: {0}")]
    BearerTokenDecodeFailed(::base64::DecodeError),
    #[error("Failed to parse the Bearer token: {0}")]
    BearerTokenParseFailed(::serde_json::Error),
}

pub fn get_authorization_token(request: &HttpRequest) -> Result<&str, Error> {
    request
        .headers()
        .get(HEADER_AUTHORIZATION)
        .ok_or(Error::AuthorizationTokenNotFound)
        .and_then(|token| token.to_str().map_err(Error::AuthorizationTokenParseError))
        .and_then(|token| {
            token
                .strip_prefix("Bearer ")
                .ok_or(Error::BearerTokenNotFound)
        })
}

pub fn parse_jwt<T>(token: &str) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    token
        .split('.')
        .nth(1)
        .ok_or(Error::BearerTokenNotFound)
        .and_then(|payload| {
            ::base64::engine::general_purpose::STANDARD_NO_PAD
                .decode(payload)
                .map_err(Error::BearerTokenDecodeFailed)
        })
        .and_then(|payload| {
            ::serde_json::from_slice(&payload).map_err(Error::BearerTokenParseFailed)
        })
}
