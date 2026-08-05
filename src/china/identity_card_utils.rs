// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! China administrative-data markers and resident identity-card utilities.

#[allow(unused_imports)]
use super::{
    ChinaCities,
    ChinaDistricts,
    ChinaProvinces,
};

use std::{
    collections::HashMap,
    sync::LazyLock,
};

use chrono::{
    DateTime,
    Datelike,
    NaiveDate,
    Utc,
};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    contact::District,
    person::Gender,
};

/// Utilities for 18-character Chinese resident identity-card numbers.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Serialize,
)]
pub struct IdentityCardUtils;

impl IdentityCardUtils {
    /// Total number of characters in a second-generation identity number.
    pub const NUMBER_LENGTH: usize = 18;
    /// Zero-based start of the administrative-area code.
    pub const AREA_INDEX: usize = 0;
    /// Number of digits in the administrative-area code.
    pub const AREA_LENGTH: usize = 6;
    /// Zero-based start of the birth year.
    pub const YEAR_INDEX: usize = 6;
    /// Number of digits in the birth year.
    pub const YEAR_LENGTH: usize = 4;
    /// Zero-based start of the birth month.
    pub const MONTH_INDEX: usize = 10;
    /// Number of digits in the birth month.
    pub const MONTH_LENGTH: usize = 2;
    /// Zero-based start of the birth day.
    pub const DAY_INDEX: usize = 12;
    /// Number of digits in the birth day.
    pub const DAY_LENGTH: usize = 2;
    /// Zero-based start of the personal sequence code.
    pub const SEQUENCE_INDEX: usize = 14;
    /// Number of digits in the personal sequence code.
    pub const SEQUENCE_LENGTH: usize = 3;
    /// Zero-based position of the checksum character.
    pub const VERIFY_INDEX: usize = 17;

    /// Validates the checksum and encoded calendar date of an identity number.
    ///
    /// The source implementation deliberately does not reject historical or
    /// unknown area codes.
    #[must_use]
    pub fn validate(number: &str) -> bool {
        if number.len() != Self::NUMBER_LENGTH {
            return false;
        }
        let Some(expected) = Self::get_last_char(number) else {
            return false;
        };
        number.as_bytes()[Self::VERIFY_INDEX].to_ascii_uppercase()
            == expected as u8
            && Self::get_birthday(number).is_some()
    }

    /// Calculates the checksum character from the first 17 digits.
    ///
    /// Returns `None` when fewer than 17 bytes are supplied or any of those
    /// bytes is not an ASCII digit. Bytes after the first 17 are ignored, as
    /// in the Java source.
    #[must_use]
    pub fn get_last_char(number: &str) -> Option<char> {
        const RATIOS: [u32; 17] =
            [7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2];
        const LAST_CHARS: [char; 11] =
            ['1', '0', 'X', '9', '8', '7', '6', '5', '4', '3', '2'];

        let digits = number.as_bytes().get(..Self::VERIFY_INDEX)?;
        let mut sum = 0_u32;
        for (digit, ratio) in digits.iter().zip(RATIOS) {
            if !digit.is_ascii_digit() {
                return None;
            }
            sum += u32::from(*digit - b'0') * ratio;
        }
        Some(LAST_CHARS[sum as usize % LAST_CHARS.len()])
    }

    /// Extracts the encoded birthday.
    ///
    /// Returns `None` for an incorrectly sized number, nonnumeric date
    /// components, or a date that does not exist in the proleptic Gregorian
    /// calendar.
    #[must_use]
    pub fn get_birthday(number: &str) -> Option<NaiveDate> {
        if number.len() != Self::NUMBER_LENGTH {
            return None;
        }
        let year = parse_number(
            number,
            Self::YEAR_INDEX,
            Self::YEAR_INDEX + Self::YEAR_LENGTH,
        )?;
        let month = parse_number(
            number,
            Self::MONTH_INDEX,
            Self::MONTH_INDEX + Self::MONTH_LENGTH,
        )?;
        let day = parse_number(
            number,
            Self::DAY_INDEX,
            Self::DAY_INDEX + Self::DAY_LENGTH,
        )?;
        NaiveDate::from_ymd_opt(i32::try_from(year).ok()?, month, day)
    }

