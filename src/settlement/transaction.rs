// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Settlement transaction records.

use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    Utc,
};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    commons::Currency,
    invoice::InvoiceStatus,
    mixin::StatefulInfo,
    order::ReturnIssuer,
    payment::{
        Participant,
        Payment,
    },
    settlement::{
        TransactionStatus,
        TransactionType,
    },
    system::Environment,
};

/// A purchase or refund transaction and its payment participants.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Transaction {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Optional transaction classification, omitted in filtered gateway
    /// requests.
    pub r#type: Option<TransactionType>,
    /// Optional identifier of the originating transaction.
    pub origin_id: Option<i64>,
    /// Optional transaction state, omitted in filtered gateway requests.
    pub status: Option<TransactionStatus>,
    /// Optional owning application, omitted in filtered gateway requests.
    pub app: Option<StatefulInfo>,
    /// Optional transaction source.
    #[model(opaque)]
    pub source: Option<InfoWithEntity>,
    /// Optional transaction category.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,
    /// Persisted order identifier.
    pub order_id: i64,
    /// Optional persisted return identifier.
    pub return_id: Option<i64>,
    /// Optional party that initiated a return.
    pub return_issuer: Option<ReturnIssuer>,
    /// Transaction currency.
    pub currency: Currency,
    /// Amount due.
    #[model(money(scale = 4))]
    pub payable: BigDecimal,
    /// Optional discount, omitted in filtered gateway requests.
    #[model(money(scale = 4))]
    pub discount: Option<BigDecimal>,
    /// Optional amount paid, omitted in filtered gateway requests.
    #[model(money(scale = 4))]
    pub paid: Option<BigDecimal>,
    /// Optional payee, omitted in filtered gateway requests.
    pub payee: Option<Participant>,
    /// Payer information sent to the payment provider.
    pub payer: Participant,
    /// Optional provider-side payment record.
    pub payment: Option<Payment>,
    /// UTC transaction expiration timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub expired_time: DateTime<Utc>,
    /// Optional UTC completion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub complete_time: Option<DateTime<Utc>>,
    /// Optional invoice state, omitted in filtered gateway requests.
    pub invoice_status: Option<InvoiceStatus>,
    /// Optional client environment.
    pub environment: Option<Environment>,
    /// Optional transaction comment.
    pub comment: Option<String>,
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
