// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared records used across the migrated model domains.

#[allow(unused_imports)]
use super::{
    AuthorizeRecord, Category, CredentialInfo, CredentialType, Currency, DayType, Kinship, MqType,
    Owner, Owners, Payload, RequestStatus, Source, VerifyState,
};

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// A credential with ownership and audit fields.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Credential {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Credential owner.
    pub owner: Owner,

    /// Credential classification.
    pub r#type: CredentialType,

    /// Credential number.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub number: String,

    /// Optional verification state.
    pub verified: Option<VerifyState>,

    /// Position among the owner's credentials.
    pub index: i32,

    /// Optional display title.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub title: Option<String>,

    /// Optional description.
    pub description: Option<String>,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
