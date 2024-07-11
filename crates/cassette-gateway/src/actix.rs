use actix_cors::Cors;
use actix_web::{
    get, middleware,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
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

    let agent = Data::new(agent);

    // Create a http server
    let server = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();

        let app = App::new().app_data(Data::clone(&agent));
        let mut app = match &base_url {
            Some(base_url) => app.route(base_url, web::to(home)).service(
                web::scope(base_url)
                    .service(health)
                    .service(crate::routes::cassette::get)
                    .service(crate::routes::cassette::list),
            ),
            None => app
                .service(health)
                .service(health)
                .service(crate::routes::cassette::get)
                .service(crate::routes::cassette::list),
        };
        if let Some(redirect_to) = redirect_error_404.clone() {
            app = app.default_service(web::to(move || {
                let redirect_to = redirect_to.clone();
                async { web::Redirect::to(redirect_to) }
            }));
        }
        app.wrap(cors)
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
