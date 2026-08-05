// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Errors produced by domain-model operations.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Broad category assigned to a platform error code.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorType {
    /// The request itself is invalid.
    RequestError,
    /// One or more request parameters are invalid.
    ParameterError,
    /// A database operation failed.
    DatabaseError,
    /// An input/output operation failed.
    IoError,
    /// A network operation failed.
    NetworkError,
    /// An internal server operation failed.
    ServerError,
    /// A domain-logic rule was violated.
    LogicError,
    /// Authentication failed.
    AuthenticationError,
    /// Authorization failed.
    AuthorizationError,
    /// A payment operation failed.
    PaymentError,
    /// A third-party service failed.
    ThirdPartyError,
}

/// Identifies one model-field constraint violation without retaining its rejected value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationViolation {
    /// The field whose constraint failed.
    pub field: String,
    /// The constraint failure reason.
    pub reason: String,
}

/// Describes a failure encountered while constructing, converting, or validating a model.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ModelError {
    /// One or more model fields violate their declared validation constraints.
    ///
    /// `violations` may be empty, contain a single violation, or contain multiple
    /// violations. `message` supplies an optional caller-defined summary without
    /// including rejected field values.
    #[error("{display_message}", display_message = message.as_deref().unwrap_or("model validation failed"))]
    ValidationFailed {
        /// An optional summary of the validation failure.
        message: Option<String>,
        /// The individual validation failures.
        violations: Vec<ValidationViolation>,
    },
}
