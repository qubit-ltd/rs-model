// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Seedable setting fixture generator.

use std::time::{
    SystemTime,
    UNIX_EPOCH,
};

use chrono::{
    DateTime,
    Utc,
};

use crate::setting::{
    DataType,
    Setting,
};

/// Generates valid settings across the source model's supported scalar types.
#[derive(Clone, Debug)]
pub struct SettingRandomizer {
    /// Internal xorshift state.
    state: u64,
    /// Inclusive range for generated value counts.
    collection_size_range: (usize, usize),
    /// Inclusive range for generated string lengths.
    string_length_range: (usize, usize),
}

impl SettingRandomizer {
    /// Data types supported by the source randomizer.
    pub const SUPPORTED_TYPES: [DataType; 15] = [
        DataType::Bool,
        DataType::Char,
        DataType::Byte,
        DataType::Short,
        DataType::Int,
        DataType::Long,
        DataType::Float,
        DataType::Double,
        DataType::String,
        DataType::Date,
        DataType::Time,
        DataType::Datetime,
        DataType::Instant,
        DataType::BigInteger,
        DataType::BigDecimal,
    ];

    /// Creates a deterministic randomizer from a seed.
    #[must_use]
    pub const fn with_seed(seed: u64) -> Self {
        Self {
            state: if seed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                seed
            },
            collection_size_range: (1, 3),
            string_length_range: (5, 15),
        }
    }

    /// Sets the inclusive range used for generated value counts.
    ///
    /// # Panics
    ///
    /// Panics when `minimum` is greater than `maximum`.
    pub fn set_collection_size_range(
        &mut self,
        minimum: usize,
        maximum: usize,
    ) {
        assert!(
            minimum <= maximum,
            "minimum collection size exceeds maximum"
        );
        self.collection_size_range = (minimum, maximum);
    }

    /// Sets the inclusive range used for generated text lengths.
    ///
    /// # Panics
    ///
    /// Panics when the range is empty or reversed.
    pub fn set_string_length_range(&mut self, minimum: usize, maximum: usize) {
        assert!(minimum > 0, "minimum string length must be positive");
        assert!(minimum <= maximum, "minimum string length exceeds maximum");
        self.string_length_range = (minimum, maximum);
    }

    /// Generates the next valid setting.
    pub fn get(&mut self) -> Setting {
        let name_length = self.next_range(self.string_length_range);
        let random_name = self.random_text(name_length);
        let type_index = self.next_range((0, Self::SUPPORTED_TYPES.len() - 1));
        let data_type = Self::SUPPORTED_TYPES[type_index];
        let value_count = self.next_range(self.collection_size_range);
        let values = (0..value_count)
            .map(|_| self.random_value(data_type))
            .collect();
        let readonly = self.next_bool();
        let nullable = value_count == 0 || self.next_bool();
        let encrypted = self.next_bool();
        let description = self
            .next_bool()
            .then(|| format!("Description for setting_{random_name}"));
        let create_time = Some(self.random_timestamp());
        let modify_time = self.next_bool().then(|| self.random_timestamp());
        Setting {
            name: format!("setting_{random_name}"),
            data_type,
            values,
            readonly,
            nullable,
            multiple: value_count > 1,
            encrypted,
            description,
            create_time,
            modify_time,
        }
    }

    /// Advances the xorshift generator.
    fn next_u64(&mut self) -> u64 {
        let mut value = self.state;
        value ^= value << 13;
        value ^= value >> 7;
        value ^= value << 17;
        self.state = value;
        value
    }

    /// Generates a boolean.
    fn next_bool(&mut self) -> bool {
        self.next_u64() & 1 == 1
    }

    /// Generates a value in an inclusive range.
    fn next_range(&mut self, range: (usize, usize)) -> usize {
        let width = range.1 - range.0 + 1;
        range.0 + self.next_u64() as usize % width
    }

    /// Generates lowercase ASCII fixture text.
    fn random_text(&mut self, length: usize) -> String {
        (0..length)
            .map(|_| (b'a' + self.next_range((0, 25)) as u8) as char)
            .collect()
    }

    /// Generates one canonical textual value for a data type.
    fn random_value(&mut self, data_type: DataType) -> String {
        match data_type {
            DataType::Bool => self.next_bool().to_string(),
            DataType::Char => self.random_text(1),
            DataType::Byte => (self.next_u64() as i8).to_string(),
            DataType::Short => (self.next_u64() as i16).to_string(),
            DataType::Int => (self.next_u64() as i32).to_string(),
            DataType::Long | DataType::BigInteger => {
                (self.next_u64() as i64).to_string()
            }
            DataType::Float => {
                format!("{:.3}", self.next_u64() as f32 / 1_000.0)
            }
            DataType::Double => {
                format!("{:.6}", self.next_u64() as f64 / 1_000_000.0)
            }
            DataType::String => {
                let length = self.next_range(self.string_length_range);
                self.random_text(length)
            }
            DataType::Date => format!(
                "{:04}-{:02}-{:02}",
                2020 + self.next_range((0, 9)),
                self.next_range((1, 12)),
                self.next_range((1, 28))
            ),
            DataType::Time => format!(
                "{:02}:{:02}:{:02}",
                self.next_range((0, 23)),
                self.next_range((0, 59)),
                self.next_range((0, 59))
            ),
            DataType::Datetime => format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}",
                2020 + self.next_range((0, 9)),
                self.next_range((1, 12)),
                self.next_range((1, 28)),
                self.next_range((0, 23)),
                self.next_range((0, 59)),
                self.next_range((0, 59))
            ),
            DataType::Instant => self.random_timestamp().to_rfc3339(),
            DataType::BigDecimal => {
                format!(
                    "{}.{:06}",
                    self.next_u64(),
                    self.next_u64() % 1_000_000
                )
            }
            DataType::Timestamp
            | DataType::ByteArray
            | DataType::Class
            | DataType::StringArray
            | DataType::Enum
            | DataType::EnumArray => {
                unreachable!("unsupported randomizer data type")
            }
        }
    }

    /// Generates a stable timestamp for deterministic seeded output.
    fn random_timestamp(&mut self) -> DateTime<Utc> {
        let seconds =
            1_700_000_000 + (self.next_u64() % (10 * 365 * 86_400)) as i64;
        DateTime::from_timestamp(seconds, 0)
            .expect("generated timestamp is in range")
    }
}

impl Default for SettingRandomizer {
    fn default() -> Self {
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0x9E37_79B9_7F4A_7C15, |duration| {
                duration.as_nanos() as u64
            });
        Self::with_seed(seed)
    }
}
