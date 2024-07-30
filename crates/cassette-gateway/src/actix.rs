use actix_cors::Cors;
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    get,
    http::header,
    middleware, web, App, HttpResponse, HttpServer, Responder, Scope,
};
use actix_web_opentelemetry::{RequestMetrics, RequestTracing};
use anyhow::{anyhow, Result};
use ark_core::signal::FunctionSignal;
use tracing::{error, info, instrument, Level};

use crate::agent::Agent;

#[instrument(level = Level::INFO)]
async fn home() -> impl Responder {
    HttpResponse::Ok().json("cassette-gateway")
}

#[instrument(level = Level::INFO)]
#[get("/_health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json("healthy")
}

#[instrument(level = Level::INFO)]
#[get("/robots.txt")]
async fn robots_txt() -> impl Responder {
    HttpResponse::Ok()
        .append_header(header::ContentType(::mime::TEXT_PLAIN_UTF_8))
        .message_body(include_str!("../robots.txt"))
}

pub async fn loop_forever(signal: FunctionSignal, agent: Agent) {
    match try_loop_forever(agent).await {
        Ok(()) => signal.terminate(),
        Err(error) => {
            error!("failed to operate http server: {error}");
            signal.terminate_on_panic();
        }
    }
}

async fn try_loop_forever(agent: Agent) -> Result<()> {
    info!("Starting http server...");

    // Initialize pipe
    let addr = agent.bind_addr();
    let base_url = agent.base_url();
    let redirect_error_404 = agent.redirect_error_404();

    let agent = web::Data::new(agent);
    #[cfg(feature = "kubernetes")]
    let kube = ::cassette_plugin_kubernetes_api::build_app_data().await?;

    // Create a http server
    let server = HttpServer::new(move || {
        let app = App::new().app_data(agent.clone());
        #[cfg(feature = "kubernetes")]
        let app = app.app_data(kube.clone());
        let app = build_default_service(app, redirect_error_404.clone());
        let app = build_services(app, base_url.as_deref());

        app.wrap(build_cors())
            .wrap(middleware::NormalizePath::new(
                middleware::TrailingSlash::Trim,
            ))
            .wrap(RequestTracing::default())
            .wrap(RequestMetrics::default())
    })
    .bind(addr)
    .map_err(|error| anyhow!("failed to bind to {addr}: {error}"))?;

    // Start http server
    server.run().await.map_err(Into::into)
}

fn build_cors() -> Cors {
    Cors::default()
        .allow_any_header()
        .allow_any_method()
        .allow_any_origin()
}

fn build_default_service<T>(app: App<T>, redirect_error_404: Option<String>) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = ::actix_web::Error, InitError = ()>,
{
    match redirect_error_404.clone() {
        Some(to) => app.default_service(web::to(move || {
            let to = to.clone();
            async { web::Redirect::to(to) }
        })),
        None => app,
    }
}

fn build_services<T>(app: App<T>, base_url: Option<&str>) -> App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = ::actix_web::Error, InitError = ()>,
{
    let scope = web::scope(base_url.unwrap_or(""));
    let scope = build_core_services(scope);
    let scope = build_plugin_services(scope);

    app.route(
        base_url.filter(|&path| !path.is_empty()).unwrap_or("/"),
        web::get().to(home),
    )
    .service(scope)
}

fn build_core_services(scope: Scope) -> Scope {
    scope
        .service(health)
        .service(robots_txt)
        .service(crate::routes::cassette::get)
        .service(crate::routes::cassette::list)
}

fn build_plugin_services(scope: Scope) -> Scope {
    #[cfg(feature = "cdl")]
    let scope = ::cassette_plugin_cdl_api::build_services(scope);
    #[cfg(feature = "helm")]
    let scope = ::cassette_plugin_helm_api::build_services(scope);
    #[cfg(feature = "kubernetes")]
    let scope = ::cassette_plugin_kubernetes_api::build_services(scope);

    scope
}
