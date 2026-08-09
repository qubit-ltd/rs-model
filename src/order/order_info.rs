// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Compact order snapshots.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

use crate::commons::Currency;
use crate::mixin::StatefulInfo;
use crate::order::Buyer;
use crate::order::OrderStatus;
use crate::order::PayType;
use crate::product::Seller;

/// A compact order summary used by transaction and query results.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrderInfo {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(unique, opaque)]
    pub id: Id,

    /// Identifier of the owning-user; its default value means that no related record is stored.
    #[model(opaque)]
    pub user_id: Id,

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
