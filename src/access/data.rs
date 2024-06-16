// This code is generated by generate_code.py, do not modify it manually.

//! Provides access to the data files for the PARSEC stellar evolution models.

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use crate::{error::ParsecAccessError, trajectory::Trajectory};

use super::metallicity::Metallicity;

lazy_static! {
    static ref Z0_0001_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0002_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0005_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0010_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0020_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0040_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0060_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0080_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0100_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0140_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0170_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0200_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0300_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0400_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
    static ref Z0_0600_DATA: Mutex<Result<ParsecData, ParsecAccessError>> =
        Mutex::new(ParsecData::new());
}

static DATA: [&Mutex<Result<ParsecData, ParsecAccessError>>; 15] = [
    &Z0_0001_DATA,
    &Z0_0002_DATA,
    &Z0_0005_DATA,
    &Z0_0010_DATA,
    &Z0_0020_DATA,
    &Z0_0040_DATA,
    &Z0_0060_DATA,
    &Z0_0080_DATA,
    &Z0_0100_DATA,
    &Z0_0140_DATA,
    &Z0_0170_DATA,
    &Z0_0200_DATA,
    &Z0_0300_DATA,
    &Z0_0400_DATA,
    &Z0_0600_DATA,
];

#[derive(Deserialize, Serialize)]
pub(crate) struct ParsecData {
    pub metallicity: Metallicity,
    pub(super) data: Vec<Trajectory>,
}