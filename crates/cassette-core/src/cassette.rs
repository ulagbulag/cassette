#[cfg(feature = "ui")]
use std::{any::Any, collections::BTreeMap, ops, rc::Rc};
use std::{
    borrow::Borrow,
    cmp,
    hash::{Hash, Hasher},
};

use garde::Validate;
use kube::CustomResource;
use schemars::JsonSchema;
#[cfg(feature = "ui")]
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[cfg(feature = "ui")]
use yew::prelude::*;

use crate::component::CassetteComponentSpec;
#[cfg(feature = "ui")]
use crate::net::fetch::{FetchState, FetchStateHandle};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema, Validate, CustomResource)]
#[kube(
    group = "cassette.ulagbulag.io",
    version = "v1alpha1",
    kind = "Cassette",
    root = "CassetteCrd",
    shortname = "cas",
    namespaced,
    printcolumn = r#"{
        "name": "created-at",
        "type": "date",
        "description": "created time",
        "jsonPath": ".metadata.creationTimestamp"
    }"#,
    printcolumn = r#"{
        "name": "version",
        "type": "integer",
        "description": "cassette version",
        "jsonPath": ".metadata.generation"
    }"#
)]
#[serde(rename_all = "camelCase")]
pub struct CassetteSpec {
    #[garde(length(min = 1, max = 253), pattern("^[a-z][a-z0-9-]*[a-z0-9]*$"))]
    #[serde(default)]
    pub component: String,
    #[garde(length(min = 1, max = 1024))]
    #[serde(default)]
    pub description: Option<String>,
    #[garde(length(min = 1, max = 1024))]
    #[serde(default)]
    pub group: Option<String>,
    #[garde(skip)]
    #[serde(default)]
    pub priority: Option<u32>,
}

pub type CassetteRef = Cassette<Uuid>;

#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Cassette<Component = CassetteComponentSpec> {
    pub id: Uuid,
    pub component: Component,
    pub name: String,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub priority: Option<u32>,
}

impl<Component> PartialEq for Cassette<Component>
where
    Component: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl<Component> Eq for Cassette<Component> where Component: Eq {}

impl<Component> PartialOrd for Cassette<Component>
where
    Component: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl<Component> Ord for Cassette<Component>
where
    Component: Ord,
{
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<Component> Hash for Cassette<Component>
where
    Component: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl<Component> Borrow<Uuid> for Cassette<Component> {
    fn borrow(&self) -> &Uuid {
        &self.id
    }
}

#[cfg(feature = "ui")]
#[derive(Clone, Debug, PartialEq)]
pub struct CassetteState {
    cassette: Cassette,
    // FIXME: cache unchanged task outputs
    root: UseStateHandle<RootCassetteState>,
}

#[cfg(feature = "ui")]
impl CassetteState {
    pub const fn new(cassette: Cassette, root: UseStateHandle<RootCassetteState>) -> Self {
        Self { cassette, root }
    }

    fn set(&mut self, name: &str, value: crate::task::TaskSpec) {
        self.root.set({
            let mut root = (&*self.root).clone();
            root.spec.set_child(name, value);
            root
        })
    }

    fn use_handler<T>(
        &self,
        task_name: String,
        handler_name: String,
        f_init: impl FnOnce() -> T,
    ) -> Rc<T>
    where
        T: Any,
    {
        let key = (task_name, handler_name);
        let handler = match self.root.handlers.get(&key) {
            Some(handler) => handler.clone(),
            None => {
                let mut root = (&*self.root).clone();
                let handler = Rc::new(f_init());
                root.handlers.insert(key, handler.clone());
                self.root.set(root);
                handler
            }
        };

        match handler.downcast() {
            Ok(handler) => handler,
            Err(_) => panic!("Cannot create a handler with heterogeneous types"),
        }
    }
}

#[cfg(feature = "ui")]
#[derive(Clone, Debug, Default)]
pub struct RootCassetteState {
    handlers: BTreeMap<(String, String), Rc<dyn Any>>,
    spec: crate::task::TaskSpec,
}

#[cfg(feature = "ui")]
impl PartialEq for RootCassetteState {
    fn eq(&self, other: &Self) -> bool {
        self.handlers.len() == other.handlers.len()
            && self.handlers.iter().zip(&other.handlers).all(
                |((key_a, value_a), (key_b, value_b))| {
                    // the handler value is **immutable**, so comparing pointers is valid to check changes
                    key_a == key_b && Rc::ptr_eq(value_a, value_b)
                },
            )
            && self.spec == other.spec
    }
}

#[cfg(feature = "ui")]
#[derive(Debug, PartialEq)]
pub struct CassetteContext<'a> {
    state: &'a mut CassetteState,
    task: &'a crate::task::CassetteTask,
}

#[cfg(feature = "ui")]
impl<'a> CassetteContext<'a> {
    pub fn new(state: &'a mut CassetteState, task: &'a crate::task::CassetteTask) -> Self {
        Self { state, task }
    }

    pub(crate) fn get(&self, key: &str) -> Result<&::serde_json::Value, String> {
        self.state.root.spec.get(key)
    }

    pub(crate) fn get_task_state<T>(&self) -> Result<Option<T>, String>
    where
        T: DeserializeOwned,
    {
        self.state
            .root
            .spec
            .try_get(&self.task.name)
            .map(|value| {
                ::serde_json::from_value(value.clone())
                    .map_err(|error| format!("Failed to decode task state: {error}"))
            })
            .transpose()
    }

    pub(crate) fn set(self, state: crate::task::TaskState) -> crate::task::TaskState<()> {
        match state {
            crate::task::TaskState::Break { body, state } => crate::task::TaskState::Break {
                body,
                state: match state {
                    Some(state) => self.set_task_state(state),
                    None => (),
                },
            },
            crate::task::TaskState::Continue { body } => crate::task::TaskState::Continue { body },
            crate::task::TaskState::Skip { state } => crate::task::TaskState::Skip {
                state: match state {
                    Some(state) => self.set_task_state(state),
                    None => (),
                },
            },
        }
    }

    pub fn set_task_state(self, value: crate::task::TaskSpec) {
        self.state.set(&self.task.name, value)
    }

    pub fn use_state<T>(&self, id: String, f_init: impl FnOnce() -> T) -> CassetteTaskHandle<T>
    where
        T: Any,
    {
        let task_name = self.task.name.clone();
        CassetteTaskHandle {
            root: self.state.root.clone(),
            id: (task_name.clone(), id.clone()),
            item: self.state.use_handler(task_name, id, f_init),
        }
    }
}

#[cfg(feature = "ui")]
#[derive(Debug, PartialEq)]
pub struct CassetteTaskHandle<T> {
    root: UseStateHandle<RootCassetteState>,
    id: (String, String),
    item: Rc<T>,
}

#[cfg(feature = "ui")]
impl<T> ops::Deref for CassetteTaskHandle<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &*self.item
    }
}

#[cfg(feature = "ui")]
impl<T> Clone for CassetteTaskHandle<T> {
    fn clone(&self) -> Self {
        Self {
            root: self.root.clone(),
            id: self.id.clone(),
            item: self.item.clone(),
        }
    }
}

#[cfg(feature = "ui")]
impl<T> FetchStateHandle<T> for CassetteTaskHandle<FetchState<T>> {
    fn get(&self) -> &FetchState<T> {
        &*self.item
    }

    fn set(&mut self, value: FetchState<T>)
    where
        T: 'static,
    {
        self.item = Rc::new(value);
        self.root.set({
            let mut root = (&*self.root).clone();
            root.handlers.insert(self.id.clone(), self.item.clone());
            root
        })
    }
}
