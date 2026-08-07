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
    AuthorizeRecord, Credential, CredentialInfo, CredentialType, Currency, DayType, Kinship,
    MqType, Owner, Owners, Payload, RequestStatus, Source, VerifyState,
};

use chrono::{DateTime, Utc};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// A common category record.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Eq, Serialize)]
pub struct Category {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Entity discriminator.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,

    /// Globally unique category code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Category name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional icon URL or key.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,

    /// Optional description.
    pub description: Option<String>,

    /// Query-computed display title.
    pub title: Option<String>,

    /// Optional parent category information.
    #[model(opaque)]
    pub parent: Option<InfoWithEntity>,

    /// Whether this record is predefined.
    pub predefined: bool,

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
