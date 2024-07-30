use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder, Scope};
use cassette_core::result::HttpResult;
use kube::Client;

use crate::UserClient;

pub(crate) fn build_services(scope: Scope) -> Scope {
    scope.service(get_me)
}

#[get("/user/me")]
async fn get_me(client: Data<Client>, request: HttpRequest) -> impl Responder {
    match UserClient::from_request(client, &request).await {
        Ok(client) => HttpResponse::from(HttpResult::Ok(client.spec)),
        Err(error) => HttpResponse::Unauthorized().json(HttpResult::<()>::Err(error.to_string())),
    }
}
