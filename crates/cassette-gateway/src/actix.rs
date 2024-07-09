use actix_web::{get, middleware, web::Data, App, HttpResponse, HttpServer, Responder};
use actix_web_opentelemetry::{RequestMetrics, RequestTracing};
use anyhow::{anyhow, Result};
use ark_core::signal::FunctionSignal;
use tracing::{error, info, instrument, Level};

use crate::agent::Agent;

#[instrument(level = Level::INFO)]
#[get("/")]
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

    let agent = Data::new(agent);

    // Create a http server
    let server = HttpServer::new(move || {
        let app = App::new().app_data(Data::clone(&agent));
        let app = app
            .service(home)
            .service(health)
            .service(crate::routes::cassette::get)
            .service(crate::routes::cassette::list);
        app.wrap(middleware::NormalizePath::new(
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
