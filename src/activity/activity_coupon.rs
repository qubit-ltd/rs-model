// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Models for campaign coupons issued to individual people.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::order::OrderInfo;
use crate::person::Person;

/// A coupon granted by a campaign and associated with its recipient and use.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct ActivityCoupon {
    /// Database identifier; the default value denotes an unpersisted coupon record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Reference to the campaign that issued this coupon.
    #[model(opaque)]
    pub activity: Info,

    /// Secret code presented when the recipient redeems the coupon.
    #[model(text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    pub coupon_code: String,

    /// Person to whom this coupon was issued.
    #[redact(nested)]
    pub person: Person,

    /// Order that redeemed this coupon; its empty reference denotes that it is unused.
    pub order: OrderInfo,

    /// UTC instant, rounded to seconds, when the coupon record was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant, rounded to seconds, when the recipient received the coupon.
    #[model(time(precision = second, normalization = utc))]
    pub receive_time: DateTime<Utc>,
}
