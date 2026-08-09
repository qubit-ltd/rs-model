// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Settlement transaction records.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

use crate::commons::App;
use crate::commons::Category;
use crate::commons::Currency;
use crate::commons::Source;
use crate::invoice::InvoiceStatus;
use crate::mixin::StatefulInfo;
use crate::order::Order;
use crate::order::Return;
use crate::order::ReturnIssuer;
use crate::payment::Participant;
use crate::payment::Payment;
use crate::settlement::TransactionStatus;
use crate::settlement::TransactionType;
use crate::system::Environment;

/// A purchase or refund transaction and its payment participants.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Transaction {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Optional transaction classification, omitted in filtered gateway
    /// requests.
    pub r#type: Option<TransactionType>,

    /// Optional identifier of the originating transaction.
    #[model(reference(target = Transaction, target_field = id), opaque)]
    pub origin_id: Id,

    /// Optional transaction state, omitted in filtered gateway requests.
    pub status: Option<TransactionStatus>,

    /// Optional owning application, omitted in filtered gateway requests.
    #[model(reference(target = App, target_field = info))]
    pub app: Option<StatefulInfo>,

    /// Optional transaction source.
    #[model(reference(target = Source, target_field = info), opaque)]
    pub source: Option<InfoWithEntity>,

    /// Optional transaction category.
    #[model(reference(target = Category, target_field = info), opaque)]
    pub category: Option<InfoWithEntity>,

    /// Persisted order identifier.
    #[model(reference(target = Order, target_field = id), opaque)]
    pub order_id: Id,

    /// Identifier of the return; its default value means that no related record is stored.
    #[model(reference(target = Return, target_field = id), opaque)]
    pub return_id: Id,

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

    /// UTC instant at which processing completed, or `None` until it completes.
    #[model(time(precision = second, normalization = utc))]
    pub complete_time: Option<DateTime<Utc>>,

    /// Optional invoice state, omitted in filtered gateway requests.
    pub invoice_status: Option<InvoiceStatus>,

    /// Optional client environment.
    pub environment: Option<Environment>,

    /// Optional transaction comment.
    pub comment: Option<String>,

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
