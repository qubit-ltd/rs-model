// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order return records.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::Currency;
use crate::invoice::InvoiceStatus;
use crate::order::Client;
use crate::order::ReturnIssuer;
use crate::order::ReturnReason;
use crate::order::ReturnStatus;
use crate::product::ProductInfo;
use crate::system::Environment;

/// A return request and its refund, shipping, and lifecycle state.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Return {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted order identifier.
    #[model(opaque)]
    pub order_id: Id,

    /// Persisted order-item identifier.
    #[model(opaque)]
    pub order_item_id: Id,

    /// Optional persisted refund-transaction identifier.
    #[model(opaque)]
    pub transaction_id: Id,

    /// Party that initiated the return.
    pub issuer: ReturnIssuer,

    /// Returned product snapshot.
    pub product: ProductInfo,

    /// Returned quantity.
    pub count: i32,

    /// Refund currency.
    pub currency: Currency,

    /// Optional amount eligible for refund.
    #[model(money(scale = 4))]
    pub refundable: Option<BigDecimal>,

    /// Optional amount already refunded.
    #[model(money(scale = 4))]
    pub refunded: Option<BigDecimal>,

    /// Optional clients associated with the returned item.
    pub clients: Option<Vec<Client>>,

    /// Return reason.
    pub reason: ReturnReason,

    /// Optional return comment.
    pub comment: Option<String>,

    /// Return lifecycle state.
    pub status: ReturnStatus,

    /// Optional rejection reason.
    pub reject_reason: Option<String>,

    /// Optional persisted return-shipment identifier.
    #[model(opaque)]
    pub shipping_id: Id,

    /// Optional return-shipment tracking number.
    pub shipping_number: Option<String>,

    /// Invoice lifecycle state.
    pub invoice_status: InvoiceStatus,

    /// Optional submitting-client environment.
    pub environment: Option<Environment>,

    /// UTC return expiration timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub expired_time: DateTime<Utc>,

    /// Optional UTC refund timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub refund_time: Option<DateTime<Utc>>,

    /// Optional UTC shipment timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub ship_time: Option<DateTime<Utc>>,

    /// Optional UTC completion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub complete_time: Option<DateTime<Utc>>,

    /// Optional UTC cancellation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub cancel_time: Option<DateTime<Utc>>,

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
