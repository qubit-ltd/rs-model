// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Client-oriented order query records.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

use crate::invoice::InvoiceStatus;
use crate::mixin::StatefulInfo;
use crate::order::Client;
use crate::order::OrderStatus;
use crate::order::ReturnStatus;
use crate::payment::Account;
use crate::payment::PaymentChannel;
use crate::payment::PaymentMode;
use crate::product::ProductInfo;

/// Order and refund data projected for one client.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct ClientOrder {
    /// Client projection owner.
    pub client: Client,

    /// Application information.
    #[model(opaque)]
    pub app: Info,

    /// Persisted order identifier.
    pub order_id: i64,

    /// Optional ordered payload entries.
    #[model(opaque)]
    pub payload: Option<Vec<(String, String)>>,

    /// Persisted order-item identifier.
    pub order_item_id: i64,

    /// Optional persisted return identifier.
    pub return_id: Option<i64>,

    /// Persisted payment transaction identifier.
    pub pay_transaction_id: i64,

    /// Persisted refund transaction identifier.
    pub refund_transaction_id: i64,

    /// Order lifecycle state.
    pub order_status: OrderStatus,

    /// Return lifecycle state.
    pub return_status: ReturnStatus,

    /// Invoice lifecycle state.
    pub invoice_status: InvoiceStatus,

    /// Purchased product snapshot.
    pub product: ProductInfo,

    /// Purchased quantity.
    pub count: i32,

    /// Amount payable.
    #[model(money(scale = 4))]
    pub payable: BigDecimal,

    /// Amount paid.
    #[model(money(scale = 4))]
    pub paid: BigDecimal,

    /// Optional refundable amount.
    #[model(money(scale = 4))]
    pub refundable: Option<BigDecimal>,

    /// Optional refunded amount.
    #[model(money(scale = 4))]
    pub refunded: Option<BigDecimal>,

    /// UTC order timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub order_time: DateTime<Utc>,

    /// Optional UTC payment timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub pay_time: Option<DateTime<Utc>>,

    /// Optional UTC refund timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub refund_time: Option<DateTime<Utc>>,

    /// Payment channel.
    pub pay_channel: PaymentChannel,

    /// Payment interaction mode.
    pub pay_mode: PaymentMode,

    /// Platform payment number.
    pub pay_number: String,

    /// Optional provider payment number.
    pub pay_channel_number: Option<String>,

    /// Optional refund channel.
    pub refund_channel: Option<PaymentChannel>,

    /// Optional refund interaction mode.
    pub refund_mode: Option<PaymentMode>,

    /// Optional platform refund number.
    pub refund_number: Option<String>,

    /// Optional provider refund number.
    pub refund_channel_number: Option<String>,

    /// Optional order source.
    #[model(opaque)]
    pub source: Option<InfoWithEntity>,

    /// Optional organization snapshot.
    pub organization: Option<StatefulInfo>,

    /// Optional account used by the client.
    pub account: Option<Account>,
}
