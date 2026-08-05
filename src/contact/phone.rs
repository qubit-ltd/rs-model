//! Telephone number value objects.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// A telephone number split into its source-domain country, city, and local parts.
///
/// The three components preserve the Java model's representation while the
/// `textual` capability permits mobile-format metadata on fields that use it.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Eq, Serialize)]
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
    pub number: String,
}
