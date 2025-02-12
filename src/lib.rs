#![warn(clippy::unwrap_used)]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#[macro_use]
extern crate uom;

mod access;
pub mod data;
pub mod error;
mod file;
pub mod getters;
pub mod line;
pub mod trajectory;
pub mod units;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const PACKAGE_VERSION: &str = env!("CARGO_PKG_VERSION");
