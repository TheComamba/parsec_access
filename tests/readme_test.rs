#[test]
fn readme_test() {
    use parsec_access::getters::*;
    // The input for initial mass and age is typed using the uom crate.
    use uom::si::{length::kilometer, mass::Mass, thermodynamic_temperature::kelvin, time::Time};
    // The parsec_access crate introduces some custom units.
    use parsec_access::units::*;

    if !is_data_ready() {
        // In your productive code, do some graceful error handling instead.
        panic!("Loading the PARSEC data failed.");
    }

    // The main use-case is mapping a metallicity, initial mass and age to other physical parameters.
    let metallicity_mass_fraction = 0.004;
    let initial_mass = Mass::new::<solar>(1.8);
    let current_age = Time::new::<gigayear>(0.6);
    let parameters = get_closest_parameters(metallicity_mass_fraction, initial_mass, current_age);
    println!(
        "The star has a current mass of {} solar masses.",
        parameters.mass.get::<solar>()
    );
    println!(
        "The star has a current temperature of {}.",
        parameters.temperature.get::<kelvin>()
    );
    println!(
        "The star has a current radius of {} km.",
        parameters.radius.get::<kilometer>()
    );
    println!(
        "The star has a current luminosity of {} sol.",
        parameters.luminosity_in_solar
    );

    // If performance is an issue and for example your metallicity and initial mass is fixed, you can ask for the index which corresponds to your parameters and pass that on to subsequent calls.
    let metallicity_index =
        get_closest_metallicity_index_from_mass_fraction(metallicity_mass_fraction);
    let mass_index = get_closest_mass_index(metallicity_index, initial_mass);

    // You can also get data structures higher up the hierarchy (by index or value).
    let trajectory = get_trajectory(metallicity_index, mass_index);
    println!(
        "The star is expected to reach the proud age of {} Gyr.",
        trajectory.lifetime.get::<gigayear>()
    );
}
