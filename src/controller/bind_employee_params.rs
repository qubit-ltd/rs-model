// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Employee-binding parameters.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::contact::Phone;
use crate::mixin::StatefulInfo;

/// Identifying information used to bind a user to an employee record.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct BindEmployeeParams {
    /// Username to bind.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub username: String,

    /// Employee name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Employer information.
    pub organization: StatefulInfo,

    /// Mobile contact channel to associate with the employee record.
    #[redact(nested)]
    pub mobile: Option<Phone>,

    /// Email contact channel to associate with the employee record.
    #[redact(level = "secret")]
    pub email: Option<String>,

    /// Verification code.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub verify_code: String,
}
