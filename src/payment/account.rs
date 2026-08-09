// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Payment account records.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::mixin::StatefulInfo;
use crate::payment::AccountType;

/// A payment account belonging to a domain owner.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Account {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Application that owns this account.
    pub app: StatefulInfo,

    /// Entity discriminator of the account owner.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub owner_type: String,

    /// Persisted identifier of the account owner.
    #[model(opaque)]
    pub owner_id: Id,

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

    /// UTC instant at which this record was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the most recent update, or `None` when no update has occurred.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the record remains active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
