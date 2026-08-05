//! Telephone number value objects.

use std::fmt::{self, Display, Formatter};

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// A telephone number split into its source-domain country, city, and local parts.
///
/// The three components preserve the Java model's representation while the
/// `textual` capability permits mobile-format metadata on fields that use it.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Eq, Redact, Serialize)]
#[model(textual)]
pub struct Phone {
    /// Optional international dialing area without the `+` prefix.
    #[model(text(min_chars = 1, max_chars = 16, repertoire = ascii))]
    pub country_area: Option<String>,
    /// Optional city or regional dialing area.
    #[model(text(min_chars = 1, max_chars = 16, repertoire = ascii))]
    pub city_area: Option<String>,
    /// Required local subscriber number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub number: String,
}

impl Display for Phone {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        if let Some(country_area) = &self.country_area {
            write!(formatter, "+{country_area}-")?;
        }
        if let Some(city_area) = &self.city_area {
            write!(formatter, "{city_area}-")?;
        }
        formatter.write_str(&self.number)
    }
}
