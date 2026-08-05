// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! String codec for telephone numbers.

use crate::contact::{ContactCodecError, Phone};

/// Converts telephone numbers to and from their Java-compatible textual form.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhoneCodec;

impl PhoneCodec {
    /// Decodes an optional telephone number, treating blank input as absent.
    pub fn decode(source: Option<&str>) -> Result<Option<Phone>, ContactCodecError> {
        let Some(source) = source.map(str::trim).filter(|value| !value.is_empty()) else {
            return Ok(None);
        };
        let parts: Vec<&str> = source.split('-').collect();
        let phone = match parts.as_slice() {
            [number] => Phone::from(*number),
            [first, number] if first.starts_with('+') => Phone {
                country_area: Some(first.trim_start_matches('+').to_owned()),
                city_area: None,
                number: (*number).to_owned(),
            },
            [city_area, number] => Phone {
                country_area: None,
                city_area: Some((*city_area).to_owned()),
                number: (*number).to_owned(),
            },
            [country_area, city_area, number] if country_area.starts_with('+') => Phone {
                country_area: Some(country_area.trim_start_matches('+').to_owned()),
                city_area: Some((*city_area).to_owned()),
                number: (*number).to_owned(),
            },
            _ => return Err(ContactCodecError::InvalidPhone),
        };
        Ok(Some(phone))
    }

    /// Decodes a telephone number and returns absence if decoding fails.
    #[must_use]
    pub fn decode_without_error(source: Option<&str>) -> Option<Phone> {
        Self::decode(source).ok().flatten()
    }

    /// Encodes an optional telephone number.
    #[must_use]
    pub fn encode(phone: Option<&Phone>) -> Option<String> {
        phone.map(ToString::to_string)
    }
}
