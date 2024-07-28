use std::{cell::RefCell, collections::VecDeque};

use gloo_storage::{LocalStorage, Storage as _};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web_sys::Storage;

pub struct History {
    storage: Storage,
    log: VecDeque<HistoryLog>,
}

impl History {
    const KEY: &'static str = "history";
    const MAX_LEN: usize = 100;

    thread_local! {
        static GLOBAL: RefCell<Option<History>> = RefCell::default();
    }

    fn with_borrow<R>(f: impl FnOnce(&mut Self) -> R) -> R {
        Self::GLOBAL.with_borrow_mut(|option| {
            f(option.get_or_insert_with(|| {
                let storage = LocalStorage::raw();
                let log = storage
                    .get_item(Self::KEY)
                    .ok()
                    .flatten()
                    .and_then(|value| ::serde_json::from_str(&value).ok())
                    .unwrap_or_default();
                Self { storage, log }
            }))
        })
    }

    pub fn get() -> Vec<HistoryLog> {
        Self::with_borrow(|history| history.log.iter().cloned().collect())
    }

    pub fn push(log: HistoryLog) {
        Self::with_borrow(|history| {
            if history.log.back().map(|last| *last != log).unwrap_or(true) {
                if history.log.len() >= Self::MAX_LEN {
                    history.log.pop_front();
                }
                history.log.push_back(log);

                if let Ok(value) = ::serde_json::to_string(&history.log) {
                    history.storage.set_item(Self::KEY, &value).ok();
                }
            }
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct HistoryLog {
    pub id: Uuid,
    pub name: String,
}