    /// Extracts the gender encoded by the last sequence digit.
    ///
    /// Returns `None` for an incorrectly sized number or a nonnumeric
    /// sequence digit.
    #[must_use]
    pub fn get_gender(number: &str) -> Option<Gender> {
        if number.len() != Self::NUMBER_LENGTH {
            return None;
        }
        let digit = *number.as_bytes().get(Self::VERIFY_INDEX - 1)?;
        if !digit.is_ascii_digit() {
            return None;
        }
        Some(if (digit - b'0').is_multiple_of(2) {
            Gender::Female
        } else {
            Gender::Male
        })
    }

    /// Builds a district value from the number's administrative-area code.
    ///
    /// Returns `None` when the number is incorrectly sized or the bundled
    /// source dataset does not contain its area code. The source data does not
    /// carry persistent IDs, postal metadata, or audit timestamps, so those
    /// fields use their neutral values.
    #[must_use]
    pub fn get_district(number: &str) -> Option<District> {
        if number.len() != Self::NUMBER_LENGTH {
            return None;
        }
        let area = number
            .get(Self::AREA_INDEX..Self::AREA_INDEX + Self::AREA_LENGTH)?;
        let name = Self::get_area_map().get(area)?;
        let city_code = format!("{}00", &area[..4]);
        let city_name = Self::get_area_map()
            .get(city_code.as_str())
            .copied()
            .unwrap_or_default();
        Some(District {
            id: None,
            code: area.to_owned(),
            name: (*name).to_owned(),
            city: Info::new(None, city_code, city_name.to_owned(), None),
            level: None,
            postalcode: None,
            icon: None,
            url: None,
            description: None,
            location: None,
            predefined: true,
            create_time: DateTime::<Utc>::UNIX_EPOCH,
            modify_time: None,
            delete_time: None,
        })
    }

    /// Tests whether the number has a known six-digit area code.
    #[must_use]
    pub fn is_area_valid(number: &str) -> bool {
        if number.len() != Self::NUMBER_LENGTH {
            return false;
        }
        number
            .get(Self::AREA_INDEX..Self::AREA_INDEX + Self::AREA_LENGTH)
            .is_some_and(|area| Self::get_area_map().contains_key(area))
    }

    /// Returns the immutable GB/T 2260 area-code dataset bundled with the
    /// crate.
    #[must_use]
    pub fn get_area_map() -> &'static HashMap<&'static str, &'static str> {
        &AREA_MAP
    }

    /// Replaces the birthday and recalculates the checksum character.
    ///
    /// Returns `None` when `number` is incorrectly sized or its preserved area
    /// and sequence components do not form 17 ASCII digits.
    #[must_use]
    pub fn change_birthday(
        number: &str,
        birthday: NaiveDate,
    ) -> Option<String> {
        if number.len() != Self::NUMBER_LENGTH {
            return None;
        }
        let area = number
            .get(Self::AREA_INDEX..Self::AREA_INDEX + Self::AREA_LENGTH)?;
        let sequence = number.get(
            Self::SEQUENCE_INDEX..Self::SEQUENCE_INDEX + Self::SEQUENCE_LENGTH,
        )?;
        let mut result =
            format!("{area}{}{sequence}", get_birthday_code(birthday));
        result.push(Self::get_last_char(&result)?);
        Some(result)
    }
}

/// Parses an ASCII decimal number from the requested byte range.
fn parse_number(number: &str, start: usize, end: usize) -> Option<u32> {
    if end <= start {
        return None;
    }
    let bytes = number.as_bytes().get(start..end)?;
    let mut result = 0_u32;
    for byte in bytes {
        if !byte.is_ascii_digit() {
            return None;
        }
        result = result
            .checked_mul(10)?
            .checked_add(u32::from(*byte - b'0'))?;
    }
    Some(result)
}

/// Formats a birthday as the fixed-width `YYYYMMDD` identity-card code.
fn get_birthday_code(birthday: NaiveDate) -> String {
    format!(
        "{:04}{:02}{:02}",
        birthday.year(),
        birthday.month(),
        birthday.day()
    )
}

/// Parses the checked-in Java-compatible properties resource once.
fn load_area_map() -> HashMap<&'static str, &'static str> {
    include_str!("china-area.properties")
        .lines()
        .filter_map(|line| {
            let (code, name) = line.split_once('=')?;
            Some((code.trim(), name.trim()))
        })
        .collect()
}

static AREA_MAP: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(load_area_map);
