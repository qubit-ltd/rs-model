// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! XML transfer representation for settings.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::setting::DataType;
use crate::setting::Setting;
use crate::setting::SettingAdapterError;
use crate::setting::data_type_source_name;
use crate::setting::parse_data_type_name;

/// XML-oriented setting value with default-valued attributes omitted.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "camelCase")]
pub struct SettingXmlAdapted {
    /// Stable setting name.
    pub name: String,

    /// XML source type name when it differs from the default string representation.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,

    /// XML override that marks this imported setting as not user-modifiable.
    pub readonly: Option<bool>,

    /// XML override allowing this imported setting to have no values.
    pub nullable: Option<bool>,

    /// XML override allowing this imported setting to contain a value list.
    pub multiple: Option<bool>,

    /// XML override requiring encryption for the imported setting values.
    pub encrypted: Option<bool>,

    /// Human-facing XML description explaining the setting's operational purpose.
    pub description: Option<String>,

    /// Source creation time retained when importing a setting definition.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// Source modification time retained when importing a setting definition.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Raw values in source order before the adapter constructs the typed setting.
    #[redact(plain)]
    pub values: Option<Vec<String>>,
}

impl SettingXmlAdapted {
    /// Creates the XML transfer representation of a setting.
    #[must_use]
    pub fn from_setting(setting: &Setting) -> Self {
        Self {
            name: setting.name.clone(),
            type_name: (setting.data_type != DataType::default())
                .then(|| data_type_source_name(setting.data_type).to_ascii_lowercase()),
            readonly: (setting.readonly != Setting::DEFAULT_READONLY).then_some(setting.readonly),
            nullable: (setting.nullable != Setting::DEFAULT_NULLABLE).then_some(setting.nullable),
            multiple: (setting.multiple != Setting::DEFAULT_MULTIPLE).then_some(setting.multiple),
            encrypted: (setting.encrypted != Setting::DEFAULT_ENCRYPTED)
                .then_some(setting.encrypted),
            description: setting.description.clone(),
            create_time: setting.create_time,
            modify_time: setting.modify_time,
            values: (!setting.values.is_empty()).then(|| setting.values.clone()),
        }
    }

    /// Converts the transfer representation back into a setting.
    pub fn to_setting(&self) -> Result<Setting, SettingAdapterError> {
        let data_type = self
            .type_name
            .as_deref()
            .map(|name| {
                parse_data_type_name(name)
                    .ok_or_else(|| SettingAdapterError::InvalidDataType(name.to_owned()))
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
