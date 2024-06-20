use simple_si_units::base::{Mass, Time};
use std::ops::Index;

use super::line::ParsecLine;

#[derive(Clone)]
pub struct Trajectory {
    params: Vec<ParsecLine>,
    pub initial_mass: Mass<f64>,
    pub lifetime: Time<f64>,
    pub ages_in_years: Vec<f64>,
}

impl Index<usize> for Trajectory {
    type Output = ParsecLine;

    fn index(&self, index: usize) -> &Self::Output {
        &self.params[index]
    }
}

impl Trajectory {
    pub(super) fn new(params: Vec<ParsecLine>) -> Self {
        let initial_mass = params[0].mass;
        let lifetime = match params.last() {
            Some(last) => last.age,
            None => Time { s: 0. },
        };
        let ages_in_years = params.iter().map(|line| line.age.to_yr()).collect();

        Self {
            params,
            initial_mass,
            lifetime,
            ages_in_years,
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.params.is_empty()
    }
}
