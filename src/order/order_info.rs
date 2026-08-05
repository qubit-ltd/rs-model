// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Compact order snapshots.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{
    commons::Currency,
    mixin::StatefulInfo,
    order::{Buyer, OrderStatus, PayType},
    product::Seller,
};

/// A compact order summary used by transaction and query results.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct OrderInfo {
    /// Optional persisted identifier.
    pub id: Option<i64>,
    /// Optional persisted owning-user identifier.
    pub user_id: Option<i64>,
    /// Owning application.
    pub app: StatefulInfo,
    /// Buyer information.
    pub buyer: Buyer,
    /// Seller information.
    pub seller: Seller,
    /// Optional order source.
    #[model(opaque)]
    pub source: Option<InfoWithEntity>,
    /// Optional order category.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,
    /// Payment program.
    pub pay_type: PayType,
    /// Currency.
    pub currency: Currency,
    /// Sum of item prices.
    #[model(money(scale = 4))]
    pub total_price: BigDecimal,
    /// Sum of item shipping costs.
    #[model(money(scale = 4))]
    pub total_shipping_cost: BigDecimal,
    /// Sum of item discounts.
    #[model(money(scale = 4))]
    pub total_discount: BigDecimal,
    /// Order-level discount.
    #[model(money(scale = 4))]
    pub discount: BigDecimal,
    /// Order-level shipping cost.
    #[model(money(scale = 4))]
    pub shipping_cost: BigDecimal,
    /// Amount payable.
    #[model(money(scale = 4))]
    pub payable: BigDecimal,
    /// Order lifecycle state.
    pub status: OrderStatus,
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
