// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! JSON serializer for settings.

use crate::setting::Setting;

/// Serializes settings using the source model's JSON field names and omissions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SettingJsonSerializer;

impl SettingJsonSerializer {
    /// Serializes a setting into the source-compatible JSON object representation.
    ///
    /// Uses camel-case field names and emits the data type as `type`. Empty `values` are omitted;
    /// default flags remain explicit, and absent optional description and timestamp fields are
    /// emitted as JSON `null`. Returns the JSON text on success, or the underlying
    /// [`serde_json::Error`] if JSON serialization fails.
    pub fn serialize(value: &Setting) -> Result<String, serde_json::Error> {
        serde_json::to_string(value)
    }
}
