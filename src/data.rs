use std::ops::Index;

use crate::{
    file::{get_data_dir, read_data_files},
    trajectory::Trajectory,
};

pub struct ParsecData {
    pub metallicity_in_mass_fraction: f64,
    pub(crate) data: Vec<Trajectory>,
}

impl ParsecData {
    pub(crate) fn new(metallicity_index: usize) -> ParsecData {
        let data_dir = match get_data_dir() {
            Ok(dir) => dir,
            Err(err) => {
                eprintln!("Error getting data directory: {}", err);
                return ParsecData::default();
            }
        };
        let result = read_data_files(metallicity_index, &data_dir);
        match result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error reading PARSEC data: {}", err);
                ParsecData::default()
            }
        }
    }

    pub(crate) fn is_valid(&self) -> bool {
        let has_valid_metallicity = self.metallicity_in_mass_fraction > 0.0;
        if !has_valid_metallicity {
            return false;
        }
        if self.data.is_empty() {
            return false;
        }
        for trajectory in self.data.iter() {
            if trajectory.is_empty() {
                return false;
            }
        }
        true
    }
}

impl Default for ParsecData {
    fn default() -> Self {
        Self {
            metallicity_in_mass_fraction: 0.0,
            data: Vec::new(),
        }
    }
}

impl Index<usize> for ParsecData {
    type Output = Trajectory;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

#[cfg(test)]
mod test {
    use simple_si_units::base::{Distance, Mass, Temperature, Time};

    use crate::line::ParsecLine;

    use super::*;

    #[test]
    fn default_data_is_invalid() {
        let data = ParsecData::default();
        assert!(!data.is_valid());
    }

    #[test]
    fn empty_data_is_invalid() {
        let mut data = ParsecData::default();
        data.metallicity_in_mass_fraction = 0.05;
        assert!(!data.is_valid());
    }

    #[test]
    fn data_with_empty_trajectory_is_invalid() {
        let mut data = ParsecData::default();
        data.metallicity_in_mass_fraction = 0.05;
        let valid_line = ParsecLine {
            mass: Mass { kg: 1. },
            age: Time { s: 1. },
            luminosity_in_solar: 1.,
            temperature: Temperature { K: 1. },
            radius: Distance { m: 1. },
        };
        data.data.push(Trajectory::new(vec![valid_line]));
        data.data.push(Trajectory::new(vec![]));
        assert!(!data.is_valid());
    }
}
