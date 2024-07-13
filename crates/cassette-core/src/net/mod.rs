#[cfg(feature = "ui")]
pub mod fetch;
#[cfg(feature = "ui")]
pub mod gateway;

#[cfg(feature = "examples")]
pub const DEFAULT_NAMESPACE: &str = "examples";
#[cfg(not(feature = "examples"))]
pub const DEFAULT_NAMESPACE: &str = "default";
