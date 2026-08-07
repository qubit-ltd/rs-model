// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Person-binding parameters.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{commons::CredentialInfo, contact::Phone};

/// Identifying information used to bind a user to a person record.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct BindPersonParams {
    /// Username to bind.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub username: String,

    /// Person name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional mobile number.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<Phone>,

    /// Optional email address.
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Identity credential.
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<CredentialInfo>,

    /// Verification code.
    #[model(text(min_chars = 1, max_chars = 64))]
    #[redact(level = "secret")]
    pub verify_code: String,
}
