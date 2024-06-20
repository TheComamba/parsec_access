#[cfg(test)]
mod tests {
    use parsec_access::getters::{
        get_masses_in_solar, get_metallicities_in_mass_fractions, get_trajectory,
    };

    #[test]
    fn lifetime_mostly_decreases_with_mass() {
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
                    lifetime,
                    mass_index - 1,
                    previous_lifetime
                );
            }
        }
    }
}
