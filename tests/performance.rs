use parsec_access::{
    getters::{get_closest_parameters, get_parameters, is_data_ready},
    units::solar,
};
use serial_test::serial;
use uom::{
    fmt::DisplayStyle,
    si::{
        f64::{Mass, Time},
        mass::kilogram,
        time::year,
    },
};

const N: usize = 1e6 as usize;

#[test]
#[serial]
fn get_parameters_is_fast() {
    const PRIME1: usize = 1009;
    const PRIME2: usize = 1013;
    const PRIME3: usize = 10007;
    const MAX_METALLICITY_INDEX: usize = 10;
    const MAX_MASS_INDEX: usize = 50;
    const MAX_AGE_INDEX: usize = 100;

    assert!(is_data_ready());

    // Create pseudo-random indices.
    let mut indices = Vec::new();
    for i in 0..N {
        let metallicity_index = (i * PRIME1) % MAX_METALLICITY_INDEX;
        let mass_index = (i * PRIME2) % MAX_MASS_INDEX;
        let age_index = (i * PRIME3) % MAX_AGE_INDEX;
        indices.push((metallicity_index, mass_index, age_index));
    }

    // Access the data in a pseudo-random order.
    let now = std::time::Instant::now();
    let mut total_mass = Mass::new::<kilogram>(0.);
    for (metallicity_index, mass_index, age_index) in indices {
        let m = get_parameters(metallicity_index, mass_index, age_index).mass;
        total_mass += m;
    }
    let elapsed = now.elapsed();
    println!(
        "Collected a total mass of {} solar masses.",
        total_mass.into_format_args(solar, DisplayStyle::Abbreviation)
    );

    println!(
        "Accessing {} data points took {:?}, or {:?} per access",
        N,
        elapsed,
        elapsed / (N as u32)
    );
}

#[test]
#[serial]
fn get_closest_parameters_is_reasonably_fast() {
    const PRIME1: usize = 10037;
    const PRIME2: usize = 10039;
    const PRIME3: usize = 10061;
    const GRANULARITY: usize = 1000;
    const MAX_METALLICITY: f64 = 0.09;
    let max_mass = Mass::new::<solar>(370.);
    let max_age = Time::new::<year>(15.0e9);

    assert!(is_data_ready());

    // Create pseudo-random indices.
    let mut params = Vec::new();
    for i in 0..N {
        let metallicity_index = (i * PRIME1) % GRANULARITY;
        let metallicity = (metallicity_index as f64) / (GRANULARITY as f64) * MAX_METALLICITY;
        let mass_index = (i * PRIME2) % GRANULARITY;
        let mass = (mass_index as f64) / (GRANULARITY as f64) * max_mass;
        let age_index = (i * PRIME3) % GRANULARITY;
        let age = (age_index as f64) / (GRANULARITY as f64) * max_age;
        params.push((metallicity, mass, age));
    }

    // Access the data in a pseudo-random order.
    let now = std::time::Instant::now();
    let mut total_mass = Mass::new::<kilogram>(0.);
    for (metallicity, mass, age) in params {
        let m = get_closest_parameters(metallicity, mass, age).mass;
        total_mass += m;
    }
    let elapsed = now.elapsed();
    println!(
        "Collected a total mass of {} solar masses.",
        total_mass.into_format_args(solar, DisplayStyle::Abbreviation)
    );

    println!(
        "Accessing {} data points took {:?}, or {:?} per access",
        N,
        elapsed,
        elapsed / (N as u32)
    );
}
