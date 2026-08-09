// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Billed medical items and their reimbursement allocations.

use bigdecimal::BigDecimal;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::DictEntryInfo;
use crate::medical::MedicareItemType;

/// A billed service or medicine together with its patient and insurer liability.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MedicalSettlementItem {
    /// Typed identifier used when this settlement line is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted identifier of the owning medical settlement.
    #[model(opaque)]
    pub settlement_id: Id,

    /// Zero-based source order of this line in the settlement detail list.
    pub index: i32,

    /// Medical-insurance item classification.
    pub r#type: MedicareItemType,

    /// Dictionary charge category used by the source medical-insurance system.
    pub charge_type: DictEntryInfo,

    /// Charged item code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Charged item name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Source item specification, absent when the billed item has none.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub specification: Option<String>,

    /// Billing unit, absent when the source reports an amount without one.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub unit: Option<String>,

    /// Unit price.
    #[model(money(scale = 4))]
    pub price: BigDecimal,

    /// Billed quantity.
    #[model(decimal(scale = 2))]
    pub amount: BigDecimal,

    /// Gross line charge before applying coverage and reimbursement limits.
    #[model(money(scale = 4))]
    pub total_price: BigDecimal,

    /// Patient self-paid amount.
    #[model(money(scale = 4))]
    pub self_paid: BigDecimal,

    /// Source-provided self-pay rate used to allocate this line to the patient.
    #[model(decimal(scale = 4))]
    pub self_paid_rate: BigDecimal,

    /// Maximum amount of this line eligible for reimbursement.
    #[model(money(scale = 4))]
    pub reimburse_limit: BigDecimal,

    /// Amount payable for this line after the settlement allocation.
    #[model(money(scale = 4))]
    pub payable: BigDecimal,

    /// Source line-item note, absent when no clarification accompanies the charge.
    pub remark: Option<String>,
}
