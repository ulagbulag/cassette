pub mod api;
pub mod client;
pub mod hooks;

// use anyhow::{anyhow, bail, Result};
// use inflector::Inflector;
// use kube::{
//     api::DynamicObject,
//     discovery::{self, ApiGroup, Scope},
//     Api, Client,
// };
// use tracing::{instrument, Level};

// pub async fn init_client() -> Result<Client> {
//     Client::try_default()
//         .await
//         .map_err(|error| anyhow!("failed to init kubernetes client: {error}"))
// }

// #[instrument(level = Level::INFO, skip(kube))]
// pub async fn get_api(
//     kube: Client,
//     namespace: Option<&str>,
//     api_group: Option<&str>,
//     version: Option<&str>,
//     kind: &str,
// ) -> Result<Api<DynamicObject>> {
//     let api_group = api_group.unwrap_or(ApiGroup::CORE_GROUP);
//     let kind_pascal = kind.to_pascal_case();

//     // Discover most stable version variant of document
//     let apigroup = discovery::group(&kube, api_group).await?;
//     let (ar, caps) = match match version {
//         Some(version) => apigroup.versioned_resources(version),
//         None => apigroup.recommended_resources(),
//     }
//     .into_iter()
//     .find(|(ar, _)| ar.kind == kind_pascal)
//     {
//         Some((ar, caps)) => (ar, caps),
//         None => bail!(
//             "Cannot find resource: {kind}.{api_group}/{version}",
//             api_group = if api_group.is_empty() {
//                 "core"
//             } else {
//                 api_group
//             },
//             version = version.unwrap_or("auto"),
//         ),
//     };

//     // Use the discovered kind in an Api, and Controller with the ApiResource as its DynamicType
//     Ok(match caps.scope {
//         Scope::Cluster => Api::all_with(kube, &ar),
//         Scope::Namespaced => match namespace {
//             Some(namespace) => Api::namespaced_with(kube, namespace, &ar),
//             None => Api::default_namespaced_with(kube, &ar),
//         },
//     })
// }
