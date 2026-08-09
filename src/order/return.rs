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
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted order identifier.
    #[model(opaque)]
    pub order_id: Id,

    /// Persisted order-item identifier.
    #[model(opaque)]
    pub order_item_id: Id,

    /// Identifier of the refund-transaction; its default value means that no related record is stored.
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

    /// Identifier of the return-shipment; its default value means that no related record is stored.
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

    /// UTC instant at which the refund completed, or `None` before it succeeds.
    #[model(time(precision = second, normalization = utc))]
    pub refund_time: Option<DateTime<Utc>>,

    /// UTC instant at which the goods were handed to the carrier, or `None` before dispatch.
    #[model(time(precision = second, normalization = utc))]
    pub ship_time: Option<DateTime<Utc>>,

    /// UTC instant at which processing completed, or `None` until it completes.
    #[model(time(precision = second, normalization = utc))]
    pub complete_time: Option<DateTime<Utc>>,

    /// UTC instant at which it was cancelled, or `None` unless cancellation occurred.
    #[model(time(precision = second, normalization = utc))]
    pub cancel_time: Option<DateTime<Utc>>,

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
