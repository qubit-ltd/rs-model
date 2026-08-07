// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical settlement line items.

use bigdecimal::BigDecimal;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{commons::DictEntryInfo, medical::MedicareItemType};

/// A charged medical item and its insurance reimbursement breakdown.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct MedicalSettlementItem {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Persisted identifier of the owning medical settlement.
    pub settlement_id: i64,

    /// Zero-based position in the settlement item list.
    pub index: i32,

    /// Medical-insurance item classification.
    pub r#type: MedicareItemType,

    /// Charge-type dictionary entry.
    pub charge_type: DictEntryInfo,

    /// Charged item code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Charged item name.
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

    /// Billed quantity.
    #[model(decimal(scale = 2))]
    pub amount: BigDecimal,

    /// Total price before reimbursement.
    #[model(money(scale = 4))]
    pub total_price: BigDecimal,

    /// Patient self-paid amount.
    #[model(money(scale = 4))]
    pub self_paid: BigDecimal,

    /// Patient self-paid rate.
    #[model(decimal(scale = 4))]
    pub self_paid_rate: BigDecimal,

    /// Maximum reimbursable amount.
    #[model(money(scale = 4))]
    pub reimburse_limit: BigDecimal,

    /// Amount due after reimbursement.
    #[model(money(scale = 4))]
    pub payable: BigDecimal,

    /// Optional remark.
    pub remark: Option<String>,
}
