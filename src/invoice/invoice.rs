// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Complete invoice records.

use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{
    commons::Currency,
    invoice::{InvoiceInfo, InvoiceItem, InvoiceStatus},
    mixin::StatefulInfo,
    payment::{Participant, PaymentChannel, PaymentMode},
    settlement::Settlement,
};

/// An issued invoice with payment parties, amounts, items, and lifecycle data.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Invoice {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Application that owns this invoice.
    pub app: StatefulInfo,
    /// Organization that owns this invoice.
    pub organization: StatefulInfo,
    /// Invoice-issuing location.
    #[model(opaque)]
    pub place: Info,
    /// Invoice code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,
    /// Invoice number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub number: String,
    /// Invoice verification code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub verify_code: String,
    /// Shared business number for invoices issued for one transaction.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub business_number: String,
    /// UTC timestamp of the invoiced business event.
    #[model(time(precision = second, normalization = utc))]
    pub business_time: DateTime<Utc>,
    /// Payment payee.
    pub payee: Participant,
    /// Payment payer.
    pub payer: Participant,
    /// Payment channel.
    pub payment_channel: PaymentChannel,
    /// Payment mode.
    pub payment_mode: PaymentMode,
    /// Invoice currency.
    pub currency: Currency,
    /// Optional exchange rate.
    #[model(decimal(scale = 8))]
    pub exchange_rate: Option<BigDecimal>,
    /// Total invoiced amount.
    #[model(money(scale = 4))]
    pub total_price: BigDecimal,
    /// Optional remark.
    pub remark: Option<String>,
    /// Invoice drawer's name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub drawer: String,
    /// Invoice auditor's name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub auditor: String,
    /// Optional fiscal-supervisor remark.
    pub supervisor_remark: Option<String>,
    /// Optional original invoice referenced by a red invoice.
    pub related_invoice: Option<InvoiceInfo>,
    /// Issuing-organization seal number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub seal_number: String,
    /// Optional invoice line items.
    pub items: Option<Vec<InvoiceItem>>,
    /// Optional associated settlement summary.
    pub settlement: Option<Settlement>,
    /// Optional electronic-invoice URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,
    /// Optional invoice issuance state.
    pub status: Option<InvoiceStatus>,
    /// Optional invoice issue date.
    pub issue_date: Option<NaiveDate>,
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
