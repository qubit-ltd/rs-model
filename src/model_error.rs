// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Errors returned when data cannot be accepted as a valid domain model.

use thiserror::Error;

use super::ValidationViolation;

/// Describes a failure while constructing, converting, or validating a model.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ModelError {
    /// One or more model fields violate their declared constraints.
    ///
    /// `violations` contains every detected field failure and can be empty
    /// when only a model-wide constraint failed. `message` is `None` when no
    /// caller-defined summary is available. Callers must ensure that the
    /// message and every violation omit sensitive rejected values before they
    /// are stored in this error.
    #[error("{display_message}", display_message = message.as_deref().unwrap_or("model validation failed"))]
    ValidationFailed {
        /// A caller-defined summary stored verbatim, or `None` when unavailable.
        message: Option<String>,
        /// All detected field-level violations supplied by the caller.
        violations: Vec<ValidationViolation>,
    },
}
