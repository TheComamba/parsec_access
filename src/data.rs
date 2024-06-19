use serde::{Deserialize, Serialize};
use std::ops::Index;

use crate::{
    access::metallicity::METALLICITY_NAMES,
    file::{create_parsec_data_file, get_data_dir, read_existing_parsec_file},
    trajectory::Trajectory,
};

#[derive(Deserialize, Serialize)]
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
        let metallicity_name = METALLICITY_NAMES[metallicity_index].to_string();
        let file_path = data_dir.join(metallicity_name + ".rmp");

        let result = if file_path.exists() {
            read_existing_parsec_file(file_path)
        } else {
            create_parsec_data_file(metallicity_index, &data_dir, file_path)
        };
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
