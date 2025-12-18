#![warn(clippy::unwrap_used)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

mod access;
pub mod data;
pub mod error;
mod file;
pub mod getters;
pub mod line;
pub mod trajectory;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
