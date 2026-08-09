// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Ways in which coupon rules reduce a purchase price.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The discount algorithm selected by a coupon rule.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CouponType {
    /// Subtracts one fixed amount from the qualifying price.
    DirectDiscount,
    /// Applies the discount once for each qualifying price interval.
    DiscountEvery,
}
