use astro_units::mass::solar_mass;
use parsec_access::getters::{
    get_closest_parameters, get_masses_in_solar, get_metallicities_in_mass_fractions,
    get_trajectory, is_data_ready,
};
use uom::{
    fmt::DisplayStyle,
    si::{
        f64::{Length, Mass, ThermodynamicTemperature, Time},
        length::kilometer,
        thermodynamic_temperature::kelvin,
        time::year,
    },
};

#[test]
fn recreating_the_sun() {
    assert!(is_data_ready());
    let sun_metallicity = 0.0122;
    let sun_mass = Mass::new::<solar_mass>(1.);
    let sun_age = Time::new::<year>(4.6e9);
    let sun_temperature = ThermodynamicTemperature::new::<kelvin>(5772.);
    let sun_radius = Length::new::<kilometer>(696_300.);
    let params = get_closest_parameters(sun_metallicity, sun_mass, sun_age);
    assert!(
        (params.luminosity_in_solar - 1.).abs() < 0.15,
        "Expected luminosity of 1 sol, got {}",
        params.luminosity_in_solar,
    );
    let ratio = params.temperature.get::<kelvin>() / sun_temperature.get::<kelvin>();
    assert!(
        (ratio - 1.).abs() < 0.15,
        "Expected {}, got {}, which is off by a factor of {}",
        sun_temperature.into_format_args(kelvin, DisplayStyle::Abbreviation),
        params
            .temperature
            .into_format_args(kelvin, DisplayStyle::Abbreviation),
        ratio,
    );
    let ratio = params.radius.get::<kilometer>() / sun_radius.get::<kilometer>();
    assert!(
        (ratio - 1.).abs() < 0.15,
        "Expected {}, got {}, which is off by a factor of {}",
        sun_radius.get::<kilometer>(),
        params.radius.get::<kilometer>(),
        ratio
    );
}

#[test]
fn lifetime_mostly_decreases_with_mass() {
    assert!(is_data_ready());
    let max_metallicity_index = get_metallicities_in_mass_fractions().len();
    for metallicity_index in 0..max_metallicity_index {
        let max_mass_index = get_masses_in_solar(metallicity_index).len();
        for mass_index in 0..max_mass_index {
            if mass_index == 0 {
                continue;
            }
            let lifetime = get_trajectory(metallicity_index, mass_index).lifetime;
            let previous_lifetime = get_trajectory(metallicity_index, mass_index - 1).lifetime;
            assert!(
                    lifetime < 1.2 * previous_lifetime,
                    "Metallicity index is {}, lifetime of star {} is {} years, while lifetime of star {} is {} years",
                    metallicity_index,
                    mass_index,
                    lifetime.into_format_args(year, DisplayStyle::Abbreviation),
                    mass_index - 1,
                    previous_lifetime.into_format_args(year, DisplayStyle::Abbreviation)
                );
        }
    }
}

#[test]
fn bolometric_luminosity_fits_radius_and_temperature() {
    let sun_temperature = ThermodynamicTemperature::new::<kelvin>(5772.);
    let sun_radius = Length::new::<kilometer>(696_300.);

    assert!(is_data_ready());
    let max_metallicity_index = get_metallicities_in_mass_fractions().len();
    for metallicity_index in 0..max_metallicity_index {
        let max_mass_index = get_masses_in_solar(metallicity_index).len();
        for mass_index in 0..max_mass_index {
            let trajectory = get_trajectory(metallicity_index, mass_index);
            let max_age_index = trajectory.ages_in_years.len();
            for age_index in 0..max_age_index {
                let params = &trajectory[age_index];
                let luminosity = params.luminosity_in_solar;
                let radius_in_solar =
                    params.radius.get::<kilometer>() / sun_radius.get::<kilometer>();
                let temperature_in_solar =
                    params.temperature.get::<kelvin>() / sun_temperature.get::<kelvin>();
                let expected_luminosity = radius_in_solar.powi(2) * temperature_in_solar.powi(4);
                let ratio = luminosity / expected_luminosity;
                assert!(
                    (ratio - 1.).abs() < 0.01,
                    "Expected luminosity of {} sol, got {} sol, which is off by a factor of {}",
                    expected_luminosity,
                    luminosity,
                    ratio
                );
            }
        }
    }
}
