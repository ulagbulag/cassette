#[cfg(feature = "vine")]
pub mod user;

use std::collections::BTreeMap;

use actix_web::{
    body::BoxBody,
    http::StatusCode,
    web::{Data, Path, Payload, Query},
    HttpRequest, HttpResponse, Responder, Scope,
};
use anyhow::{anyhow, Result};
use cassette_plugin_jwt::{get_authorization_token, parse_jwt};
use cassette_plugin_kubernetes_core::user::{UserMetadata, UserRoleSpec, UserSpec};
use http::Request;
use kube::{config::AuthInfo, core::ErrorResponse, Client, Config};
use url::Url;

pub async fn build_app_data() -> Result<Data<Client>> {
    Client::try_default()
        .await
        .map(Data::new)
        .map_err(Into::into)
}

pub fn build_services(scope: Scope) -> Scope {
    let scope = self::user::build_services(scope);
    scope.route("/kube/{path:.*}", ::actix_web::web::route().to(handle))
}

async fn handle(
    client: Data<Client>,
    request: HttpRequest,
    payload: Payload,
    uri: Path<String>,
    queries: Query<BTreeMap<String, String>>,
) -> impl Responder {
    match UserClient::from_request(client, &request).await {
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
    pub spec: UserSpec,
}

impl UserClient {
    pub async fn from_request(client: Data<Client>, request: &HttpRequest) -> Result<Self> {
        #[cfg(not(feature = "vine"))]
        {
            let _ = client;
            Self::from_request_with_oidc(request).await
        }
        #[cfg(feature = "vine")]
        {
            Self::from_request_with_vine(client, request).await
        }
    }

    pub async fn from_request_with_oidc(request: &HttpRequest) -> Result<Self> {
        let token = get_authorization_token(request)?;
        let metadata: UserMetadata = parse_jwt(token)?;

        let config = Config {
            auth_info: AuthInfo {
                token: Some(token.to_string().into()),
                ..Default::default()
            },
            ..Config::incluster()
                .map_err(|error| anyhow!("failed to create a kubernetes config: {error}"))?
        };
        let client = Client::try_from(config)
            .map_err(|error| anyhow!("failed to create a kubernetes client: {error}"))?;

        let name = metadata.preferred_username.clone();
        let namespace = client.default_namespace().into();

        Ok(Self {
            kube: client,
            spec: UserSpec {
                metadata,
                name,
                namespace,
                role: UserRoleSpec::default(),
            },
        })
    }

    #[cfg(feature = "vine")]
    pub async fn from_request_with_vine(
        client: Data<Client>,
        request: &HttpRequest,
    ) -> Result<Self> {
        use vine_api::user_session::UserSession;
        use vine_rbac::auth::AuthUserSession;

        let token = get_authorization_token(request)?;
        let metadata = parse_jwt(token)?;

        UserSession::from_request(&client, request)
            .await
            .map_err(Into::into)
            .and_then(|session| UserSpec::from_vine_session(metadata, session))
            .map(|(spec, kube)| Self { kube, spec })
    }
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
