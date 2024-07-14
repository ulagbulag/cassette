use std::collections::BTreeMap;

use actix_web::{
    body::BoxBody,
    http::{Method, StatusCode},
    web::{Path, Payload, Query},
    HttpResponse, Responder,
};
use anyhow::{anyhow, Result};
use http::Request;
use kube::{core::ErrorResponse, Client};
use url::Url;

pub async fn handle(
    method: Method,
    payload: Payload,
    uri: Path<String>,
    queries: Query<BTreeMap<String, String>>,
) -> impl Responder {
    try_handle(method, uri.into_inner(), queries.0, payload)
        .await
        .unwrap_or_else(|error| {
            let response = ErrorResponse {
                status: "MethodNotAllowed".into(),
                message: error.to_string(),
                reason: "MethodNotAllowed".into(),
                code: 405,
            };
            HttpResponse::MethodNotAllowed().json(response)
        })
}

async fn try_handle(
    method: Method,
    uri: String,
    queries: BTreeMap<String, String>,
    payload: Payload,
) -> Result<HttpResponse> {
    let method = method.as_str();
    let body = payload
        .to_bytes()
        .await
        .map_err(|error| anyhow!("{error}"))?
        .into();

    let uri = {
        let mut url: Url = format!("/{uri}").parse()?;
        {
            let mut pairs = url.query_pairs_mut();
            for (key, value) in &queries {
                pairs.append_pair(key, value);
            }
        }
        url.to_string()
    };

    let request = Request::builder().method(method).uri(uri).body(body)?;

    let client = Client::try_default().await?;
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
