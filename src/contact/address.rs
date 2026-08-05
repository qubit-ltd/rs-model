//! Postal-address values.

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::contact::Location;

/// A postal address linked to each administrative level and an optional location.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Address {
    /// Basic information for the country.
    #[model(opaque)]
    pub country: Info,
    /// Basic information for the province.
    #[model(opaque)]
    pub province: Info,
    /// Basic information for the city.
    #[model(opaque)]
    pub city: Info,
    /// Basic information for the district.
    #[model(opaque)]
    pub district: Info,
    /// Basic information for the street.
    #[model(opaque)]
    pub street: Info,
    /// Detailed street address or house number.
    #[model(text(min_chars = 1, max_chars = 4096))]
    pub detail: String,
    /// Optional ASCII postal code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub postalcode: Option<String>,
    /// Optional geographic location for the address.
    pub location: Option<Location>,
}

impl Address {
    /// Reports whether this and `other` identify the same street address.
    ///
    /// Equality follows the Java helper and compares detail, postal code,
    /// location, and the persisted street identifier.
    #[must_use]
    pub fn is_same(&self, other: &Self) -> bool {
        self.detail == other.detail
            && self.postalcode == other.postalcode
            && self.location == other.location
            && self.street.id == other.street.id
    }
}
