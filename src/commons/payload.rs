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
    AuthorizeRecord,
    Category,
    Credential,
    CredentialInfo,
    CredentialType,
    Currency,
    DayType,
    Kinship,
    MqType,
    Owner,
    Owners,
    RequestStatus,
    Source,
    VerifyState,
};

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// A named payload belonging to an owner.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Payload {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Payload key.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub key: String,
    /// Optional payload value.
    pub value: Option<String>,
    /// Payload owner.
    pub owner: Owner,
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
