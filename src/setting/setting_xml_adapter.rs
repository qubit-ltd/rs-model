// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! XML adapter for settings.

use crate::setting::Setting;
use crate::setting::SettingAdapterError;
use crate::setting::SettingXmlAdapted;

/// Marshals settings through their XML-oriented transfer representation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct SettingXmlAdapter;

impl SettingXmlAdapter {
    /// Converts an optional setting to its XML transfer value.
    ///
    /// Returns `None` for `None`. For a setting, omits default type and flag values and omits an
    /// empty value list; no validation or fallible conversion is performed.
    #[must_use]
    pub fn marshal(setting: Option<&Setting>) -> Option<SettingXmlAdapted> {
        setting.map(SettingXmlAdapted::from_setting)
    }

    /// Converts an optional XML transfer value back into a setting.
    ///
    /// Returns `Ok(None)` for `None`. For `Some`, missing XML type and flags use [`Setting`]'s
    /// defaults and missing values become an empty vector. Returns
    /// [`SettingAdapterError::InvalidDataType`] when the supplied type name is unknown.
    pub fn unmarshal(
        adapted: Option<&SettingXmlAdapted>,
    ) -> Result<Option<Setting>, SettingAdapterError> {
        adapted.map(SettingXmlAdapted::to_setting).transpose()
    }
}
