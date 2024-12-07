//! Contains the `ParsecLine` struct, which holds the PARSEC data for a given metallicity, initial mass and age.

use quantity::{Length, Mass, Temperature, Time, KELVIN};

use crate::error::ParsecAccessError;

pub(super) struct RawParsecLine {
    mass: f64,
    age: f64,
    log_l: f64,
    log_te: f64,
    log_r: f64,
}

/// The data struct holding the PARSEC data for a given metallicity, initial mass and age.
/// This struct cannot be created directly, but can only be read and accessed through the crate api.
#[derive(Clone)]
pub struct ParsecLine {
    /// The current mass of the star.
    pub mass: Mass,
    /// The current age of the star.
    pub age: Time,
    /// The luminosity of the star in units solar luminosities.
    pub luminosity_in_solar: f64,
    /// The current effective temperature of the star.
    pub temperature: Temperature,
    /// The current radius of the star.
    pub radius: Length,
}

impl ParsecLine {
    const MASS_INDEX: usize = 1;
    const AGE_INDEX: usize = 2;
    const LOG_L_INDEX: usize = 3;
    const LOG_TE_INDEX: usize = 4;
    const LOG_R_INDEX: usize = 5;
    pub(crate) const LARGEST_REQUIRED_INDEX: usize = 5;

    pub(super) fn read(line: String) -> Result<Self, ParsecAccessError> {
        let entries: Vec<&str> = line.split_whitespace().collect();
        let mass_entry = entries
            .get(Self::MASS_INDEX)
            .ok_or(ParsecAccessError::DataNotAvailable("mass".to_string()))?;

        let age_entry = entries
            .get(Self::AGE_INDEX)
            .ok_or(ParsecAccessError::DataNotAvailable("age".to_string()))?;
        let log_l_entry = entries
            .get(Self::LOG_L_INDEX)
            .ok_or(ParsecAccessError::DataNotAvailable("log_l".to_string()))?;
        let log_te_entry = entries
            .get(Self::LOG_TE_INDEX)
            .ok_or(ParsecAccessError::DataNotAvailable("log_te".to_string()))?;
        let log_r_entry = entries
            .get(Self::LOG_R_INDEX)
            .ok_or(ParsecAccessError::DataNotAvailable("log_r".to_string()))?;
        if let (Ok(mass), Ok(age), Ok(log_l), Ok(log_te), Ok(log_r)) = (
            mass_entry.parse::<f64>(),
            age_entry.parse::<f64>(),
            log_l_entry.parse::<f64>(),
            log_te_entry.parse::<f64>(),
            log_r_entry.parse::<f64>(),
        ) {
            let parsec_line = RawParsecLine {
                mass,
                age,
                log_l,
                log_te,
                log_r,
            }
            .parse();

            Ok(parsec_line)
        } else {
            Err(ParsecAccessError::DataNotAvailable(
                "[Parsing failed]".to_string(),
            ))
        }
    }
}

impl RawParsecLine {
    fn parse(self) -> ParsecLine {
        ParsecLine {
            mass: self.mass,
            age: self.age,
            luminosity_in_solar: 10f64.powf(self.log_l),
            temperature: 10f64.powf(self.log_te) * KELVIN,
            radius: Distance::from_cm(10f64.powf(self.log_r)),
        }
    }
}
