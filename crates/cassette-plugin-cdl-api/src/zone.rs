use actix_web::{get, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use cassette_core::{
    data::{
        csv::CsvTable,
        table::{DataTable, DataTableLog, DataTableSource},
    },
    result::Result as HttpResult,
};
use cassette_plugin_kubernetes_api::{load_client, Client};
use serde::Serialize;

#[get("/cdl/zone")]
pub async fn list(request: HttpRequest) -> impl Responder {
    async fn try_handle(client: Client) -> Result<DataTable> {
        Ok(DataTable {
            name: "cdl-zones".into(),
            data: DataTableSource::Csv(CsvTable::default()),
            log: DataTableLog::default(),
        })
    }

    match load_client(&request).await {
        Ok(client) => response(try_handle(client).await),
        Err(error) => HttpResponse::Unauthorized().json(HttpResult::<()>::Err(error.to_string())),
    }
}

fn response<T>(result: Result<T>) -> HttpResponse
where
    T: Serialize,
{
    match result {
        Ok(data) => HttpResponse::Ok().json(HttpResult::Ok(data)),
        Err(error) => {
            HttpResponse::MethodNotAllowed().json(HttpResult::<T>::Err(error.to_string()))
        }
    }
}
