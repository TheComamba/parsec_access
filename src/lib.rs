#![warn(clippy::unwrap_used)]
#![warn(missing_docs)]

pub mod access;
pub mod api;
pub mod error;
pub(crate) mod file;
pub(crate) mod getters;
pub(crate) mod line;
pub(crate) mod trajectory;

pub(crate) const PACKAGE_NAME: &'static str = env!("CARGO_PKG_NAME");
pub(crate) const PACKAGE_VERSION: &'static str = env!("CARGO_PKG_VERSION");
