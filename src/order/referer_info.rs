// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Referral identity records.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::order::OpenidType;

/// A referral identity linked to another domain entity.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct RefererInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Referral identifier namespace.
    pub openid_type: OpenidType,
    /// Referral open identifier.
    #[redact(level = "secret")]
    pub openid: String,
    /// Entity discriminator of the referring object.
    pub referer_type: String,
    /// Persisted identifier of the referring object.
    pub referer_id: i64,
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
