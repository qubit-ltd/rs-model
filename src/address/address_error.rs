// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Address-domain error codes and exceptions.

use std::collections::BTreeMap;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::contact::Phone;
use crate::error::ErrorType;

/// A stable address-module error code.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
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

/// An expected and actual mobile number do not match.
#[derive(Clone, Debug, Error, Model, PartialEq, Redact)]
#[error("mobile number mismatch for {name}")]
pub struct MismatchMobileException {
    /// Entity name or description associated with the mismatch.
    pub name: String,
    /// Expected mobile number.
    #[redact(nested)]
    pub expected_mobile: Phone,
    /// Actual mobile number.
    #[redact(nested)]
    pub actual_mobile: Phone,
}

impl MismatchMobileException {
    /// Creates a mismatch error with its template values.
    #[must_use]
    pub fn new(name: &str, expected_mobile: Phone, actual_mobile: Phone) -> Self {
        Self {
            name: name.to_owned(),
            expected_mobile,
            actual_mobile,
        }
    }

    /// Returns the stable address error code.
    #[must_use]
    pub const fn code(&self) -> AddressErrorCode {
        AddressErrorCode::MismatchMobile
    }

    /// Returns the broad logic-error category.
    #[must_use]
    pub const fn error_type(&self) -> ErrorType {
        self.code().error_type()
    }

    /// Returns all message-template parameters used by the Java exception.
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
