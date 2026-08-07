// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Password-update parameters.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Old and new credentials supplied to a password-update operation.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct UpdatePasswordParams {
    /// Optional current password.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub old_password: Option<String>,

    /// New password.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub new_password: String,

    /// Whether the next login must require another password change.
    pub change_password: Option<bool>,

    /// Optional verification code.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub verify_code: Option<String>,
}
