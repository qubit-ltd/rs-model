// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Errors produced by domain-model operations.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Broad category assigned to a platform error code.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
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
