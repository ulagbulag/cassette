use std::sync::Arc;

use cassette_core::{
    cassette::{Cassette, CassetteCrd, CassetteRef},
    component::CassetteComponentCrd,
};
use cassette_loader_core::CassetteDB as CassetteDBInner;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone, Default)]
pub(crate) struct CassetteDB(Arc<RwLock<CassetteDBInner>>);

impl CassetteDB {
    pub(crate) async fn get(&self, namespace: &str, id: Uuid) -> Option<Cassette> {
        self.0.read().await.get(namespace, id)
    }

    pub(crate) async fn list(&self, namespace: &str) -> Vec<CassetteRef> {
        self.0.read().await.list(namespace)
    }
}

impl CassetteDB {
    pub(crate) async fn insert(&self, cr: CassetteCrd) {
        self.0.write().await.insert(cr)
    }

    pub(crate) async fn remove(&self, cr: CassetteCrd) {
        self.0.write().await.remove(cr)
    }
}

impl CassetteDB {
    pub(crate) async fn insert_component(&self, cr: CassetteComponentCrd) {
        self.0.write().await.insert_component(cr)
    }

    pub(crate) async fn remove_component(&self, cr: CassetteComponentCrd) {
        self.0.write().await.remove_component(cr)
    }
}
