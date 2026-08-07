// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Coupon domain models.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{commons::State, mixin::StatefulInfo, product::CouponRule};

/// A seller coupon and the interval in which it can be used.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Coupon {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// App-scoped unique coupon code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Human-readable coupon name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Application that owns this coupon.
    pub app: StatefulInfo,

    /// Persisted identifier of the owning seller.
    pub seller_id: i64,

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
