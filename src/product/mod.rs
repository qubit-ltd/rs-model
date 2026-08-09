// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Catalogue models for products, prices, sellers, coupons, and eligibility rules.

mod coupon;
mod coupon_rule;
mod coupon_type;
mod person_constraint;
#[allow(clippy::module_inception)]
mod product;
mod product_constraint;
mod product_info;
mod product_item;
mod product_price;
mod quality;
mod seller;

pub use coupon::Coupon;
pub use coupon_rule::CouponRule;
pub use coupon_type::CouponType;
pub use person_constraint::PersonConstraint;
pub use product::Product;
pub use product_constraint::ProductConstraint;
pub use product_info::ProductInfo;
pub use product_item::ProductItem;
pub use product_price::ProductPrice;
pub use quality::Quality;
pub use seller::Seller;
