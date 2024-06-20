use simple_si_units::base::{Distance, Luminosity, Mass, Temperature, Time};

use crate::error::ParsecAccessError;

pub(super) struct RawParsecLine {
    mass: f64,
    age: f64,
    log_l: f64,
    log_te: f64,
    log_r: f64,
}

#[derive(Clone)]
pub struct ParsecLine {
    pub mass: Mass<f64>,
    pub age: Time<f64>,
    pub luminous_intensity: Luminosity<f64>,
    pub temperature: Temperature<f64>,
    pub radius: Distance<f64>,
}

impl ParsecLine {
    const MASS_INDEX: usize = 1;
    const AGE_INDEX: usize = 2;
    const LOG_L_INDEX: usize = 3;
    const LOG_TE_INDEX: usize = 4;
    const LOG_R_INDEX: usize = 5;

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
        const SOLAR_LUMINOUS_INTENSITY: Luminosity<f64> = Luminosity { cd: 2.98e27 };

        ParsecLine {
            mass: Mass::from_solar_mass(self.mass),
            age: Time::from_yr(self.age),
            luminous_intensity: 10f64.powf(self.log_l) * SOLAR_LUMINOUS_INTENSITY,
            temperature: Temperature::from_K(10f64.powf(self.log_te)),
            radius: Distance::from_cm(10f64.powf(self.log_r)),
        }
    }
}
