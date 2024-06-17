use simple_si_units::base::{Mass, Time};

use crate::{
    access::{data::ParsecData, metallicity::Metallicity},
    line::ParsecLine,
    trajectory::Trajectory,
};

pub fn getData(metallicity: &Metallicity) -> &'static ParsecData {
    todo!()
}

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
