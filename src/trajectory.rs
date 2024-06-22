//! Contains the `Trajectory` struct, which holds the PARSEC data for a given metallicity and initial mass.

use simple_si_units::base::{Mass, Time};
use std::ops::Index;

use super::line::ParsecLine;

/// The data struct holding the PARSEC data for a given metallicity and initial mass.
/// This struct cannot be created directly, but can only be read and accessed through the crate api.
/// If you know the age index, the contained parameters can be accessed via the index operator.
#[derive(Clone)]
pub struct Trajectory {
    params: Vec<ParsecLine>,
    /// The initial mass of the star.
    pub initial_mass: Mass<f64>,
    /// The lifetime of the star. This is simply the age of the last entry in the trajectory.
    pub lifetime: Time<f64>,
    /// The ages of the star in years. This information is redundant, but useful for looping over the trajectory.
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
        let initial_mass = match params.first() {
            Some(params) => params.mass,
            None => Mass { kg: 0. },
        };
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

#[cfg(test)]
mod test {
    use super::Trajectory;

    #[test]
    fn constructor_with_empty_params_does_not_throw() {
        let trajectory = Trajectory::new(vec![]);
        assert!(trajectory.is_empty());
    }
}
