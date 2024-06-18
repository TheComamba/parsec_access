//! Provides a set of api functions exposing the main functionality of this crate.

use simple_si_units::base::{Mass, Time};

use crate::{
    access::{
        data::DATA,
        masses::MASSES,
        metallicity::{METALLICITIES_IN_DEX, METALLICITIES_IN_MASS_FRACTION},
    },
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
    for i in 0..METALLICITIES_IN_MASS_FRACTION.len() {
        if !DATA[i].is_valid() {
            return false;
        }
    }
    return true;
}

/// Fetches a reference to the ParsecData object for a given metallicity.
/// This is functionally similar to get_closest_data, but much faster.
/// To find the correct metallicity index, use get_closest_metallicity_index_from_mass_fraction.
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_data, is_data_ready};
///
/// assert!(is_data_ready());
/// let data = get_data(1);
/// assert!(data.metallicity_in_mass_fraction > 0.);
/// let first_trajectory = data[0];
/// ```
pub fn get_data(metallicity_index: usize) -> &'static ParsecData {
    &DATA[metallicity_index]
}

/// Fetches a reference to the ParsecData object for the metallicity that is closest to the provided value.
/// The untyped input value is expected to be a mass fraction of all metals to total mass.
/// This is a convenience wrapper around the much faster get_data().
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_closest_data, is_data_ready};
///
/// assert!(is_data_ready());
/// let data = get_closest_data(0.01);
/// assert!(data.metallicity_in_mass_fraction > 0.009);
/// assert!(data.metallicity_in_mass_fraction < 0.011);
/// ```
pub fn get_closest_data(mass_fraction: f64) -> &'static ParsecData {
    let metallicity_index = get_closest_metallicity_index_from_mass_fraction(mass_fraction);
    get_data(metallicity_index)
}

/// Fetches a reference to the trajectory for a given metallicity and mass.
/// This is functionally similar to get_closest_trajectory, but much faster.
/// To find the correct metallicity and mass index, use get_closest_metallicity_index_from_mass_fraction and get_closest_mass_index.
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_trajectory, is_data_ready};
///
/// assert!(is_data_ready());
/// let trajectory = get_trajectory(1, 2);
/// assert!(trajectory.initial_mass > 0.);
/// assert!(trajectory.lifetime > 0.);
/// ```
pub fn get_trajectory(metallicity_index: usize, mass_index: usize) -> &'static Trajectory {
    &DATA[metallicity_index].data[mass_index]
}

/// Fetches a reference to the trajectory for the metallicity and mass that are closest to the provided values.
/// The untyped mass_fraction is expected to be the mass fraction of all metals to total mass.
/// This is a convenience wrapper around the much faster get_trajectory().
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_closest_trajectory, is_data_ready};
///
/// assert!(is_data_ready());
/// let trajectory = get_closest_trajectory(0.01, 1.);
/// assert!(trajectory.initial_mass.to_solar_masses() > 0.9);
/// assert!(trajectory.initial_mass.to_solar_masses() < 1.1);
/// ```
pub fn get_closest_trajectory(mass_fraction: f64, mass: Mass<f64>) -> &'static Trajectory {
    let metallicity_index = get_closest_metallicity_index_from_mass_fraction(mass_fraction);
    let mass_index = get_closest_mass_index(metallicity_index, mass);
    get_trajectory(metallicity_index, mass_index)
}

/// Fetches a reference to the ParsecLine object for a given metallicity, mass, and age.
/// This is functionally similar to get_closest_parameters, but much faster.
/// To find the correct metallicity, mass, and age index, use get_closest_metallicity_index_from_mass_fraction, get_closest_mass_index, and get_closest_age_index.
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_parameters, is_data_ready};
///
/// assert!(is_data_ready());
/// let parameters = get_parameters(1, 2, 3);
/// assert!(parameters.mass.to_solar_masses() > 0.);
/// assert!(parameters.age.to_yr() > 0.);
/// assert!(parameters.luminous_intensity.to_cd() > 0.);
/// assert!(parameters.temperature.to_kelvin() > 0.);
/// assert!(parameters.radius.to_solar_radii() > 0.);
/// ```
pub fn get_parameters(
    metallicity_index: usize,
    mass_index: usize,
    age_index: usize,
) -> &'static ParsecLine {
    &DATA[metallicity_index].data[mass_index][age_index]
}

