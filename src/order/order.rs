// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order aggregate records.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

use crate::commons::Currency;
use crate::invoice::InvoiceStatus;
use crate::mixin::StatefulInfo;
use crate::order::Buyer;
use crate::order::Consignee;
use crate::order::OrderItem;
use crate::order::OrderStatus;
use crate::order::PayType;
use crate::product::Seller;
use crate::shipping::ShippingDemand;
use crate::shipping::ShippingMode;
use crate::system::Environment;

/// A submitted order with line items, pricing, delivery, and lifecycle data.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Order {
    /// Optional persisted identifier and order number.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Optional persisted owning-user identifier.
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

    /// Ordered line items.
    #[model(sequence(min_items = 1, max_items = 20))]
    pub items: Vec<OrderItem>,

    /// Payment program.
    pub pay_type: PayType,

    /// Currency.
    pub currency: Currency,

    /// Sum of line prices.
    #[model(money(scale = 4))]
    pub total_price: BigDecimal,

    /// Sum of line shipping costs.
    #[model(money(scale = 4))]
    pub total_shipping_cost: BigDecimal,

    /// Sum of line discounts.
    #[model(money(scale = 4))]
    pub total_discount: BigDecimal,

    /// Order-level discount.
    #[model(money(scale = 4))]
    pub discount: BigDecimal,

    /// Optional order-level discount reason.
    pub discount_reason: Option<String>,

    /// Order-level shipping cost.
    #[model(money(scale = 4))]
    pub shipping_cost: BigDecimal,

    /// Final amount payable.
    #[model(money(scale = 4))]
    pub payable: BigDecimal,

    /// Delivery mode.
    pub shipping_mode: ShippingMode,

    /// Optional consignee.
    pub consignee: Option<Consignee>,

    /// Optional delivery requirements.
    pub shipping_demand: Option<ShippingDemand>,

    /// Optional persisted shipment identifier.
    #[model(opaque)]
    pub shipping_id: Id,

    /// Optional shipment tracking number.
    pub shipping_number: Option<String>,

    /// Optional order comment.
    pub comment: Option<String>,

    /// Order lifecycle state.
    pub status: OrderStatus,

    /// UTC order expiration timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub expired_time: DateTime<Utc>,

    /// Optional UTC payment timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub pay_time: Option<DateTime<Utc>>,

    /// Optional UTC shipment timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub ship_time: Option<DateTime<Utc>>,

    /// Optional UTC refund timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub refund_time: Option<DateTime<Utc>>,

    /// Optional UTC completion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub complete_time: Option<DateTime<Utc>>,

    /// Optional UTC cancellation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub cancel_time: Option<DateTime<Utc>>,

    /// Invoice lifecycle state.
    pub invoice_status: InvoiceStatus,

    /// Optional submitting-client environment.
    pub environment: Option<Environment>,

    /// Optional ordered payload entries.
    #[model(opaque)]
    pub payload: Option<Vec<(String, String)>>,

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
