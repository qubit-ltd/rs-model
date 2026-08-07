// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Employee-binding parameters.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{contact::Phone, mixin::StatefulInfo};

/// Identifying information used to bind a user to an employee record.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
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

    /// Optional mobile number.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<Phone>,

    /// Optional email address.
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Verification code.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub verify_code: String,
}
