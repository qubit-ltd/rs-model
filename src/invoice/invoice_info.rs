// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Lightweight invoice information.

use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::Currency;
use crate::invoice::InvoiceStatus;

/// A compact invoice snapshot used by related-invoice references.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct InvoiceInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Invoice code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Invoice number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub number: String,

    /// Shared business number for invoices issued for one transaction.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub business_number: String,

    /// Optional invoice issue date.
    pub issue_date: Option<NaiveDate>,

    /// Payee's display name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub payee_name: String,

    /// Payer's display name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub payer_name: String,

    /// Invoice currency.
    pub currency: Currency,

    /// Total invoiced amount.
    #[model(money(scale = 4))]
    pub total_amount: BigDecimal,

    /// Optional electronic-invoice URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub url: Option<String>,

    /// Optional invoice issuance state.
    pub status: Option<InvoiceStatus>,
}
