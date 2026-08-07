// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Address-domain error codes and exceptions.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::error::ErrorType;

/// A stable address-module error code.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AddressErrorCode {
    /// An actual mobile number differs from the expected number.
    MismatchMobile,
}

impl AddressErrorCode {
    /// Returns the broad platform error category.
    #[must_use]
    pub const fn error_type(self) -> ErrorType {
        ErrorType::LogicError
    }
}
