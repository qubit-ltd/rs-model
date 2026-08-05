// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Coupon classifications.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Describes how a coupon reduces a price.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CouponType {
    /// Subtracts a fixed amount directly.
    DirectDiscount,
    /// Subtracts an amount for every qualifying price interval.
    DiscountEvery,
}
