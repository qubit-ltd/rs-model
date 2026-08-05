//! Administrative-region model values.

use chrono::{DateTime, Utc};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::contact::Location;

/// A country in the administrative hierarchy.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Country {
    /// Optional persisted identifier.
    pub id: Option<i64>,
    /// Globally unique ASCII country code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Globally unique country name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Telephone area code.
    #[model(text(min_chars = 1, max_chars = 16, repertoire = ascii))]
    pub phone_area: String,
    /// Optional ASCII postal code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub postalcode: Option<String>,
    /// Optional ASCII icon URI.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,
    /// Optional ASCII web URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Whether this is predefined reference data.
    pub predefined: bool,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

/// A province in the administrative hierarchy.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Province {
    /// Optional persisted identifier.
    pub id: Option<i64>,
    /// Globally unique ASCII province code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Globally unique province name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Basic information for the country.
    #[model(opaque)]
    pub country: Info,
    /// Optional ASCII postal code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub postalcode: Option<String>,
    /// Optional administrative level.
    pub level: Option<i32>,
    /// Optional ASCII icon URI.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,
    /// Optional ASCII web URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Whether this is predefined reference data.
    pub predefined: bool,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

/// A city in the administrative hierarchy.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct City {
    /// Optional persisted identifier.
    pub id: Option<i64>,
    /// Globally unique ASCII city code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Globally unique city name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Basic information for the province.
    #[model(opaque)]
    pub province: Info,
    /// Optional telephone area code.
    #[model(text(min_chars = 1, max_chars = 16, repertoire = ascii))]
    pub phone_area: Option<String>,
    /// Optional ASCII postal code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub postalcode: Option<String>,
    /// Optional administrative level.
    pub level: Option<i32>,
    /// Optional ASCII icon URI.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,
    /// Optional ASCII web URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Optional geographic location.
    pub location: Option<Location>,
    /// Whether this is predefined reference data.
    pub predefined: bool,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

/// A district in the administrative hierarchy.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct District {
    /// Optional persisted identifier.
    pub id: Option<i64>,
    /// Globally unique ASCII district code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// District name, unique within its city.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Basic information for the city.
    #[model(opaque)]
    pub city: Info,
    /// Optional administrative level.
    pub level: Option<i32>,
    /// Optional ASCII postal code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub postalcode: Option<String>,
    /// Optional ASCII icon URI.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,
    /// Optional ASCII web URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Optional geographic location.
    pub location: Option<Location>,
    /// Whether this is predefined reference data.
    pub predefined: bool,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

/// A street in the administrative hierarchy.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Street {
    /// Optional persisted identifier.
    pub id: Option<i64>,
    /// Globally unique ASCII street code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Street name, unique within its district.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Basic information for the district.
    #[model(opaque)]
    pub district: Info,
    /// Optional administrative level.
    pub level: Option<i32>,
    /// Optional ASCII postal code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub postalcode: Option<String>,
    /// Optional ASCII icon URI.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,
    /// Optional ASCII web URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,
    /// Optional description.
    pub description: Option<String>,
    /// Optional geographic location.
    pub location: Option<Location>,
    /// Whether this is predefined reference data.
    pub predefined: bool,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
