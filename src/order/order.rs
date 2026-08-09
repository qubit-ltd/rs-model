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

    /// Identifier of the shipment; its default value means that no related record is stored.
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

    /// UTC instant at which payment completed, or `None` before successful payment.
    #[model(time(precision = second, normalization = utc))]
    pub pay_time: Option<DateTime<Utc>>,

    /// UTC instant at which the goods were handed to the carrier, or `None` before dispatch.
    #[model(time(precision = second, normalization = utc))]
    pub ship_time: Option<DateTime<Utc>>,

    /// UTC instant at which the refund completed, or `None` before it succeeds.
    #[model(time(precision = second, normalization = utc))]
    pub refund_time: Option<DateTime<Utc>>,

    /// UTC instant at which processing completed, or `None` until it completes.
    #[model(time(precision = second, normalization = utc))]
    pub complete_time: Option<DateTime<Utc>>,

    /// UTC instant at which it was cancelled, or `None` unless cancellation occurred.
    #[model(time(precision = second, normalization = utc))]
    pub cancel_time: Option<DateTime<Utc>>,

    /// Invoice lifecycle state.
    pub invoice_status: InvoiceStatus,

    /// Optional submitting-client environment.
    pub environment: Option<Environment>,

    /// Optional ordered payload entries.
    #[model(opaque)]
    pub payload: Option<Vec<(String, String)>>,

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
