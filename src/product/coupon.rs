// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Coupon domain models.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::State;
use crate::mixin::StatefulInfo;
use crate::product::CouponRule;

/// A seller coupon and the interval in which it can be used.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Coupon {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// App-scoped unique coupon code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Human-readable coupon name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Application that owns this coupon.
    pub app: StatefulInfo,

    /// Persisted identifier of the owning seller.
    #[model(opaque)]
    pub seller_id: Id,

    /// Optional owning-seller name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub seller_name: Option<String>,

    /// Optional coupon image URL or key.
    #[model(text(min_chars = 1, max_chars = 512))]
    pub image: Option<String>,

    /// Optional coupon description.
    pub description: Option<String>,

    /// Calculation rule used by this coupon.
    pub rule: CouponRule,

    /// Inclusive UTC start timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub start_time: DateTime<Utc>,

    /// Optional exclusive UTC end timestamp; `None` means no expiry.
    #[model(time(precision = second, normalization = utc))]
    pub end_time: Option<DateTime<Utc>>,

    /// Coupon lifecycle state.
    pub state: State,

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
