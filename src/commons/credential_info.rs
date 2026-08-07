// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared records used across the migrated model domains.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::CredentialType;
use super::VerifyState;

/// Lightweight credential information.
#[derive(
    Clone, Debug, Deserialize, Model, PartialEq, Eq, Redact, Serialize,
)]
pub struct CredentialInfo {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Credential classification.
    pub r#type: CredentialType,

    /// Credential number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    #[redact(level = "secret")]
    pub number: String,

    /// Optional verification state.
    pub verified: Option<VerifyState>,
}

impl CredentialInfo {
    /// Reports whether another credential has the same type and number.
    #[must_use]
    pub fn is_same(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.number == other.number
    }
}
