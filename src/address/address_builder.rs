// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Fluent construction of hierarchical addresses.

use bigdecimal::BigDecimal;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Address;
use crate::contact::Location;

/// Incrementally assembles a postal address from administrative references and coordinates.
#[derive(Model, Redact, Clone, Default)]
#[redact(debug, display, serde)]
pub struct AddressBuilder {
    address: AddressParts,
    longitude: Option<BigDecimal>,
    latitude: Option<BigDecimal>,
}

/// Mutable values accumulated by [`AddressBuilder`].
#[derive(Model, Redact, Clone, Default)]
#[redact(debug, display, serde)]
struct AddressParts {
    #[model(opaque)]
    country: Info,

    #[model(opaque)]
    province: Info,

    #[model(opaque)]
    city: Info,

    #[model(opaque)]
    district: Info,

    #[model(opaque)]
    street: Info,
    detail: String,
    postalcode: Option<String>,
}

impl AddressBuilder {
    /// Creates an empty address builder.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the country reference identifier; `None` clears an existing identifier.
    #[must_use]
    pub fn country_id(mut self, id: Option<i64>) -> Self {
        self.address.country.id = id;
        self
    }

    /// Sets the stable code used to identify the country.
    #[must_use]
    pub fn country_code(mut self, code: &str) -> Self {
        self.address.country.code = code.to_owned();
        self
    }

    /// Sets the human-readable country name.
    #[must_use]
    pub fn country_name(mut self, name: &str) -> Self {
        self.address.country.name = name.to_owned();
        self
    }

    /// Sets the province reference identifier; `None` clears an existing identifier.
    #[must_use]
    pub fn province_id(mut self, id: Option<i64>) -> Self {
        self.address.province.id = id;
        self
    }

    /// Sets the stable code used to identify the province.
    #[must_use]
    pub fn province_code(mut self, code: &str) -> Self {
        self.address.province.code = code.to_owned();
        self
    }

    /// Sets the human-readable province name.
    #[must_use]
    pub fn province_name(mut self, name: &str) -> Self {
        self.address.province.name = name.to_owned();
        self
    }

    /// Sets the city reference identifier; `None` clears an existing identifier.
    #[must_use]
    pub fn city_id(mut self, id: Option<i64>) -> Self {
        self.address.city.id = id;
        self
    }

    /// Sets the stable code used to identify the city.
    #[must_use]
    pub fn city_code(mut self, code: &str) -> Self {
        self.address.city.code = code.to_owned();
        self
    }

    /// Sets the human-readable city name.
    #[must_use]
    pub fn city_name(mut self, name: &str) -> Self {
        self.address.city.name = name.to_owned();
        self
    }

    /// Sets the district reference identifier; `None` clears an existing identifier.
    #[must_use]
    pub fn district_id(mut self, id: Option<i64>) -> Self {
        self.address.district.id = id;
        self
    }

    /// Sets the stable code used to identify the district.
    #[must_use]
    pub fn district_code(mut self, code: &str) -> Self {
        self.address.district.code = code.to_owned();
        self
    }

    /// Sets the human-readable district name.
    #[must_use]
    pub fn district_name(mut self, name: &str) -> Self {
        self.address.district.name = name.to_owned();
        self
    }

    /// Sets the street reference identifier; `None` clears an existing identifier.
    #[must_use]
    pub fn street_id(mut self, id: Option<i64>) -> Self {
        self.address.street.id = id;
        self
    }

    /// Sets the stable code used to identify the street.
    #[must_use]
    pub fn street_code(mut self, code: &str) -> Self {
        self.address.street.code = code.to_owned();
        self
    }

    /// Sets the human-readable street name.
    #[must_use]
    pub fn street_name(mut self, name: &str) -> Self {
        self.address.street.name = name.to_owned();
        self
    }

    /// Sets the house number, building, or other address detail below street level.
    #[must_use]
    pub fn detail(mut self, detail: &str) -> Self {
        self.address.detail = detail.to_owned();
        self
    }

    /// Sets the postal code, making it present in the built address.
    #[must_use]
    pub fn postalcode(mut self, postalcode: &str) -> Self {
        self.address.postalcode = Some(postalcode.to_owned());
        self
    }

    /// Sets longitude in decimal degrees for the optional built location.
    #[must_use]
    pub fn longitude(mut self, longitude: BigDecimal) -> Self {
        self.longitude = Some(longitude);
        self
    }

    /// Sets latitude in decimal degrees for the optional built location.
    #[must_use]
    pub fn latitude(mut self, latitude: BigDecimal) -> Self {
        self.latitude = Some(latitude);
        self
    }

    /// Builds an address from the accumulated fields.
    ///
    /// A location is included only when both longitude and latitude are set; altitude and
    /// coordinate system remain absent.
    #[must_use]
    pub fn build(&self) -> Address {
        let location = match (&self.longitude, &self.latitude) {
            (Some(longitude), Some(latitude)) => Some(Location {
                longitude: longitude.clone(),
                latitude: latitude.clone(),
                altitude: None,
                coordinate_system: None,
            }),
            _ => None,
        };
        Address {
            country: self.address.country.clone(),
            province: self.address.province.clone(),
            city: self.address.city.clone(),
            district: self.address.district.clone(),
            street: self.address.street.clone(),
            detail: self.address.detail.clone(),
            postalcode: self.address.postalcode.clone(),
            location,
        }
    }
}
