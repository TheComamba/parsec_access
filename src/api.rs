//! Provides a set of api functions exposing the main functionality of this crate.

use simple_si_units::base::{Mass, Time};

use crate::{
    access::{data::ParsecData, metallicity::Metallicity},
    line::ParsecLine,
    trajectory::Trajectory,
};

/// Fetches a reference to the ParsecData object for a given metallicity.
/// This is functionally similar to getClosestData, but much faster.
// Todo Example
pub fn getData(metallicity: &Metallicity) -> &'static ParsecData {
    todo!()
}

/// Fetches a reference to the ParsecData object for the metallicity that is closest to the provided value.
/// The untyped input value is expected to be a mass fraction of all metals to total mass.
/// This is a convenience wrapper around the much faster getData().
// Todo Example
pub fn getClosestData(mass_fraction: f64) -> &'static ParsecData {
    todo!()
}

pub fn getTrajectory(metallicity: &Metallicity, mass_index: usize) -> &'static Trajectory {
    todo!()
}

pub fn getClosestTrajectory(mass_fraction: f64, mass: Mass<f64>) -> &'static Trajectory {
    todo!()
}

pub fn getParameters(
    metallicity: &Metallicity,
    mass_index: usize,
    age_index: usize,
) -> &'static ParsecLine {
    todo!()
}

pub fn getClosestParameters(
    mass_fraction: f64,
    mass: Mass<f64>,
    age: Time<f64>,
) -> &'static ParsecLine {
    todo!()
}
