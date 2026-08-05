// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! XML transfer representation for settings.

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use crate::setting::{
    DataType,
    Setting,
    SettingAdapterError,
    data_type_source_name,
    parse_data_type_name,
};

/// XML-oriented setting value with default-valued attributes omitted.
#[derive(
    Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(rename_all = "camelCase")]
pub struct SettingXmlAdapted {
    /// Stable setting name.
    pub name: String,
    /// Optional non-default data-type name.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// Optional non-default read-only flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
    /// Optional non-default nullable flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nullable: Option<bool>,
    /// Optional non-default multiple-value flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple: Option<bool>,
    /// Optional non-default encrypted flag.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// Optional human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional source-order setting values.
    #[redact(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

impl SettingXmlAdapted {
    /// Creates the XML transfer representation of a setting.
    #[must_use]
    pub fn from_setting(setting: &Setting) -> Self {
        Self {
            name: setting.name.clone(),
            type_name: (setting.data_type != DataType::default()).then(|| {
                data_type_source_name(setting.data_type).to_ascii_lowercase()
            }),
            readonly: (setting.readonly != Setting::DEFAULT_READONLY)
                .then_some(setting.readonly),
            nullable: (setting.nullable != Setting::DEFAULT_NULLABLE)
                .then_some(setting.nullable),
            multiple: (setting.multiple != Setting::DEFAULT_MULTIPLE)
                .then_some(setting.multiple),
            encrypted: (setting.encrypted != Setting::DEFAULT_ENCRYPTED)
                .then_some(setting.encrypted),
            description: setting.description.clone(),
            create_time: setting.create_time,
            modify_time: setting.modify_time,
            values: (!setting.values.is_empty())
                .then(|| setting.values.clone()),
        }
    }

    /// Converts the transfer representation back into a setting.
    pub fn to_setting(&self) -> Result<Setting, SettingAdapterError> {
        let data_type = self
            .type_name
            .as_deref()
            .map(|name| {
                parse_data_type_name(name).ok_or_else(|| {
                    SettingAdapterError::InvalidDataType(name.to_owned())
                })
            })
            .transpose()?
            .unwrap_or_default();
        Ok(Setting {
            name: self.name.clone(),
            data_type,
            values: self.values.clone().unwrap_or_default(),
            readonly: self.readonly.unwrap_or(Setting::DEFAULT_READONLY),
            nullable: self.nullable.unwrap_or(Setting::DEFAULT_NULLABLE),
            multiple: self.multiple.unwrap_or(Setting::DEFAULT_MULTIPLE),
            encrypted: self.encrypted.unwrap_or(Setting::DEFAULT_ENCRYPTED),
            description: self.description.clone(),
            create_time: self.create_time,
            modify_time: self.modify_time,
        })
    }
}
