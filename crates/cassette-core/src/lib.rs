pub mod cassette;
pub mod components;
pub mod data;
pub mod document;
pub mod net;
pub mod result;
pub mod task;

#[cfg(feature = "ui")]
pub mod prelude {
    pub use crate::components::{
        error::Error,
        loading::Loading,
        todo::{todo, Todo},
    };
}
