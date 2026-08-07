// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Marketing activity records.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::activity::ActivityProductItem;
use crate::commons::State;

/// A marketing activity and the products participating in it.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[model(unique(fields(code)))]
pub struct Activity {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique stable code.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub code: String,

    /// Activity name.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub name: String,

    /// Owning application.
    #[model(opaque)]
    pub app: Info,

    /// Products participating in the activity.
    pub items: Vec<ActivityProductItem>,

    /// Optional description.
    pub description: Option<String>,

    /// UTC activity start timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub start_time: DateTime<Utc>,

    /// UTC activity end timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub end_time: DateTime<Utc>,

    /// Lifecycle state.
    #[serde(default)]
    pub state: State,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
