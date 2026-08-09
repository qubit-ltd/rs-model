// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Models for marketing campaigns and their availability windows.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::activity::ActivityProductItem;
use crate::commons::State;

/// A campaign owned by an application, together with its participating products.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Activity {
    /// Database identifier; the default value means the campaign has not been persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Stable, globally unique code used to identify the campaign across integrations.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64))]
    pub code: String,

    /// Display name shown for the campaign.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub name: String,

    /// Application reference that owns and administers the campaign.
    #[model(opaque)]
    pub app: Info,

    /// Products included in this campaign, in their configured display order.
    pub items: Vec<ActivityProductItem>,

    /// Additional campaign details, when supplied.
    pub description: Option<String>,

    /// UTC instant, rounded to seconds, at which the campaign becomes active.
    #[model(time(precision = second, normalization = utc))]
    pub start_time: DateTime<Utc>,

    /// UTC instant, rounded to seconds, after which the campaign is no longer active.
    #[model(time(precision = second, normalization = utc))]
    pub end_time: DateTime<Utc>,

    /// Current lifecycle state controlling whether the campaign may be used.
    #[serde(default)]
    pub state: State,

    /// UTC instant, rounded to seconds, when this record was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the latest update, or `None` if it has never been updated.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the campaign remains available.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
