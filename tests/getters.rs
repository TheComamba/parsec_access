use astro_units::mass::solar_mass;
use parsec_access::getters::{
    get_ages_in_years, get_closest_age_index, get_closest_mass_index,
    get_closest_metallicity_index_from_mass_fraction, get_masses_in_solar,
    get_metallicities_in_mass_fractions, is_data_ready,
};
use uom::si::{
    f64::{Mass, Time},
    time::year,
};

#[test]
fn metallicites_are_mapped_to_themselves() {
    assert!(is_data_ready());
    for (expected_index, expected_value) in get_metallicities_in_mass_fractions().iter().enumerate()
    {
        let index = get_closest_metallicity_index_from_mass_fraction(*expected_value);
        assert_eq!(expected_index, index);
    }
}

#[test]
fn masses_are_mapped_to_themselves() {
    assert!(is_data_ready());
    let metallicity_index = 3;
    for (expected_index, expected_value) in
        get_masses_in_solar(metallicity_index).iter().enumerate()
    {
        let mass = Mass::new::<solar_mass>(*expected_value);
        let index = get_closest_mass_index(metallicity_index, mass);
        assert_eq!(expected_index, index);
    }
}

#[test]
fn ages_are_mapped_to_themselves() {
    assert!(is_data_ready());
    let metallicity_index = 3;
    let mass_index = 30;
    for (expected_index, expected_value) in get_ages_in_years(metallicity_index, mass_index)
        .iter()
        .enumerate()
    {
        let age = Time::new::<year>(*expected_value);
        let index = get_closest_age_index(metallicity_index, mass_index, age);
        assert_eq!(expected_index, index);
    }
}