/// Fetches a reference to the ParsecLine object for the metallicity, mass, and age that are closest to the provided values.
/// The untyped mass_fraction is expected to be the mass fraction of all metals to total mass.
/// This is a convenience wrapper around the much faster get_parameters().
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_closest_parameters, is_data_ready};
///
/// assert!(is_data_ready());
/// let parameters = get_closest_parameters(0.01, Mass::from_solar(1.), Time::from_Gyr(1.));
/// assert!(parameters.mass.to_solar_masses() > Mass::from_solar(0.9));
/// assert!(parameters.mass.to_solar_masses() < Mass::from_solar(1.1));
/// assert!(parameters.age.to_yr() > Time::from_Gyr(0.9));
/// assert!(parameters.age.to_yr() < Time::from_Gyr(1.1));
/// ```
pub fn get_closest_parameters(
    mass_fraction: f64,
    mass: Mass<f64>,
    age: Time<f64>,
) -> &'static ParsecLine {
    let metallicity_index = get_closest_metallicity_index_from_mass_fraction(mass_fraction);
    let mass_index = get_closest_mass_index(metallicity_index, mass);
    let age_index = get_closest_age_index(metallicity_index, mass_index, age);
    get_parameters(metallicity_index, mass_index, age_index)
}

/// Returns a reference to the array of available metallicities in units of the mass fractions Z.
///
/// # Example
/// ```
/// use parsec_access::getters::get_metallicities_in_mass_fractions;
///
/// assert!(get_metallicities_in_mass_fractions().len() > 0);
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
/// assert!(get_metallicities_in_fe_dex().len() > 0);
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

/// Returns a reference to the array of available masses in units of solar masses.
///
/// # Example
/// ```
/// use parsec_access::getters::get_masses_in_solar;
///
/// assert!(get_masses_in_solar(0).len() > 0);
///
/// for mass in get_masses_in_solar(0).take(10) {
///    println!("Mass in solar masses: {}", mass);
/// }
/// ```
pub fn get_masses_in_solar(metallicity_index: usize) -> &'static [f64] {
    &MASSES[metallicity_index]
}

/// Finds the closest mass enum variant to the given mass in solar masses.
///
/// The midpoint between two masses is calculated as the arithmetic mean of the two solar masses.
///
/// # Example
/// ```
/// use parsec_access::getters::get_closest_mass_index;
///
/// let index = get_closest_mass_index(0, Mass::from_solar(1.));
/// let expected = get_masses_in_solar(0)[index];
/// assert!((expected-1.).abs() < 1e-8);
/// let index = get_closest_mass_index(0, Mass::from_solar(0.));
/// let expected = 0;
/// assert_eq!(index, expected);
/// let index = get_closest_mass_index(0, Mass::from_solar(1000.));
/// let expected = get_masses_in_solar(0).len() - 1;
/// assert_eq!(index, expected);
/// ```
pub fn get_closest_mass_index(metallicity_index: usize, mass: Mass<f64>) -> usize {
    get_closest_index(&MASSES[metallicity_index], mass.to_solar_mass())
}

/// Returns a reference to the array of available ages in years.
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_ages_in_years, is_data_ready};
///
/// assert!(is_data_ready());
///
/// assert!(get_ages_in_years(0, 0).len() > 0);
///
/// for age in get_ages_in_years(0, 0).take(10) {
///   println!("Age in years: {}", age);
/// }
/// ```
pub fn get_ages_in_years(metallicity_index: usize, mass_index: usize) -> &'static [f64] {
    &DATA[metallicity_index].data[mass_index].ages_in_years
}

/// Finds the closest age enum variant to the given age in years.
///
/// The midpoint between two ages is calculated as the arithmetic mean of the two years.
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_closest_age_index, is_data_ready};
///
/// assert!(is_data_ready());
///
/// let index = get_closest_age_index(0, 0, Time::from_Gyr(1.));
/// let expected = get_ages_in_years(0, 0)[index];
/// assert!((expected-1.).abs() < 1e-8);
/// let index = get_closest_age_index(0, 0, Time::from_yr(0.));
/// let expected = 0;
/// assert_eq!(index, expected);
/// let index = get_closest_age_index(0, 0, Time::from_yr(1e15));
/// let expected = get_ages_in_years(0, 0).len() - 1;
/// assert_eq!(index, expected);
/// ```
pub fn get_closest_age_index(metallicity_index: usize, mass_index: usize, age: Time<f64>) -> usize {
    get_closest_index(
        &DATA[metallicity_index].data[mass_index].ages_in_years,
        age.to_yr(),
    )
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
