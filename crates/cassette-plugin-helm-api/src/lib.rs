use std::rc::Rc;

use actix_web::{get, web::Data, HttpRequest, HttpResponse, Responder, Scope};
use anyhow::Result;
use cassette_core::{
    data::{
        csv::CsvTable,
        table::{DataTable, DataTableLog, DataTableSource},
    },
    result::HttpResult,
};
use cassette_plugin_kubernetes_api::{load_client, UserClient};
use itertools::Itertools;
use k8s_openapi::api::core::v1::Secret;
use kube::{api::ListParams, Api, Client, ResourceExt};
use serde_json::Value;

pub fn build_services(scope: Scope) -> Scope {
    scope.service(list)
}

#[get("/helm")]
async fn list(client: Data<Client>, request: HttpRequest) -> impl Responder {
    async fn try_handle(client: UserClient) -> Result<DataTable> {
        let UserClient {
            is_admin,
            kube,
            user_name,
        } = client;

        // Load data

        let api = if is_admin {
            Api::<Secret>::all(kube)
        } else {
            Api::<Secret>::default_namespaced(kube)
        };
        let lp = ListParams {
            label_selector: Some("owner=helm".into()),
            ..Default::default()
        };
        let list = api.list_metadata(&lp).await?;

        // Create a data table

        const LABEL_NAME: &str = "name";
        const LABEL_STATE: &str = "status";
        const LABEL_VERSION: &str = "version";

        let name = format!("helm-{user_name}");
        let headers = vec![
            "namespace".into(),
            "name".into(),
            "version".into(),
            "state".into(),
            "created_at".into(),
        ];
        let mut records = list
            .items
            .into_iter()
            .filter(|item| {
                item.labels()
                    .get(LABEL_STATE)
                    .map(|state| state != "superseded")
                    .unwrap_or_default()
            })
            .filter_map(|item| {
                let get_label = |name| item.labels().get(name).cloned().map(Value::String);

                Some(vec![
                    Value::String(item.namespace()?),
                    get_label(LABEL_NAME)?,
                    get_label(LABEL_VERSION)?,
                    get_label(LABEL_STATE)?,
                    Value::String(item.creation_timestamp()?.0.to_rfc3339()),
                ])
            })
            // NOTE: kubernetes list_metadata outputs should be ordered
            .rev()
            .unique_by(|item| (item[0].clone(), item[1].clone()))
            .collect_vec();
        records.reverse();

        Ok(DataTable {
            name,
            data: Rc::new(DataTableSource::Csv(CsvTable {
                headers,
                records: Rc::new(records),
            })),
            log: DataTableLog::default(),
        })
    }

    match load_client(client, &request).await {
        Ok(client) => HttpResponse::from(HttpResult::from(try_handle(client).await)),
        Err(error) => HttpResponse::Unauthorized().json(HttpResult::<()>::Err(error.to_string())),
    }
}
