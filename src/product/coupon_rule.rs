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
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::App;
use crate::mixin::StatefulInfo;

/// A reusable expression that determines how a coupon is calculated.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[model(unique(name = "coupon_rule_app_code", fields(app, code), ignore_case(code)))]
pub struct CouponRule {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// App-scoped unique rule code.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub code: String,

    /// Human-readable rule name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Application that owns this rule.
    #[model(reference(target = App, target_field = info))]
    pub app: StatefulInfo,

    /// Rule expression evaluated by the coupon engine.
    pub rule: String,

    /// Optional rule description.
    pub description: Option<String>,

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
