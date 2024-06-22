# ParsecAccess

A Rust crate to access the [PARSEC database](https://people.sissa.it/~sbressan/CAF09_V1.2S_M36_LT/) for stellar evolutionary trajectories.

## Usage

In your `Cargo.toml`, include the `parsec_access` crate, and for almost all use cases, the `simple-si-units` crate:

```toml
[dependencies]
parsec_access = "1.0"
simple-si-units = "1.1"
```

Upon first usage, the PARSEC data is downloaded to and stored on your computer. The console output will tell you where, but you don't need to worry about that.

It is then lazily initialised, meaning it gets loaded into memory once you first try to access it. For performance reasons, the accessing functions do not validate the data. The function `is_data_ready()` fills that void. It is good practice to call it once at the beginning of the part of your code that accesses the data.

TODO: Usage example.

# Example

```Rust
use parsec_access::getters::*;
// The input for initial mass and age is typed using the simple-si-units crate.
use simple_si_units::base::Mass;
use simple_si_units::base::Time;

if !is_data_ready() {
    // In your productive code, do some graceful error handling instead.
    panic!("Loading the PARSEC data failed.");
}

// The main use-case is mapping a metallicity, initial mass and age to other physical parameters.
let metallicity_mass_fraction = 0.004;
let initial_mass = Mass::from_solar_mass(1.8);
let current_age = Time::from_Gyr(0.6);
let parameters = get_closest_parameters(metallicity_mass_fraction, initial_mass, current_age);
println!("The star has a current mass of {} solar masses.", parameters.mass.to_solar_mass());
println!("The star has a current temperature of {}.", parameters.temperature);
println!("The star has a current radius of {} km.", parameters.radius.to_km());
println!("The star has a current luminosity of {} sol.", parameters.luminosity_in_solar);

// If performance is an issue and for example your metallicity and initial mass is fixed, you can ask for the index which corresponds to your parameters and pass that on to subsequent calls.
let metallicity_index = get_closest_metallicity_index_from_mass_fraction(metallicity_mass_fraction);
let mass_index = get_closest_mass_index(metallicity_index, initial_mass);

// You can also get data structures higher up the hierarchy (by index or value).
let trajectory = get_trajectory(metallicity_index, mass_index);
println!("The star is expected to reach the proud age of {} Gyr.", trajectory.lifetime.to_Gyr());
```

## License

This software is distributed under the [MIT](https://choosealicense.com/licenses/mit/) license. In a nutshell this means that all code is made public, and you are free to use it without any charge.
