#![warn(clippy::unwrap_used)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub(crate) mod access;
pub mod data;
pub mod error;
pub(crate) mod file;
pub mod getters;
pub mod line;
pub mod trajectory;

pub(crate) const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
pub(crate) const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
