// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Products participating in marketing activities.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{
    activity::Activity,
    product::{Product, ProductInfo},
};

/// One indexed product entry within an activity.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct ActivityProductItem {
    /// Identifier of the owning activity.
    #[model(reference(target = Activity, target_field = id))]
    pub activity_id: i64,

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
