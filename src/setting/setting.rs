// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Setting values and stable setting names.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use std::cmp::Ordering;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::DataType;

/// Represents a named system setting containing zero, one, or multiple textual values.
#[derive(Model, Redact, Clone, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    /// Stable setting name.
    pub name: String,

    /// Declared value type.
    #[serde(rename = "type")]
    pub data_type: DataType,

    /// Values represented in the source model's canonical string form.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[redact(plain)]
    pub values: Vec<String>,

    /// Prevents administrative or application callers from changing this managed setting.
    pub readonly: bool,

    /// Allows the setting to be explicitly present with an empty value collection.
    pub nullable: bool,

    /// Allows a collection of values instead of requiring exactly one typed value.
    pub multiple: bool,

    /// Requires the persisted value payload to be encrypted because it contains secrets.
    pub encrypted: bool,

    /// Administrator-facing explanation of the setting's purpose and expected value.
    pub description: Option<String>,

    /// UTC instant when the setting definition was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// UTC instant when the setting definition or its values last changed.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
}

impl Setting {
    /// Default read-only state.
    pub const DEFAULT_READONLY: bool = false;
    /// Default nullable state.
    pub const DEFAULT_NULLABLE: bool = true;
    /// Default multiple-value state.
    pub const DEFAULT_MULTIPLE: bool = true;
    /// Default encrypted state.
    pub const DEFAULT_ENCRYPTED: bool = false;
    /// Source representation of a null value.
    pub const NULL_STRING: &'static str = "\\CDATA\\[null\\CDATA\\]";
    /// Separator used for multiple string values.
    pub const STRING_DELIMITER: &'static str = "§\u{200B}§§";
    /// Separator used for multiple non-string values.
    pub const STANDARD_DELIMITER: &'static str = ",";

    /// Creates an empty setting with the supplied name and data type.
    ///
    /// Initializes no values and applies the source defaults for all flags. This constructor does
    /// not validate the name or the eventual value cardinality.
    #[must_use]
    pub fn new(name: impl Into<String>, data_type: DataType) -> Self {
        Self {
            name: name.into(),
            data_type,
            values: Vec::new(),
            readonly: Self::DEFAULT_READONLY,
            nullable: Self::DEFAULT_NULLABLE,
            multiple: Self::DEFAULT_MULTIPLE,
            encrypted: Self::DEFAULT_ENCRYPTED,
            description: None,
            create_time: None,
            modify_time: None,
        }
    }

    /// Reports whether this setting satisfies its nullability and cardinality constraints.
    ///
    /// Returns `false` when a non-nullable setting has no values or a single-value setting has
    /// more than one value. It does not validate value syntax, encryption, or the setting name.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        (self.nullable || !self.values.is_empty()) && (self.multiple || self.values.len() <= 1)
    }

    /// Encodes all values into the source database representation.
    ///
    /// Returns `None` for no values, the sole value unchanged for one value, and a delimiter-joined
    /// string for multiple values. Strings use [`Self::STRING_DELIMITER`]; every other data type
    /// uses [`Self::STANDARD_DELIMITER`]. Values containing the applicable delimiter cannot be
    /// losslessly reconstructed by [`Self::set_persistent_value`].
    #[must_use]
    pub fn persistent_value(&self) -> Option<String> {
        match self.values.as_slice() {
            [] => None,
            [value] => Some(value.clone()),
            values => {
                let delimiter = if self.data_type == DataType::String {
                    Self::STRING_DELIMITER
                } else {
                    Self::STANDARD_DELIMITER
                };
                Some(values.join(delimiter))
            }
        }
    }

    /// Replaces all values from the source database representation.
    ///
    /// `None` clears the collection. Otherwise the value is split on the delimiter selected by
    /// the current [`Self::data_type`]; a value without that delimiter becomes one value, including
    /// an empty string. This method does not validate nullability, cardinality, or escaped
    /// delimiters.
    pub fn set_persistent_value(&mut self, persistent_value: Option<&str>) {
        let Some(value) = persistent_value else {
            self.values.clear();
            return;
        };
        let delimiter = if self.data_type == DataType::String {
            Self::STRING_DELIMITER
        } else {
            Self::STANDARD_DELIMITER
        };
        self.values = if value.contains(delimiter) {
            value.split(delimiter).map(str::to_owned).collect()
        } else {
            vec![value.to_owned()]
        };
    }
}

impl Default for Setting {
    fn default() -> Self {
        Self::new(String::new(), DataType::default())
    }
}

impl Ord for Setting {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}

impl PartialOrd for Setting {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Parses a case-insensitive source data-type name after trimming and converting hyphens to underscores.
pub(crate) fn parse_data_type_name(name: &str) -> Option<DataType> {
    let normalized = name.trim().replace('-', "_").to_ascii_uppercase();
    serde_json::from_value(serde_json::Value::String(normalized)).ok()
}

/// Produces the canonical uppercase source enumeration name for a data type.
pub(crate) fn data_type_source_name(data_type: DataType) -> &'static str {
    match data_type {
        DataType::Bool => "BOOL",
        DataType::Char => "CHAR",
        DataType::Byte => "BYTE",
        DataType::Short => "SHORT",
        DataType::Int => "INT",
        DataType::Long => "LONG",
        DataType::Float => "FLOAT",
        DataType::Double => "DOUBLE",
        DataType::String => "STRING",
        DataType::Date => "DATE",
        DataType::Time => "TIME",
        DataType::Datetime => "DATETIME",
        DataType::Instant => "INSTANT",
        DataType::Timestamp => "TIMESTAMP",
        DataType::ByteArray => "BYTE_ARRAY",
        DataType::Class => "CLASS",
        DataType::BigInteger => "BIG_INTEGER",
        DataType::BigDecimal => "BIG_DECIMAL",
        DataType::StringArray => "STRING_ARRAY",
        DataType::Enum => "ENUM",
        DataType::EnumArray => "ENUM_ARRAY",
    }
}
