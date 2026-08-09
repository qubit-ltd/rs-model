// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Campaigns, their ordered product entries, and the coupons they issue.

#[allow(clippy::module_inception)]
mod activity;
mod activity_coupon;
mod activity_product_item;

pub use activity::Activity;
pub use activity_coupon::ActivityCoupon;
pub use activity_product_item::ActivityProductItem;
