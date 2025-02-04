//! Provides a set of api functions exposing the main functionality of this crate.

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
/// use parsec_access::getters::is_data_ready;
///
/// assert!(is_data_ready());
/// ```
pub fn is_data_ready() -> bool {
    for i in 0..METALLICITIES_IN_MASS_FRACTION.len() {
        if !DATA[i].is_valid() {
            return false;
        }
    }
    true
}

/// Fetches a reference to the ParsecData object for a given metallicity.
/// This is functionally similar to get_closest_data, but faster by about a factor of 10.
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
/// let first_trajectory = &data[0];
/// ```
pub fn get_data(metallicity_index: usize) -> &'static ParsecData {
    DATA[metallicity_index]
}

/// Fetches a reference to the ParsecData object for the metallicity that is closest to the provided value.
/// The untyped input value is expected to be a mass fraction of all metals to total mass.
/// This is a convenience wrapper around the faster get_data().
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
/// This is functionally similar to get_closest_trajectory, but faster by about a factor of 10.
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
/// assert!(trajectory.initial_mass.to_solar_mass() > 0.);
/// assert!(trajectory.lifetime.to_yr() > 0.);
/// ```
pub fn get_trajectory(metallicity_index: usize, mass_index: usize) -> &'static Trajectory {
    &DATA[metallicity_index].data[mass_index]
}

/// Fetches a reference to the trajectory for the metallicity and mass that are closest to the provided values.
/// The untyped mass_fraction is expected to be the mass fraction of all metals to total mass.
/// This is a convenience wrapper around the faster get_trajectory().
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_closest_trajectory, is_data_ready};
/// use simple_si_units::base::Mass;
///
/// assert!(is_data_ready());
/// let trajectory = get_closest_trajectory(0.01, Mass::from_solar_mass(1.));
/// assert!(trajectory.initial_mass.to_solar_mass() > 0.9);
/// assert!(trajectory.initial_mass.to_solar_mass() < 1.1);
/// ```
pub fn get_closest_trajectory(mass_fraction: f64, mass: Mass<f64>) -> &'static Trajectory {
    let metallicity_index = get_closest_metallicity_index_from_mass_fraction(mass_fraction);
    let mass_index = get_closest_mass_index(metallicity_index, mass);
    get_trajectory(metallicity_index, mass_index)
}

/// Fetches a reference to the ParsecLine object for a given metallicity, mass, and age.
/// This is functionally similar to get_closest_parameters, but faster by about a factor of 10.
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
/// assert!(parameters.mass.to_solar_mass() > 0.);
/// assert!(parameters.age.to_yr() > 0.);
/// assert!(parameters.luminosity_in_solar > 0.);
/// assert!(parameters.temperature.to_K() > 0.);
/// assert!(parameters.radius.to_km() > 0.);
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
/// This is a convenience wrapper around the faster get_parameters().
///
/// # Safety
///
/// This function does not perform any out-of-bounds checks.
/// Call is_data_ready() once before using this function to ensure that the data is loaded and valid.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_closest_parameters, is_data_ready};
/// use simple_si_units::base::Mass;
/// use simple_si_units::base::Time;
///
/// assert!(is_data_ready());
/// let parameters = get_closest_parameters(0.01, Mass::from_solar_mass(1.), Time::from_Gyr(1.));
/// assert!(parameters.mass > Mass::from_solar_mass(0.9));
/// assert!(parameters.mass < Mass::from_solar_mass(1.1));
/// assert!(parameters.age > Time::from_Gyr(0.9));
/// assert!(parameters.age < Time::from_Gyr(1.1));
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
/// use parsec_access::getters::{get_closest_metallicity_index_from_mass_fraction, get_metallicities_in_mass_fractions};
///
/// let index = get_closest_metallicity_index_from_mass_fraction(0.0101);
/// let expected = get_metallicities_in_mass_fractions()[index];
/// assert!((expected-0.01).abs() < 1e-8);
/// let index = get_closest_metallicity_index_from_mass_fraction(0.);
/// let expected = 0;
/// assert_eq!(index, expected);
/// let index = get_closest_metallicity_index_from_mass_fraction(0.999);
/// let expected = get_metallicities_in_mass_fractions().len() - 1;
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
/// use parsec_access::getters::{get_closest_metallicity_index_from_fe_dex, get_metallicities_in_fe_dex};
///
/// let index = get_closest_metallicity_index_from_fe_dex(0.);
/// println!("{} dex is the closest metallicity to 0 dex", get_metallicities_in_fe_dex()[index]);
/// let index = get_closest_metallicity_index_from_fe_dex(-10.);
/// let expected = 0;
/// assert_eq!(index, expected);
/// let index = get_closest_metallicity_index_from_fe_dex(10.);
/// let expected = get_metallicities_in_fe_dex().len() - 1;
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
/// for mass in get_masses_in_solar(0) {
///    println!("Mass in solar masses: {}", mass);
/// }
/// ```
pub fn get_masses_in_solar(metallicity_index: usize) -> &'static [f64] {
    MASSES[metallicity_index]
}

/// Finds the closest mass enum variant to the given mass in solar masses.
///
/// The midpoint between two masses is calculated as the arithmetic mean of the two solar masses.
///
/// # Example
/// ```
/// use parsec_access::getters::{get_closest_mass_index, get_masses_in_solar};
/// use simple_si_units::base::Mass;
///
/// let index = get_closest_mass_index(0, Mass::from_solar_mass(1.));
/// let expected = get_masses_in_solar(0)[index];
/// assert!((expected-1.).abs() < 1e-8);
/// let index = get_closest_mass_index(0, Mass::from_solar_mass(0.));
/// let expected = 0;
/// assert_eq!(index, expected);
/// let index = get_closest_mass_index(0, Mass::from_solar_mass(1000.));
/// let expected = get_masses_in_solar(0).len() - 1;
/// assert_eq!(index, expected);
/// ```
pub fn get_closest_mass_index(metallicity_index: usize, mass: Mass<f64>) -> usize {
    get_closest_index(MASSES[metallicity_index], mass.to_solar_mass())
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
/// let mut count = 0;
/// for age in get_ages_in_years(0, 0) {
///   println!("Age in years: {}", age);
///   count += 1;
///   if count > 10 {
///     break;
///   }
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
/// use parsec_access::getters::{get_ages_in_years, get_closest_age_index, is_data_ready};
/// use simple_si_units::base::Time;
///
/// assert!(is_data_ready());
///
/// let index = get_closest_age_index(0, 0, Time::from_Gyr(1.));
/// let expected = get_ages_in_years(0, 0)[index];
/// assert!((Time::from_yr(expected)-Time::from_Gyr(1.)).to_Gyr().abs() < 1e-1);
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
