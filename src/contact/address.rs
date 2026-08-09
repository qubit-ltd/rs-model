// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Postal-address values.

use serde::Deserialize;

use qubit_mixin::Emptyful;
use qubit_mixin::Info;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::City;
use crate::contact::Country;
use crate::contact::District;
use crate::contact::Location;
use crate::contact::Province;
use crate::contact::Street;

/// A postal address linked to each administrative level and an optional
/// location.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Address {
    /// Basic information for the country.
    #[model(reference(target = Country, target_field = info), index, opaque)]
    pub country: Info,

    /// Basic information for the province.
    #[model(reference(target = Province, target_field = info), index, opaque)]
    pub province: Info,

    /// Basic information for the city.
    #[model(reference(target = City, target_field = info), index, opaque)]
    pub city: Info,

    /// Basic information for the district.
    #[model(reference(target = District, target_field = info), index, opaque)]
    pub district: Info,

    /// Basic information for the street.
    #[model(reference(target = Street, target_field = info), index, opaque)]
    pub street: Info,

    /// Detailed street address or house number.
    #[model(index, text(min_chars = 1, max_chars = 4096))]
    #[redact(level = "secret")]
    pub detail: String,

    /// Optional ASCII postal code.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub postalcode: Option<String>,

    /// Optional geographic location for the address.
    #[model(index)]
    #[redact(skip)]
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

impl Normalizable for Address {
    fn normalize(&mut self) {
        self.country.normalize();
        self.province.normalize();
        self.city.normalize();
        self.district.normalize();
        self.street.normalize();
        self.detail.normalize();
        self.postalcode.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.country.is_empty()
            && self.province.is_empty()
            && self.city.is_empty()
            && self.district.is_empty()
            && self.street.is_empty()
            && self.detail.is_empty()
            && self.postalcode.is_none()
            && self.location.is_none()
    }
}
