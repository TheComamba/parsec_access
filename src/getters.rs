//! Provides a set of api functions exposing the main functionality of this crate.

use simple_si_units::base::{Mass, Time};

use crate::{
    access::{masses::MASSES, metallicity::{METALLICITIES_IN_DEX, METALLICITIES_IN_MASS_FRACTION}},
    data::ParsecData,
    line::ParsecLine,
    trajectory::Trajectory,
};

/// Loads the Parsec data and makes sure that it is valid.
/// This step can and should be used once before accessing the data, because the getter functions do not perform any checks due to performance reasons.
///
/// # Example
/// ```
/// use parsec::getters::is_data_ready;
///
/// assert!(is_data_ready());
/// ```
pub fn is_data_ready() -> bool {
    todo!()
}

/// Fetches a reference to the ParsecData object for a given metallicity.
/// This is functionally similar to getClosestData, but much faster.
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
// Todo Example
pub fn get_data(metallicity_index: usize) -> &'static ParsecData {
    todo!()
}

/// Fetches a reference to the ParsecData object for the metallicity that is closest to the provided value.
/// The untyped input value is expected to be a mass fraction of all metals to total mass.
/// This is a convenience wrapper around the much faster getData().
// Todo Example
pub fn get_closest_data(mass_fraction: f64) -> &'static ParsecData {
    todo!()
}

pub fn get_trajectory(metallicity_index: usize, mass_index: usize) -> &'static Trajectory {
    todo!()
}

pub fn get_closest_trajectory(mass_fraction: f64, mass: Mass<f64>) -> &'static Trajectory {
    todo!()
}

pub fn get_parameters(
    metallicity_index: usize,
    mass_index: usize,
    age_index: usize,
) -> &'static ParsecLine {
    todo!()
}

pub fn get_closest_parameters(
    mass_fraction: f64,
    mass: Mass<f64>,
    age: Time<f64>,
) -> &'static ParsecLine {
    todo!()
}

/// Returns a reference to the array of available metallicities in units of the mass fractions Z.
///
/// # Example
/// ```
/// use parsec_access::getters::get_metallicities_in_mass_fractions;
///
/// for metallicity in get_metallicities_in_mass_fractions() {
///     println!("Metallicity mass fraction: {}", metallicity);
/// }
/// ```
pub fn get_metallicities_in_mass_fractions() -> &'static [f64] {
    &METALLICITIES_IN_MASS_FRACTION
}

/// Returns a reference to the array of available metallicities in units of dex for the element iron,
///
/// This relies on several assumptions:
///
/// PARSEC lists metallicity as
/// Z = m_M / m_tot ,
/// the mass fraction of all metals (i.e. elements heavier than Helium) in the star.
///
/// The chemical abundance ratio on the other hand is conventionally given as
/// [Fe/H] = log10(N_Fe / N_H) - log10(N_Fe / N_H)_sun ,
/// where N_Fe and N_H are the number densities of iron and hydrogen atoms, respectively.
///
/// Assuming that iron always makes up more or less the same fraction of the metal mass,
/// N_Fe = a * m_M
/// and that the total mass is dominated by hydrogen,
/// N_H = m_tot ,
/// we find
/// [Fe/H] = log10(a * m_M / m_tot) - log10(a * m_M / m_tot)_sun
///        = log10(Z / Z_sun) .
///
/// The solar metallicity is Z_sun = 0.0122.
///
/// # Example
/// ```
/// use parsec_access::getters::get_metallicities_in_fe_dex;
///
/// for metallicity in get_metallicities_in_fe_dex() {
///     println!("Metallicity fe dex: {}", metallicity);
/// }
/// ```
pub fn get_metallicities_in_fe_dex() -> &'static [f64] {
    &METALLICITIES_IN_DEX
}

/// Finds the closest metallicity enum variant to the given mass fraction Z.
///
/// The midpoint between two metallicities is calculated as the arithmetic mean of the two mass fractions.
/// Note that this means that there are cases where find_closest_from_fe_dex can lead to a different result.
///
/// # Example
/// ```
/// use parsec_access::getters::get_closest_metallicity_index_from_mass_fraction;
///
/// let index = get_closest_metallicity_index_from_mass_fraction(0.0101);
/// let expected = get_metallicites_in_mass_fractions()[index];
/// assert!((expected-0.01).abs() < 1e-8);
/// let index = get_closest_metallicity_index_from_mass_fraction(0.);
/// let expected = 0;
/// assert_eq!(index, expected);
/// let index = get_closest_metallicity_index_from_mass_fraction(0.999);
/// let expected = get_metallicites_in_mass_fractions().len() - 1;
/// assert_eq!(index, expected);
/// ```
pub fn get_closest_metallicity_index_from_mass_fraction(mass_fraction: f64) -> usize {
    get_closest_index(&METALLICITIES_IN_MASS_FRACTION, mass_fraction)
}

/// Finds the closest metallicity enum variant to the given dex for the element iron.
///
/// See the documentation of get_metallicities_in_fe_dex() for the assumptions going into the unit conversion.
///
/// The midpoint between two metallicities is calculated as the arithmetic mean of the two dex values.
/// Note that this means that there are cases where find_closest_from_mass_fraction can lead to a different result.
///
/// # Example
/// ```
/// use parsec_access::getters::get_closest_metallicity_index_from_fe_dex;
///
/// let index = get_closest_metallicity_index_from_fe_dex(0.);
/// println!("{} dex is the closest metallicity to 0 dex", get_metallicities_in_fe_dex()[index]);:
/// let index = get_closest_metallicity_index_from_fe_dex(-10.);
/// let expected = 0;
/// assert_eq!(index, expected);
/// let index = get_closest_metallicity_index_from_fe_dex(10.);
/// let expected = get_metallicites_in_mass_fractions().len() - 1;
/// assert_eq!(index, expected);
/// ```
pub fn get_closest_metallicity_index_from_fe_dex(fe_dex: f64) -> usize {
    get_closest_index(&METALLICITIES_IN_DEX, fe_dex)
}

pub fn get_masses_in_solar(metallicity_index: usize) -> &'static [f64] {
    &MASSES[metallicity_index]
}

pub fn get_closest_mass_index(metallicity_index: usize, mass: Mass<f64>) -> usize {
    todo!()
}

pub fn get_ages_in_years(metallicity_index: usize, mass_index: usize) -> &'static [f64] {
    todo!()
}

pub fn get_closest_age_index(metallicity_index: usize, mass_index: usize, age: Time<f64>) -> usize {
    todo!()
}

pub(super) fn get_closest_index(list: &[f64], value: f64) -> usize {
    let mut min_index = 0;
    let mut max_index = list.len() - 1;
    while max_index - min_index > 1 {
        let mid_index = (max_index + min_index) / 2;
        let mid_mass = list[mid_index];
        if value > mid_mass {
            min_index = mid_index;
        } else {
            max_index = mid_index;
        }
    }
    if (value - list[min_index]).abs() < (value - list[max_index]).abs() {
        min_index
    } else {
        max_index
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
