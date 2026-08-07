// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Products participating in marketing activities.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::activity::Activity;
use crate::product::Product;
use crate::product::ProductInfo;

/// One indexed product entry within an activity.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct ActivityProductItem {
    /// Identifier of the owning activity.
    #[model(reference(target = Activity, target_field = id), opaque)]
    pub activity_id: Id,

    /// Zero-based position within the activity's product list.
    pub index: i32,

    /// Product snapshot.
    #[model(reference(target = Product, target_field = info))]
    pub product: ProductInfo,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
