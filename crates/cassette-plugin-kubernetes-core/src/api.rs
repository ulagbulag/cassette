use std::{marker::PhantomData, rc::Rc};

use anyhow::Result;
use kube_core::{params::ListParams, ObjectList, Request};
use serde::de::DeserializeOwned;

use crate::client::Client;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Api<K> {
    pub api_group: Option<String>,
    pub namespace: Option<String>,
    pub plural: String,
    pub version: String,
    pub _type: PhantomData<K>,
}

impl<K> Api<K> {
    pub(crate) async fn find(api_version: String, kind: String) -> Result<Self> {
        let (api_group, version) = match api_version.split_once('/') {
            Some(("", "")) => (None, None),
            Some((api_group, "")) => (Some(api_group), None),
            Some(("", version)) => (None, Some(version)),
            Some((api_group, version)) => (Some(api_group), Some(version)),
            None => {
                if api_version.is_empty() {
                    (None, None)
                } else {
                    (None, Some(api_version.as_str()))
                }
            }
        };

        // TODO: to be implemented (server-side)
        let _ = kind;

        Ok(Self {
            api_group: api_group.map(|s| s.into()),
            namespace: Some("default".into()),
            plural: "deployments".into(),
            version: version.unwrap_or("v1").into(),
            _type: PhantomData,
        })

        // let url_path = match api_group {
        //     Some(api_group) => format!("/apis/"),
        // };

        // // Discover most stable version variant of document
        // let apigroup = discovery::group(&kube, api_group).await?;
        // let (ar, caps) = match match version {
        //     Some(version) => apigroup.versioned_resources(version),
        //     None => apigroup.recommended_resources(),
        // }
        // .into_iter()
        // .find(|(ar, _)| ar.kind == kind)
        // {
        //     Some((ar, caps)) => (ar, caps),
        //     None => bail!(
        //         "Cannot find resource: {kind}.{api_group}/{version}",
        //         api_group = if api_group.is_empty() {
        //             "core"
        //         } else {
        //             api_group
        //         },
        //         version = version.unwrap_or("auto"),
        //     ),
        // };

        // // Use the discovered kind in an Api, and Controller with the ApiResource as its DynamicType
        // Ok(match caps.scope {
        //     Scope::Cluster => Api::all_with(kube, &ar),
        //     Scope::Namespaced => match namespace {
        //         Some(namespace) => Api::namespaced_with(kube, namespace, &ar),
        //         None => Api::default_namespaced_with(kube, &ar),
        //     },
        // });
    }

    pub(crate) async fn list(self: Rc<Self>, lp: ListParams) -> Result<ObjectList<K>>
    where
        K: Clone + DeserializeOwned,
    {
        let Self {
            api_group,
            namespace,
            plural,
            version,
            _type: PhantomData,
        } = &*self;

        let url_path = match (api_group, namespace) {
            (None, None) => format!("/api/{version}/{plural}"),
            (None, Some(namespace)) => format!("/api/{version}/namespaces/{namespace}/{plural}"),
            (Some(api_group), None) => {
                format!("/apis/{api_group}/{version}/{plural}")
            }
            (Some(api_group), Some(namespace)) => {
                format!("/apis/{api_group}/{version}/namespaces/{namespace}/{plural}")
            }
        };

        let request = Request::new(url_path).list(&lp)?;

        let client = Client::current()?;
        client.request("list", request).await
    }
}
