use std::rc::Rc;

use actix_web::{get, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use byte_unit::{Byte, UnitType};
use cassette_core::{
    data::{
        csv::CsvTable,
        table::{DataTable, DataTableLog, DataTableSource},
    },
    result::HttpResult,
};
use cassette_plugin_kubernetes_api::{load_client, UserClient};
use dash_api::storage::ModelStorageCrd;
use kube::{api::ListParams, Api, ResourceExt};
use serde_json::Value;

#[get("/cdl/zone")]
pub async fn list(request: HttpRequest) -> impl Responder {
    async fn try_handle(client: UserClient) -> Result<DataTable> {
        let api = Api::<ModelStorageCrd>::default_namespaced(client.kube);
        let lp = ListParams::default();
        let list = api.list(&lp).await?;

        Ok(DataTable {
            name: "cdl-zones".into(),
            data: Rc::new(DataTableSource::Csv(CsvTable {
                headers: vec![
                    "name".into(),
                    "state".into(),
                    "created_at".into(),
                    "total_quota".into(),
                ],
                records: Rc::new(
                    list.items
                        .into_iter()
                        .map(|item| {
                            vec![
                                Value::String(item.name_any()),
                                Value::String(
                                    item.status
                                        .as_ref()
                                        .map(|status| status.state)
                                        .unwrap_or_default()
                                        .to_string(),
                                ),
                                item.creation_timestamp()
                                    .map(|timestamp| Value::String(timestamp.0.to_string()))
                                    .unwrap_or_default(),
                                item.status
                                    .as_ref()
                                    .and_then(|status| status.total_quota)
                                    .and_then(Byte::from_u128)
                                    .map(|byte| {
                                        Value::String(
                                            byte.get_appropriate_unit(UnitType::Binary).to_string(),
                                        )
                                    })
                                    .unwrap_or_default(),
                            ]
                        })
                        .collect(),
                ),
            })),
            log: DataTableLog::default(),
        })
    }

    match load_client(&request).await {
        Ok(client) => HttpResponse::from(HttpResult::from(try_handle(client).await)),
        Err(error) => HttpResponse::Unauthorized().json(HttpResult::<()>::Err(error.to_string())),
    }
}
