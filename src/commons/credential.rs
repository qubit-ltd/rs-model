// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared records used across the migrated model domains.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use super::CredentialType;
use super::Owner;
use super::VerifyState;

/// A credential with ownership and audit fields.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[model(unique(
    name = "credential_owner_type_number",
    fields(owner, type, number),
    ignore_case(number)
))]
pub struct Credential {
    /// Platform-assigned identifier of this credential record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Credential owner.
    #[model(index)]
    pub owner: Owner,

    /// Credential classification.
    #[model(index)]
    pub r#type: CredentialType,

    /// Credential number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub number: String,

    /// Optional verification state.
    #[model(index)]
    pub verified: Option<VerifyState>,

    /// Position among the owner's credentials.
    #[model(index)]
    pub index: i32,

    /// Optional display title.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub title: Option<String>,

    /// Optional explanation of the credential's purpose or provenance.
    pub description: Option<String>,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
