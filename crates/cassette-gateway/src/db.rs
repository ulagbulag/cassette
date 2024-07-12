use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use cassette_core::{
    cassette::{Cassette, CassetteCrd, CassetteRef},
    component::CassetteComponentCrd,
};
use kube::ResourceExt;
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

#[derive(Default)]
struct CassetteDBInner {
    cassettes: BTreeMap<String, BTreeSet<Cassette<String>>>,
    components: BTreeMap<Uuid, CassetteComponentCrd>,
    components_scopes: BTreeMap<Scope, Uuid>,
}

impl CassetteDBInner {
    fn get(&self, namespace: &str, id: Uuid) -> Option<Cassette> {
        let Cassette {
            id,
            component: component_name,
            name,
            group,
            description,
            priority,
        } = self
            .cassettes
            .get(namespace)
            .and_then(|cassettes| cassettes.iter().find(|cassette| cassette.id == id).cloned())?;

        let scope = Scope {
            namespace: namespace.into(),
            name: component_name,
        };
        let component_id = self.components_scopes.get(&scope)?;
        let component = self
            .components
            .get(component_id)
            .map(|cr| cr.spec.clone())?;

        Some(Cassette {
            id,
            component,
            name,
            group,
            description,
            priority,
        })
    }

    fn list(&self, namespace: &str) -> Vec<CassetteRef> {
        self.cassettes
            .get(namespace)
            .map(|cassettes| {
                cassettes
                    .iter()
                    .cloned()
                    .filter_map(|cassette| self.find_component(namespace, cassette))
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl CassetteDBInner {
    fn insert(&mut self, cr: CassetteCrd) {
        let id = match cr.uid().and_then(|uid| uid.parse().ok()) {
            Some(uid) => uid,
            None => return,
        };
        let namespace = cr.namespace().unwrap_or_else(|| "default".into());
        let name = cr.name_any();

        let cassette = Cassette {
            id,
            component: cr.spec.component,
            name,
            group: cr.spec.group,
            description: cr.spec.description,
            priority: cr.spec.priority,
        };
        self.cassettes
            .entry(namespace)
            .or_default()
            .insert(cassette);
    }

    fn remove(&mut self, cr: CassetteCrd) {
        let id: Uuid = match cr.uid().and_then(|uid| uid.parse().ok()) {
            Some(uid) => uid,
            None => return,
        };
        let namespace = cr.namespace().unwrap_or_else(|| "default".into());

        if let Some(cassettes) = self.cassettes.get_mut(&namespace) {
            cassettes.remove(&id);
            if cassettes.is_empty() {
                self.cassettes.remove(&namespace);
            }
        }
    }
}

impl CassetteDBInner {
    fn find_component(&self, namespace: &str, cassette: Cassette<String>) -> Option<CassetteRef> {
        let Cassette {
            id,
            component,
            name,
            group,
            description,
            priority,
        } = cassette;

        let scope = Scope {
            namespace: namespace.into(),
            name: component,
        };
        let component = self.components_scopes.get(&scope).copied()?;

        Some(Cassette {
            id,
            component,
            name,
            group,
            description,
            priority,
        })
    }

    fn insert_component(&mut self, cr: CassetteComponentCrd) {
        let id = match cr.uid().and_then(|uid| uid.parse().ok()) {
            Some(uid) => uid,
            None => return,
        };
        let namespace = cr.namespace().unwrap_or_else(|| "default".into());
        let name = cr.name_any();

        let scope = Scope { namespace, name };

        self.components.insert(id, cr);
        self.components_scopes.insert(scope, id);
    }

    fn remove_component(&mut self, cr: CassetteComponentCrd) {
        let id: Uuid = match cr.uid().and_then(|uid| uid.parse().ok()) {
            Some(uid) => uid,
            None => return,
        };
        let namespace = cr.namespace().unwrap_or_else(|| "default".into());
        let name = cr.name_any();

        let scope = Scope { namespace, name };

        self.components.remove(&id);
        self.components_scopes.remove(&scope);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Scope {
    namespace: String,
    name: String,
}
