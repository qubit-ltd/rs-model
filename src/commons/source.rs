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
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

use crate::mixin::StatefulInfo;

/// Source-system metadata for an imported record.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Serialize)]
pub struct Source {
    /// Persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Importing application information.
    pub app: StatefulInfo,

    /// Entity discriminator.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,

    /// Source code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Source name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional source description.
    pub description: Option<String>,

    /// Optional category information.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,

    /// Optional provider application information.
    pub provider_app: Option<StatefulInfo>,

    /// Optional provider organization information.
    pub provider_organization: Option<StatefulInfo>,

    /// Whether this source is predefined.
    pub predefined: bool,

    /// Optional UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
