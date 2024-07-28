#[cfg(feature = "ui")]
use std::{any::Any, cell::RefCell, collections::BTreeMap, ops, rc::Rc};
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
#[cfg(feature = "ui")]
use tracing::info;
use uuid::Uuid;
#[cfg(feature = "ui")]
use yew::{html::IntoPropValue, prelude::*};

use crate::components::CassetteComponentSpec;

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
#[derive(Debug)]
pub struct CassetteState {
    root: RootCassetteState,
}

#[cfg(feature = "ui")]
impl CassetteState {
    pub fn new(trigger: UseForceUpdateHandle) -> Self {
        Self {
            root: RootCassetteState::new(trigger),
        }
    }

    fn set(&mut self, name: &str, value: crate::task::TaskSpec) {
        self.root.set_child(name, value)
    }
}

#[cfg(feature = "ui")]
#[derive(Clone, Debug)]
pub struct RootCassetteState {
    trigger: UseForceUpdateHandle,
}

#[cfg(feature = "ui")]
impl Reducible for RootCassetteState {
    type Action = ();

    fn reduce(self: Rc<Self>, (): Self::Action) -> Rc<Self> {
        self
    }
}

#[cfg(feature = "ui")]
impl RootCassetteState {
    #[cfg(feature = "ui")]
    thread_local! {
        static HANDLERS: RefCell<BTreeMap<(String, String), Rc<dyn Any>>> = Default::default();
        static SPEC: RefCell<crate::task::TaskSpec> = Default::default();
    }

    const fn new(trigger: UseForceUpdateHandle) -> Self {
        Self { trigger }
    }

    fn update(&self, trigger: bool) {
        if trigger {
            self.trigger.force_update()
        }
    }

    fn get_child<T>(&self, name: &str) -> Result<Option<T>, String>
    where
        T: DeserializeOwned,
    {
        Self::SPEC.with_borrow(|spec| {
            spec.get_child(name)
                .map(|value| {
                    ::serde_json::from_value(value.clone())
                        .map_err(|error| format!("Failed to decode task state: {error}"))
                })
                .transpose()
        })
    }

    fn get_data(&self, key: &str) -> Result<::serde_json::Value, String> {
        Self::SPEC.with_borrow(|spec| spec.get(key).cloned())
    }

    fn set_child(&self, name: &str, value: crate::task::TaskSpec) {
        Self::SPEC.with_borrow_mut(|spec| {
            if spec.set_child(name, value) {
                info!("Detected child update: {name}");
                self.update(false);
            }
        })
    }

    fn set_handler<T>(&self, id: (String, String), value: T, trigger: bool)
    where
        T: 'static,
    {
        Self::HANDLERS.with_borrow_mut(|handlers| {
            info!("Detected handler::update: {id:?}");
            self.update(trigger);
            handlers.insert(id, Rc::new(value));
        })
    }

    fn use_handler<T>(
        &self,
        force_init: bool,
        id: (String, String),
        f_init: impl FnOnce() -> T,
    ) -> Rc<T>
    where
        T: 'static,
    {
        let (task_name, handler_name) = &id;

        let init_handler = |handlers: &mut BTreeMap<_, Rc<dyn Any>>| {
            let handler = Rc::new(f_init());
            {
                info!("Detected handler::create: {id:?}");
                self.update(false);
                handlers.insert(id.clone(), handler.clone());
            }
            handler
        };

        let handler = Self::HANDLERS.with_borrow_mut(|handlers| {
            if force_init {
                init_handler(handlers)
            } else {
                match handlers.get(&id) {
                    Some(handler) => handler.clone(),
                    None => init_handler(handlers),
                }
            }
        });

        match handler.downcast() {
            Ok(handler) => handler,
            Err(_) => panic!(
                "Cannot get a handler with heterogeneous types: {task_name:?}/{handler_name:?}"
            ),
        }
    }
}

#[cfg(feature = "ui")]
#[derive(Debug)]
pub struct CassetteContext<'a> {
    state: &'a mut CassetteState,
    task: &'a crate::task::CassetteTask,
}

#[cfg(feature = "ui")]
impl<'a> CassetteContext<'a> {
    pub fn new(state: &'a mut CassetteState, task: &'a crate::task::CassetteTask) -> Self {
        Self { state, task }
    }

    pub(crate) fn get_child<T>(&self) -> Result<Option<T>, String>
    where
        T: DeserializeOwned,
    {
        self.state.root.get_child(&self.task.name)
    }

    pub(crate) fn get_data(&self, key: &str) -> Result<::serde_json::Value, String> {
        self.state.root.get_data(key)
    }

    pub(crate) fn set(self, state: crate::task::TaskState) -> crate::task::TaskState<()> {
        match state {
            crate::task::TaskState::Break { body, state } => crate::task::TaskState::Break {
                body,
                state: if let Some(state) = state {
                    self.set_task_state(state)
                },
            },
            crate::task::TaskState::Continue { body, state } => crate::task::TaskState::Continue {
                body,
                state: if let Some(state) = state {
                    self.set_task_state(state)
                },
            },
            crate::task::TaskState::Skip { state } => crate::task::TaskState::Skip {
                state: if let Some(state) = state {
                    self.set_task_state(state)
                },
            },
        }
    }

