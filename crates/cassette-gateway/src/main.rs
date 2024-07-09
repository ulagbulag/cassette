mod actix;
mod agent;
mod db;
mod reloader;
mod routes;

use anyhow::anyhow;
use ark_core::signal::FunctionSignal;
use tokio::spawn;
use tracing::{error, info};

#[::tokio::main]
async fn main() {
    ::ark_core::tracer::init_once();
    info!("Welcome to cassette gateway!");

    let signal = FunctionSignal::default().trap_on_panic();
    if let Err(error) = signal.trap_on_sigint() {
        error!("{error}");
        return;
    }

    info!("Booting...");
    let agent = match self::agent::Agent::try_default().await {
        Ok(agent) => agent,
        Err(error) => {
            signal
                .panic(anyhow!("failed to init cassette gateway agent: {error}"))
                .await
        }
    };
    let reloader = match self::reloader::CassetteDBReloader::try_new(agent.db().clone()).await {
        Ok(agent) => agent,
        Err(error) => {
            signal
                .panic(anyhow!("failed to init cassette db reloader: {error}"))
                .await
        }
    };

    info!("Registering side workers...");
    let handlers = vec![
        spawn(crate::actix::loop_forever(signal.clone(), agent)),
        spawn(reloader.loop_forever(signal.clone())),
    ];

    info!("Ready");
    signal.wait_to_terminate().await;

    info!("Terminating...");
    for handler in handlers {
        handler.abort();
    }

    signal.exit().await
}
