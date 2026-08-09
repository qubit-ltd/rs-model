// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Ordered product entries belonging to marketing campaigns.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::activity::Activity;
use crate::product::Product;
use crate::product::ProductInfo;

/// A product snapshot placed at a specific position in a campaign.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct ActivityProductItem {
    /// Identifier of the owning [`Activity`].
    #[model(reference(target = Activity, target_field = id), opaque)]
    pub activity_id: Id,

    /// Zero-based position in the campaign's product collection.
    pub index: i32,

    /// Product information captured for this campaign entry.
    #[model(reference(target = Product, target_field = info))]
    pub product: ProductInfo,

    /// UTC instant, rounded to seconds, when this entry was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC soft-deletion instant, or `None` while the entry is retained.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
