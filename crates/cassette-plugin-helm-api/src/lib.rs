mod client;

use std::rc::Rc;

use actix_web::{
    get, post, put,
    web::{Data, Json, Path, Query},
    HttpRequest, HttpResponse, Responder, Scope,
};
use anyhow::Result;
use cassette_core::{
    data::{
        csv::CsvTable,
        table::{DataTable, DataTableLog, DataTableSource},
    },
    result::HttpResult,
};
use cassette_plugin_helm_core::{HelmDelete, HelmList, HelmPost, HelmPut};
use cassette_plugin_kubernetes_api::UserClient;
use cassette_plugin_kubernetes_core::user::{UserRoleSpec, UserSpec};
use itertools::Itertools;
use k8s_openapi::api::core::v1::Secret;
use kube::{api::ListParams, Api, Client, ResourceExt};
use serde_json::Value;
use tracing::{instrument, Level};
use uuid::Uuid;

pub fn build_services(scope: Scope) -> Scope {
    scope
        .service(actor)
        .service(delete)
        .service(list)
        .service(post)
        .service(put)
}

#[instrument(level = Level::INFO, skip_all)]
#[get("/helm/_actor")]
async fn actor() -> impl Responder {
    HttpResponse::from(HttpResult::Ok(::cassette_plugin_helm_core::actor()))
}

#[instrument(level = Level::INFO, skip_all)]
#[post("/helm/{id}/delete")]
async fn delete(
    client: Data<Client>,
    request: HttpRequest,
    id: Path<Uuid>,
    data: Json<HelmDelete>,
) -> impl Responder {
    match UserClient::from_request(client, &request).await {
        Ok(client) => HttpResponse::from(HttpResult::from(
            self::client::delete(client, id.into_inner(), data.0).await,
        )),
        Err(error) => HttpResponse::Unauthorized().json(HttpResult::<()>::Err(error.to_string())),
    }
}

#[instrument(level = Level::INFO, skip_all)]
#[get("/helm")]
async fn list(
    client: Data<Client>,
    request: HttpRequest,
    query: Query<HelmList>,
) -> impl Responder {
    async fn try_handle(client: UserClient, query: HelmList) -> Result<DataTable> {
        let UserClient {
            kube,
            spec:
                UserSpec {
                    name,
                    role: UserRoleSpec { is_admin },
                    ..
                },
        } = client;

        // Load data

        let HelmList { namespace } = query;

        let api = if is_admin {
            match namespace {
                Some(namespace) => Api::<Secret>::namespaced(kube, &namespace),
                None => Api::<Secret>::all(kube),
            }
        } else {
            Api::<Secret>::default_namespaced(kube)
        };
        let lp = ListParams {
            label_selector: Some("owner=helm".into()),
            ..Default::default()
        };
        let list = api.list_metadata(&lp).await?;

        // Create a data table

        let name = format!("helm-{name}");
        let headers = vec![
            "id".into(),
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
                    .get(self::labels::LABEL_STATE)
                    .map(|state| state != "superseded")
                    .unwrap_or_default()
            })
            .filter_map(|item| {
                let get_label = |name| item.labels().get(name).cloned().map(Value::String);

                Some(vec![
                    Value::String(item.uid()?),
                    Value::String(item.namespace()?),
                    get_label(self::labels::LABEL_NAME)?,
                    get_label(self::labels::LABEL_VERSION)?,
                    get_label(self::labels::LABEL_STATE)?,
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

    match UserClient::from_request(client, &request).await {
        Ok(client) => HttpResponse::from(HttpResult::from(try_handle(client, query.0).await)),
        Err(error) => HttpResponse::Unauthorized().json(HttpResult::<()>::Err(error.to_string())),
    }
}

#[instrument(level = Level::INFO, skip_all)]
#[post("/helm/{id}")]
async fn post(
    client: Data<Client>,
    request: HttpRequest,
    id: Path<Uuid>,
    data: Json<HelmPost>,
) -> impl Responder {
    match UserClient::from_request(client, &request).await {
        Ok(client) => HttpResponse::from(HttpResult::from(
            self::client::upgrade(client, id.into_inner(), data.0).await,
        )),
        Err(error) => HttpResponse::Unauthorized().json(HttpResult::<()>::Err(error.to_string())),
    }
}

#[instrument(level = Level::INFO, skip_all)]
#[put("/helm")]
async fn put(client: Data<Client>, request: HttpRequest, data: Json<HelmPut>) -> impl Responder {
    match UserClient::from_request(client, &request).await {
        Ok(client) => HttpResponse::from(HttpResult::from(
            self::client::install(client, data.0).await,
        )),
        Err(error) => HttpResponse::Unauthorized().json(HttpResult::<()>::Err(error.to_string())),
    }
}

mod labels {
    pub const LABEL_NAME: &str = "name";
    pub const LABEL_STATE: &str = "status";
    pub const LABEL_VERSION: &str = "version";
}
