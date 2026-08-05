// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Small, dependency-free message-template formatting helpers.

/// Formats indexed message templates used by the migrated model APIs.
pub struct MessageFormatter;

impl MessageFormatter {
    /// Error-message key prefix.
    pub const ERROR_PREFIX: &str = "error.";
    /// Entity key prefix.
    pub const ENTITY_PREFIX: &str = "entity.";
    /// Field key prefix.
    pub const FIELD_PREFIX: &str = "field.";
    /// Operation key prefix.
    pub const OPERATION_PREFIX: &str = "operation.";

    /// Replaces `{0}`, `{1}`, and subsequent indexed placeholders.
    ///
    /// # Parameters
    ///
    /// * `template` - Template containing indexed placeholders.
    /// * `parameters` - Replacement text in index order.
    ///
    /// # Returns
    ///
    /// The formatted message. Unmatched placeholders remain unchanged.
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
