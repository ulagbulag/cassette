use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};
use ark_core::result::Result;
use tracing::{instrument, Level};
use uuid::Uuid;

use crate::agent::Agent;

#[instrument(level = Level::INFO, skip(agent))]
#[get("/c/{namespace}/{id}")]
pub async fn get(agent: Data<Agent>, path: Path<(String, Uuid)>) -> impl Responder {
    let (namespace, id) = path.into_inner();

    HttpResponse::Ok().json(Result::Ok(agent.get(&namespace, id).await))
}

#[instrument(level = Level::INFO, skip(agent))]
#[get("/c/{namespace}")]
pub async fn list(agent: Data<Agent>, path: Path<String>) -> impl Responder {
    let namespace = path.into_inner();

    HttpResponse::Ok().json(Result::Ok(agent.list(&namespace).await))
}
