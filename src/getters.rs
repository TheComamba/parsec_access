use crate::access::data::ParsecData;

impl ParsecData {
    pub(super) fn find_closest_index(list: &[f64], mass: f64) -> usize {
        let mut min_index = 0;
        let mut max_index = list.len() - 1;
        while max_index - min_index > 1 {
            let mid_index = (max_index + min_index) / 2;
            let mid_mass = list[mid_index];
            if mass > mid_mass {
                min_index = mid_index;
            } else {
                max_index = mid_index;
            }
        }
        if (mass - list[min_index]).abs() < (mass - list[max_index]).abs() {
            min_index
        } else {
            max_index
        }
    }

    pub(super) fn is_filled(&self) -> bool {
        let mut is_filled = !self.data.is_empty();
        for trajectory in self.data.iter() {
            is_filled = is_filled && !trajectory.is_empty();
        }
        is_filled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn masses_are_mapped_to_themselves() {
    //     const SMALL_OFFSET: f64 = 1e-4;
    //     for expected_mass in ParsecData::SORTED_MASSES.iter() {
    //         let mass = *expected_mass;
    //         let mass_index = ParsecData::get_closest_mass_index(mass);
    //         let mapped_mass = ParsecData::SORTED_MASSES[mass_index];
    //         println!("mass: {}, mapped_mass: {}", mass, mapped_mass);
    //         assert!((expected_mass - mapped_mass).abs() < SMALL_OFFSET);

    //         let mass = *expected_mass + SMALL_OFFSET;
    //         let mass_index = ParsecData::get_closest_mass_index(mass);
    //         let mapped_mass = ParsecData::SORTED_MASSES[mass_index];
    //         println!("mass: {}, mapped_mass: {}", mass, mapped_mass);
    //         assert!((expected_mass - mapped_mass).abs() < SMALL_OFFSET);

    //         let mass = *expected_mass - SMALL_OFFSET;
    //         let mass_index = ParsecData::get_closest_mass_index(mass);
    //         let mapped_mass = ParsecData::SORTED_MASSES[mass_index];
    //         println!("mass: {}, mapped_mass: {}", mass, mapped_mass);
    //         assert!((expected_mass - mapped_mass).abs() < SMALL_OFFSET);
    //     }
    // }

    // #[test]
    // fn closest_params_map_to_correct_age() {
    //     for mass_index in 0..ParsecData::SORTED_MASSES.len() {
    //         let trajectory = {
    //             let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
    //             let parsec_data = parsec_data_mutex.as_ref().unwrap();
    //             (*parsec_data.get_trajectory_via_index(mass_index)).clone()
    //         };
    //         for (age_index, params) in trajectory.get_params().iter().enumerate() {
    //             let expected_age = params.age_in_years;
    //             let params_index = trajectory.get_closest_params_index(expected_age);
    //             let received_age = trajectory.get_params()[params_index].age_in_years;
    //             assert!(
    //                 (received_age - expected_age).abs() < 1e-8,
    //                 "Expected age {} should be exactly the same as received age {} (Mass index {}, Age index {})",
    //                 expected_age, received_age, mass_index, age_index
    //             );

    //             let little_less = expected_age - 0e-5;
    //             let params_index = trajectory.get_closest_params_index(little_less);
    //             let received_age = trajectory.get_params()[params_index].age_in_years;
    //             assert!(
    //                 (received_age - expected_age).abs() < 1e-8,
    //                 "Expected age {} should be a little less than received age {} (Mass index {}, Age index {})",
    //                 expected_age, received_age, mass_index, age_index
    //             );

    //             let little_more = expected_age + 0e-5;
    //             let params_index = trajectory.get_closest_params_index(little_more);
    //             let received_age = trajectory.get_params()[params_index].age_in_years;
    //             assert!(
    //                 (received_age - expected_age).abs() < 1e-8,
    //                 "Expected age {} should be a little more than received age {} (Mass index {}, Age index {})",
    //                 expected_age, received_age, mass_index, age_index
    //             );
    //         }
    //     }
    // }

    // #[test]
    // fn lifetimes_of_real_stars_are_within_limits() {
    //     let stars = get_many_stars();
    //     for star in stars {
    //         let mass = star.mass;
    //         let lifetime = star.lifetime;
    //         let mass_index = ParsecData::get_closest_mass_index(mass.to_solar_mass());
    //         let expected_lifetime = {
    //             let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
    //             let parsec_data = parsec_data_mutex.as_ref().unwrap();
    //             let trajectory = parsec_data.get_trajectory_via_index(mass_index);
    //             trajectory.lifetime
    //         };
    //         let ratio = lifetime / expected_lifetime;
    //         assert!(
    //             (0.99..1.01).contains(&ratio),
    //             "Star {} is deviant\nlifetime: {} Gyr,\nexpected lifetime: {} Gyr,\nratio: {:.2}\n",
    //             star.astronomical_name,
    //             lifetime.astro_display(),
    //             expected_lifetime.astro_display(),
    //             ratio
    //         );
    //     }
    // }

    // #[test]
    // fn lifetime_mostly_decreases_with_mass() {
    //     let trajectories = {
    //         let parsec_data_mutex = PARSEC_DATA.lock().unwrap();
    //         let parsec_data = parsec_data_mutex.as_ref().unwrap();
    //         parsec_data.data.clone()
    //     };
    //     for (i, trajectory) in trajectories.iter().enumerate() {
    //         if i == 0 {
    //             continue;
    //         }
    //         let lifetime = trajectory.lifetime;
    //         let previous_lifetime = trajectories[i - 1].lifetime;
    //         assert!(
    //             lifetime < 1.2 * previous_lifetime,
    //             "Lifetime of star {} is {} years, while lifetime of star {} is {} years",
    //             i,
    //             lifetime,
    //             i - 1,
    //             previous_lifetime
    //         );
    //     }
    // }
}
