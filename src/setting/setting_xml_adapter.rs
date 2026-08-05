// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! XML adapter for settings.

use crate::setting::{
    Setting,
    SettingAdapterError,
    SettingXmlAdapted,
};

/// Marshals settings through their XML-oriented transfer representation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SettingXmlAdapter;

impl SettingXmlAdapter {
    /// Converts an optional setting to its XML transfer value.
    #[must_use]
    pub fn marshal(setting: Option<&Setting>) -> Option<SettingXmlAdapted> {
        setting.map(SettingXmlAdapted::from_setting)
    }

    /// Converts an optional XML transfer value back into a setting.
    pub fn unmarshal(
        adapted: Option<&SettingXmlAdapted>,
    ) -> Result<Option<Setting>, SettingAdapterError> {
        adapted.map(SettingXmlAdapted::to_setting).transpose()
    }
}
