// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Coupon calculation rules.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::mixin::StatefulInfo;

/// A reusable expression that determines how a coupon is calculated.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct CouponRule {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// App-scoped unique rule code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Human-readable rule name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Application that owns this rule.
    pub app: StatefulInfo,

    /// Rule expression evaluated by the coupon engine.
    pub rule: String,

    /// Optional rule description.
    pub description: Option<String>,

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
