// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice line items.

use bigdecimal::BigDecimal;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// A billed item with discounts, tax, and final payment amounts.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InvoiceItem {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted identifier of the owning invoice.
    #[model(opaque)]
    pub invoice_id: Id,

    /// Zero-based position in the invoice line-item list.
    pub index: i32,

    /// Billed item code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Billed item name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional item specification.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub specification: Option<String>,

    /// Optional billing unit.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub unit: Option<String>,

    /// Unit price.
    #[model(money(scale = 4))]
    pub price: BigDecimal,

    /// Optional per-unit discount.
    #[model(money(scale = 4))]
    pub discount: Option<BigDecimal>,

    /// Billed quantity.
    #[model(decimal(scale = 4))]
    pub amount: BigDecimal,

    /// Optional total-price discount.
    #[model(money(scale = 4))]
    pub total_discount: Option<BigDecimal>,

    /// Amount due before tax.
    #[model(money(scale = 4))]
    pub payable: BigDecimal,

    /// Optional tax rate.
    #[model(decimal(scale = 4))]
    pub tax_rate: Option<BigDecimal>,

    /// Optional tax amount.
    #[model(money(scale = 4))]
    pub tax: Option<BigDecimal>,

    /// Amount paid including tax.
    #[model(money(scale = 4))]
    pub paid: BigDecimal,

    /// Optional remark.
    pub remark: Option<String>,
}
