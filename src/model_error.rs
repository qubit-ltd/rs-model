// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Errors produced by domain-model operations.

use thiserror::Error;

use super::ValidationViolation;

/// Describes a failure encountered while constructing, converting, or
/// validating a model.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ModelError {
    /// One or more model fields violate their declared validation constraints.
    ///
    /// `violations` may be empty, contain a single violation, or contain
    /// multiple violations. `message` supplies an optional caller-defined
    /// summary without including rejected field values.
    #[error("{display_message}", display_message = message.as_deref().unwrap_or("model validation failed"))]
    ValidationFailed {
        /// An optional summary of the validation failure.
        message: Option<String>,
        /// The individual validation failures.
        violations: Vec<ValidationViolation>,
    },
}
