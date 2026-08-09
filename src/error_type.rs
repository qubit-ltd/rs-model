// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Platform error categories used to classify failed operations.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Classifies the subsystem or boundary responsible for a platform error.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorType {
    /// The request cannot be processed as submitted.
    RequestError,
    /// One or more supplied parameters violate their contract.
    ParameterError,
    /// Persistent storage could not complete the operation.
    DatabaseError,
    /// An input or output operation failed.
    IoError,
    /// Communication with a network resource failed.
    NetworkError,
    /// The server encountered an unexpected internal failure.
    ServerError,
    /// The requested action violates a domain rule.
    LogicError,
    /// The caller could not be authenticated.
    AuthenticationError,
    /// The authenticated caller lacks permission for the action.
    AuthorizationError,
    /// A payment workflow could not complete.
    PaymentError,
    /// A required external service failed or rejected the request.
    ThirdPartyError,
}
