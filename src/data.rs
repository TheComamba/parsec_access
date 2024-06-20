use serde::{Deserialize, Serialize};
use std::ops::Index;

use crate::{
    access::metallicity::METALLICITY_NAMES,
    file::{create_serialised_parsec_data_file, get_data_dir, read_serialised_parsec_file},
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
            read_serialised_parsec_file(file_path)
        } else {
            create_serialised_parsec_data_file(metallicity_index, &data_dir, file_path)
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

#[cfg(test)]
mod tests {
    use simple_si_units::base::Mass;

    use super::*;

    #[test]
    #[ignore]
    fn data_access_is_fast() {
        // const N: usize = 1e6 as usize;
        // const PRIME1: usize = 1009;
        // const PRIME2: usize = 1013;
        // const PRIME3: usize = 10007;
        // const MAX_METALLICITY_INDEX: usize = 10;
        // const MAX_MASS_INDEX: usize = 50;
        // const MAX_TRAJECTORY_INDEX: usize = 100;

        // // Ensure that the data is loaded into memory.
        // let _ = DATA[1].as_ref().unwrap()[1][1];

        // // Create pseudo-random indices.
        // let mut indices = Vec::new();
        // for i in 0..N {
        //     let metallicity_index = (i * PRIME1) % MAX_METALLICITY_INDEX;
        //     let mass_index = (i * PRIME2) % MAX_MASS_INDEX;
        //     let trajectory_index = (i * PRIME3) % MAX_TRAJECTORY_INDEX;
        //     indices.push((metallicity_index, mass_index, trajectory_index));
        // }

        // // Access the data in a pseudo-random order.
        // let now = std::time::Instant::now();
        // let mut total_mass = Mass { kg: 0. };
        // for (metallicity_index, mass_index, trajectory_index) in indices {
        //     let m = DATA[metallicity_index].as_ref().unwrap()[mass_index][trajectory_index].mass;
        //     total_mass += m;
        // }
        // let elapsed = now.elapsed();
        // println!("Collected a total mass of {} solar masses.", total_mass);

        // println!("Accessing {} data points took {:?}", N, elapsed);
    }
}
