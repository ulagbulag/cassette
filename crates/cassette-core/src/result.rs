#[cfg(feature = "api")]
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "result", content = "spec")]
pub enum HttpResult<T> {
    Ok(T),
    Err(String),
}

impl<T, E> From<::core::result::Result<T, E>> for HttpResult<T>
where
    E: ToString,
{
    fn from(value: ::core::result::Result<T, E>) -> Self {
        match value {
            Ok(value) => Self::Ok(value),
            Err(error) => Self::Err(error.to_string()),
        }
    }
}

#[cfg(feature = "api")]
impl<T> From<HttpResult<T>> for HttpResponse
where
    T: Serialize,
{
    fn from(value: HttpResult<T>) -> Self {
        match value {
            HttpResult::Ok(_) => HttpResponse::Ok().json(value),
            HttpResult::Err(_) => HttpResponse::MethodNotAllowed().json(value),
        }
    }
}
