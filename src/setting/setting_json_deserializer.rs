// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! JSON deserializer for settings.

use chrono::DateTime;
use chrono::Utc;
use serde_json::Map;
use serde_json::Value;

use crate::setting::DataType;
use crate::setting::Setting;
use crate::setting::SettingAdapterError;
use crate::setting::parse_data_type_name;

/// Deserializes settings while applying the Java model's defaults.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SettingJsonDeserializer;

impl SettingJsonDeserializer {
    /// Deserializes a setting from a JSON object string.
    pub fn deserialize(source: &str) -> Result<Setting, SettingAdapterError> {
        let value: Value =
            serde_json::from_str(source).map_err(SettingAdapterError::InvalidJson)?;
        let object = value
            .as_object()
            .ok_or(SettingAdapterError::InvalidJsonRoot)?;
        let data_type = data_type(object)?;
        Ok(Setting {
            name: string_field(object, "name").unwrap_or_default(),
            data_type,
            values: values_field(object),
            readonly: bool_field(object, "readonly", Setting::DEFAULT_READONLY),
            nullable: bool_field(object, "nullable", Setting::DEFAULT_NULLABLE),
            multiple: bool_field(object, "multiple", Setting::DEFAULT_MULTIPLE),
            encrypted: bool_field(object, "encrypted", Setting::DEFAULT_ENCRYPTED),
            description: string_field(object, "description"),
            create_time: timestamp_field(object, "createTime")?,
            modify_time: timestamp_field(object, "modifyTime")?,
        })
    }
}

/// Reads the setting data type or applies the source default.
fn data_type(object: &Map<String, Value>) -> Result<DataType, SettingAdapterError> {
    let Some(name) = object.get("type").and_then(Value::as_str) else {
        return Ok(DataType::default());
    };
    parse_data_type_name(name).ok_or_else(|| SettingAdapterError::InvalidDataType(name.to_owned()))
}

/// Reads an optional string field.
fn string_field(object: &Map<String, Value>, name: &str) -> Option<String> {
    object.get(name).and_then(Value::as_str).map(str::to_owned)
}

/// Reads a boolean field or its source default.
fn bool_field(object: &Map<String, Value>, name: &str, default: bool) -> bool {
    object.get(name).and_then(Value::as_bool).unwrap_or(default)
}

/// Reads non-null array members using their textual representation.
fn values_field(object: &Map<String, Value>) -> Vec<String> {
    object
        .get("values")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|value| !value.is_null())
        .map(|value| match value {
            Value::String(value) => value.clone(),
            value => value.to_string(),
        })
        .collect()
}

/// Reads an optional ISO-8601 timestamp.
fn timestamp_field(
    object: &Map<String, Value>,
    name: &str,
) -> Result<Option<DateTime<Utc>>, SettingAdapterError> {
    object
        .get(name)
        .and_then(Value::as_str)
        .map(|value| {
            DateTime::parse_from_rfc3339(value)
                .map(|value| value.with_timezone(&Utc))
                .map_err(SettingAdapterError::InvalidTimestamp)
        })
        .transpose()
}
