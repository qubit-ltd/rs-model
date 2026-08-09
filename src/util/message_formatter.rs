// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Dependency-free formatting for indexed message templates.

/// Formats indexed templates and exposes standard localization key prefixes.
pub struct MessageFormatter;

impl MessageFormatter {
    /// Prefix for localized error-message keys.
    pub const ERROR_PREFIX: &str = "error.";
    /// Prefix for localized entity-name keys.
    pub const ENTITY_PREFIX: &str = "entity.";
    /// Prefix for localized field-name keys.
    pub const FIELD_PREFIX: &str = "field.";
    /// Prefix for localized operation-name keys.
    pub const OPERATION_PREFIX: &str = "operation.";

    /// Replaces indexed placeholders in `template` with `parameters` by index.
    ///
    /// Placeholders without a corresponding parameter remain unchanged, while
    /// parameters without a placeholder are ignored.
    #[must_use]
    pub fn format(template: &str, parameters: &[&str]) -> String {
        parameters
            .iter()
            .enumerate()
            .fold(template.to_owned(), |message, (index, parameter)| {
                message.replace(&format!("{{{index}}}"), parameter)
            })
    }
}