    pub fn set_task_state(self, value: crate::task::TaskSpec) {
        self.state.set(&self.task.name, value)
    }

    pub fn use_state<T>(
        &self,
        id: impl Into<String>,
        force_init: bool,
        f_init: impl FnOnce() -> T,
    ) -> CassetteTaskHandle<T>
    where
        T: Any,
    {
        let task_name = self.task.name.clone();
        let handler_name = id.into();
        let id = (task_name, handler_name);
        CassetteTaskHandle {
            root: self.state.root.clone(),
            id: id.clone(),
            item: RootCassetteState::use_handler(&self.state.root, force_init, id.clone(), f_init),
        }
    }
}

#[cfg(feature = "ui")]
pub trait GenericCassetteTaskHandle<T>
where
    Self: Clone,
{
    type Ref<'a>
    where
        Self: 'a;

    fn get<'a>(&'a self) -> <Self as GenericCassetteTaskHandle<T>>::Ref<'a>
    where
        <Self as GenericCassetteTaskHandle<T>>::Ref<'a>: ops::Deref<Target = T>;

    fn set(&self, value: T)
    where
        T: 'static;
}

#[cfg(feature = "ui")]
impl<T> GenericCassetteTaskHandle<T> for UseStateHandle<T> {
    type Ref<'a> = &'a T where T: 'a;

    fn get<'a>(&'a self) -> <Self as GenericCassetteTaskHandle<T>>::Ref<'a>
    where
        <Self as GenericCassetteTaskHandle<T>>::Ref<'a>: ops::Deref<Target = T>,
    {
        self
    }

    fn set(&self, value: T) {
        (*self).set(value)
    }
}

#[cfg(feature = "ui")]
#[derive(Debug)]
pub struct CassetteTaskHandle<T> {
    root: RootCassetteState,
    id: (String, String),
    item: Rc<T>,
}

#[cfg(feature = "ui")]
impl<T> ops::Deref for CassetteTaskHandle<T> {
    type Target = Rc<T>;

    fn deref(&self) -> &Self::Target {
        &self.item
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
impl<T> PartialEq for CassetteTaskHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && Rc::ptr_eq(&self.item, &other.item)
    }
}

#[cfg(feature = "ui")]
impl<T> Eq for CassetteTaskHandle<T> {}

#[cfg(feature = "ui")]
impl<T> IntoPropValue<T> for CassetteTaskHandle<T>
where
    T: Clone,
{
    fn into_prop_value(self) -> T {
        self.get().clone()
    }
}

#[cfg(feature = "ui")]
impl<T> GenericCassetteTaskHandle<T> for CassetteTaskHandle<T> {
    type Ref<'a> = &'a T where T: 'a;

    fn get<'a>(&'a self) -> <Self as GenericCassetteTaskHandle<T>>::Ref<'a>
    where
        <Self as GenericCassetteTaskHandle<T>>::Ref<'a>: ops::Deref<Target = T>,
    {
        &self.item
    }

    fn set(&self, value: T)
    where
        T: 'static,
    {
        RootCassetteState::set_handler(&self.root, self.id.clone(), value, true)
    }
}

#[cfg(feature = "ui")]
impl<T> CassetteTaskHandle<T> {
    pub fn lazy(self) -> CassetteLazyHandle<T> {
        CassetteLazyHandle(self)
    }
}

#[cfg(feature = "ui")]
impl<T> CassetteTaskHandle<Vec<T>> {
    pub fn get_item(&self, index: usize) -> Option<&T> {
        self.item.get(index)
    }

    pub fn set_all(&self, value: T)
    where
        T: 'static + Copy,
    {
        let mut values = (*self.item).clone();
        values.fill(value);
        self.set(values)
    }

    pub fn set_item(&self, index: usize, value: T)
    where
        T: 'static + Clone,
    {
        let mut values = (*self.item).clone();
        if let Some(place) = values.get_mut(index) {
            *place = value;
            self.set(values)
        }
    }
}
#[cfg(feature = "ui")]
#[derive(Debug)]
pub struct CassetteLazyHandle<T>(CassetteTaskHandle<T>);

#[cfg(feature = "ui")]
impl<T> Clone for CassetteLazyHandle<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[cfg(feature = "ui")]
impl<T> IntoPropValue<T> for CassetteLazyHandle<T>
where
    T: Clone,
{
    fn into_prop_value(self) -> T {
        self.get().clone()
    }
}

#[cfg(feature = "ui")]
impl<T> GenericCassetteTaskHandle<T> for CassetteLazyHandle<T> {
    type Ref<'a> = &'a T where T: 'a;

    fn get<'a>(&'a self) -> <Self as GenericCassetteTaskHandle<T>>::Ref<'a>
    where
        <Self as GenericCassetteTaskHandle<T>>::Ref<'a>: ops::Deref<Target = T>,
    {
        self.0.get()
    }

    fn set(&self, value: T)
    where
        T: 'static,
    {
        RootCassetteState::set_handler(&self.0.root, self.0.id.clone(), value, false)
    }
}

#[cfg(feature = "ui")]
impl<T> CassetteLazyHandle<T> {
    pub fn trigger(&self) {
        self.0.root.update(true)
    }
}
