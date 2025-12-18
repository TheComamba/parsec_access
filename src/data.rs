//! Contains the `ParsecData` struct, which holds the PARSEC data for a given metallicity.

use std::ops::Index;

use crate::{
    file::{get_data_dir, read_data_files},
    trajectory::Trajectory,
};

/// The data struct holding the PARSEC data for a given metallicity.
/// This struct cannot be created directly, but can only be read and accessed through the crate api.
/// If you know the metallicity index, the contained trajectories can be accessed via the index operator.
pub struct ParsecData {
    /// The metallicity of the data in units of mass fraction Z.
    pub metallicity_in_mass_fraction: f64,
    pub(crate) data: Vec<Trajectory>,
}

impl ParsecData {
    pub(crate) fn new(metallicity_index: usize) -> ParsecData {
        let data_dir = match get_data_dir() {
            Ok(dir) => dir,
            Err(err) => {
                eprintln!(
                    "Error getting data directory for metallicity index {metallicity_index}: {err}"
                );
                return ParsecData::default();
            }
        };
        let result = read_data_files(metallicity_index, &data_dir);
        match result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("Error reading PARSEC data for metallicity index {metallicity_index} from data dir '{}': {err}", data_dir.display());
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
    use uom::si::{
        f64::{Length, Mass, ThermodynamicTemperature, Time},
        length::meter,
        mass::kilogram,
        thermodynamic_temperature::kelvin,
        time::second,
    };

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
            mass: Mass::new::<kilogram>(1.),
            age: Time::new::<second>(1.),
            luminosity_in_solar: 1.,
            temperature: ThermodynamicTemperature::new::<kelvin>(1.),
            radius: Length::new::<meter>(1.),
        };
        data.data.push(Trajectory::new(vec![valid_line]));
        data.data.push(Trajectory::new(vec![]));
        assert!(!data.is_valid());
    }
}
