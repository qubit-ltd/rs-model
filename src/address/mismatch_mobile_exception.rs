// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Structured error for mismatched mobile-number checks.

use std::collections::BTreeMap;
use thiserror::Error;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::AddressErrorCode;
use crate::contact::Phone;
use crate::error::ErrorType;

/// A mobile-number validation failure that preserves both compared values.
#[derive(Model, Redact, Clone, Error, PartialEq)]
#[redact(debug, serde)]
#[error("mobile number mismatch for {name}")]
pub struct MismatchMobileException {
    /// Name or description of the subject whose mobile number was checked.
    pub name: String,

    /// Mobile number required by the business rule.
    #[redact(nested)]
    pub expected_mobile: Phone,

    /// Mobile number supplied by the caller or external system.
    #[redact(nested)]
    pub actual_mobile: Phone,
}

impl MismatchMobileException {
    /// Creates the error for a named subject and the two values that failed comparison.
    #[must_use]
    pub fn new(name: &str, expected_mobile: Phone, actual_mobile: Phone) -> Self {
        Self {
            name: name.to_owned(),
            expected_mobile,
            actual_mobile,
        }
    }

    /// Returns the machine-readable code for this validation boundary.
    #[must_use]
    pub const fn code(&self) -> AddressErrorCode {
        AddressErrorCode::MismatchMobile
    }

    /// Returns the broad platform category for this business-rule error.
    #[must_use]
    pub const fn error_type(&self) -> ErrorType {
        self.code().error_type()
    }

    /// Returns message-template values under both legacy and explicit parameter names.
    #[must_use]
    pub fn parameters(&self) -> BTreeMap<&'static str, String> {
        let expected = self.expected_mobile.to_string();
        let actual = self.actual_mobile.to_string();
        BTreeMap::from([
            ("name", self.name.clone()),
            ("expected_mobile", expected.clone()),
            ("actual_mobile", actual.clone()),
            ("expected", expected),
            ("actual", actual),
        ])
    }
}
