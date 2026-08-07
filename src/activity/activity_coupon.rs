// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Coupons issued by marketing activities.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::order::OrderInfo;
use crate::person::Person;

/// A coupon issued to a person by an activity.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct ActivityCoupon {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Activity that issued the coupon.
    #[model(opaque)]
    pub activity: Info,

    /// Coupon redemption code.
    #[model(text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    pub coupon_code: String,

    /// Person receiving the coupon.
    #[redact(nested)]
    pub person: Person,

    /// Order in which the coupon is used.
    pub order: OrderInfo,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC receipt timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub receive_time: DateTime<Utc>,
}
