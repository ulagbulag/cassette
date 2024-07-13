use std::marker::PhantomData;

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
    pub(crate) async fn list(self, lp: ListParams) -> Result<ObjectList<K>>
    where
        K: Clone + DeserializeOwned,
    {
        let Self {
            api_group,
            namespace,
            plural,
            version,
            _type: PhantomData,
        } = self;

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
