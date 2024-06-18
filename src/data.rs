use serde::{Deserialize, Serialize};
use std::ops::Index;

use crate::{
    access::metallicity::METALLICITY_NAMES,
    error::ParsecAccessError,
    file::{create_parsec_data_file, get_data_dir, read_existing_parsec_file},
    trajectory::Trajectory,
};

#[derive(Deserialize, Serialize)]
pub struct ParsecData {
    pub metallicity_in_mass_fraction: f64,
    pub(crate) data: Vec<Trajectory>,
}

impl ParsecData {
    pub(crate) fn new(metallicity_index: usize) -> Result<ParsecData, ParsecAccessError> {
        let data_dir = get_data_dir()?;
        let metallicity_name = METALLICITY_NAMES[metallicity_index].to_string();
        let file_path = data_dir.join(metallicity_name + ".rmp");

        if file_path.exists() {
            read_existing_parsec_file(file_path)
        } else {
            create_parsec_data_file(metallicity_index, &data_dir, file_path)
        }
    }

    pub(crate) fn is_filled(&self) -> bool {
        let mut is_filled = !self.data.is_empty();
        for trajectory in self.data.iter() {
            is_filled = is_filled && !trajectory.is_empty();
        }
        is_filled
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
