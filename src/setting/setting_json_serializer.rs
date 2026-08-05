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
    /// Serializes a setting into a JSON object string.
    pub fn serialize(value: &Setting) -> Result<String, serde_json::Error> {
        serde_json::to_string(value)
    }
}
