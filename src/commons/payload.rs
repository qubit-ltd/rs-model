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

use super::Owner;

/// A named payload belonging to an owner.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[model(unique(name = "payload_owner_key", fields(owner, key), ignore_case(key)))]
pub struct Payload {
    /// Platform-assigned identifier of this owner-scoped payload.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Payload key.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub key: String,

    /// Optional payload value.
    #[model(text(min_chars = 1, max_chars = 256))]
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
