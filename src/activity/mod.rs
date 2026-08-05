// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Marketing activities, product items, and issued coupons.

#[allow(clippy::module_inception)]
mod activity;
mod activity_coupon;
mod activity_product_item;

pub use activity::Activity;
pub use activity_coupon::ActivityCoupon;
pub use activity_product_item::ActivityProductItem;
