// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Stable error classification for address-domain validation failures.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::error::ErrorType;

/// Machine-readable code for a business-rule failure in address processing.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AddressErrorCode {
    /// The supplied mobile number differs from the number expected for the named subject.
    MismatchMobile,
}

impl AddressErrorCode {
    /// Returns the platform category used to report this error code.
    #[must_use]
    pub const fn error_type(self) -> ErrorType {
        ErrorType::LogicError
    }
}
