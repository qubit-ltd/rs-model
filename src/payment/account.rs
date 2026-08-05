// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment account records.

use chrono::{DateTime, Utc};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{mixin::StatefulInfo, payment::AccountType};

/// A payment account belonging to a domain owner.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Account {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Application that owns this account.
    pub app: StatefulInfo,
    /// Entity discriminator of the account owner.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub owner_type: String,
    /// Persisted identifier of the account owner.
    pub owner_id: i64,
    /// Account classification.
    pub r#type: AccountType,
    /// Display name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,
    /// Optional provider account number.
    #[model(sensitive(token), text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    #[redact(level = "secret")]
    pub number: Option<String>,
    /// Optional payment provider information.
    #[model(opaque)]
    pub provider: Option<Info>,
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
