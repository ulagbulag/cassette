use std::collections::BTreeMap;

use actix_web::{
    body::BoxBody,
    http::StatusCode,
    web::{Path, Payload, Query},
    HttpRequest, HttpResponse, Responder, Scope,
};
use anyhow::{anyhow, Result};
use http::Request;
use kube::{core::ErrorResponse, Client};
use url::Url;

pub fn build_services(scope: Scope) -> Scope {
    scope.route("/kube/{path:.*}", ::actix_web::web::route().to(handle))
}

async fn handle(
    request: HttpRequest,
    payload: Payload,
    uri: Path<String>,
    queries: Query<BTreeMap<String, String>>,
) -> impl Responder {
    match load_client(&request).await {
        Ok(UserClient { kube: client, .. }) => {
            try_handle(&request, client, uri.into_inner(), queries.0, payload)
                .await
                .unwrap_or_else(|error| response_error(StatusCode::METHOD_NOT_ALLOWED, error))
        }
        Err(error) => response_error(StatusCode::UNAUTHORIZED, error),
    }
}

async fn try_handle(
    request: &HttpRequest,
    client: Client,
    uri: String,
    queries: BTreeMap<String, String>,
    payload: Payload,
) -> Result<HttpResponse> {
    let method = request.method().as_str();
    let body = payload
        .to_bytes()
        .await
        .map_err(|error| anyhow!("{error}"))?
        .into();

    // NOTE: scheme and base_url would be ignored
    let base_url = "https://kubernetes.default.svc";
    let url = Url::parse_with_params(&format!("{base_url}/{uri}"), queries)?.to_string();
    let uri = &url[base_url.len()..];

    let mut builder = Request::builder().method(method).uri(uri);
    for (key, value) in request.headers() {
        builder = builder.header(key.as_str(), value.as_bytes());
    }
    let request = builder.body(body)?;

    let response = client.send(request).await?;
    let status = StatusCode::from_u16(response.status().as_u16())?;
    let (parts, body) = response.into_parts();
    let headers = parts.headers;
    let body = BoxBody::new(body.collect_bytes().await?);

    let mut response = HttpResponse::build(status);
    // *response.extensions_mut() = parts.extensions;
    for (key, value) in &headers {
        response.append_header((key.as_str(), value.as_bytes()));
    }
    Ok(response.body(body))
}

#[derive(Clone)]
pub struct UserClient {
    pub kube: Client,
    pub name: String,
    pub is_admin: bool,
}

#[cfg(not(feature = "vine"))]
pub async fn load_client(request: &HttpRequest) -> Result<UserClient> {
    use cassette_plugin_jwt::get_authorization_token;
    use kube::{config::AuthInfo, Config};

    let token = get_authorization_token(request)?;

    let config = Config {
        auth_info: AuthInfo {
            token: Some(token.to_string().into()),
            ..Default::default()
        },
        #[cfg(feature = "vine")]
        default_namespace: {
            let client = Client::try_default().await?;
            ::vine_rbac::auth::get_user_namespace(&client, request).await?
        },
        ..Config::incluster()
            .map_err(|error| anyhow!("failed to create a kubernetes config: {error}"))?
    };
    let client = Client::try_from(config)
        .map_err(|error| anyhow!("failed to create a kubernetes client: {error}"))?;

    Ok(UserClient {
        kube: client,
        is_admin: false,
    })
}

#[cfg(feature = "vine")]
pub async fn load_client(request: &HttpRequest) -> Result<UserClient> {
    let client = Client::try_default().await?;
    ::vine_rbac::auth::get_user_client(&client, request)
        .await
        .map(
            |::vine_rbac::auth::UserClient { kube, name, role }| UserClient {
                kube,
                name,
                is_admin: role.is_admin,
            },
        )
        .map_err(Into::into)
}

fn response_error(code: StatusCode, error: impl ToString) -> HttpResponse {
    let reason: String = code.canonical_reason().unwrap_or("Unknown").into();
    let response = ErrorResponse {
        status: reason.clone(),
        message: error.to_string(),
        reason,
        code: code.into(),
    };
    HttpResponse::MethodNotAllowed().json(response)
}
