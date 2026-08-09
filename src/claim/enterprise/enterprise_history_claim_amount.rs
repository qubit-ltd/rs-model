// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Stored prior-period claim amounts for an enterprise-insured person.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::DictEntryInfo;

/// Historical totals for one covered person, insurance product, and medical
/// category, retained for later enterprise-claim calculations.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct EnterpriseHistoryClaimAmount {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted insurance-product identifier.
    #[model(opaque)]
    pub product_id: Id,

    /// Insured person's name.
    pub name: String,

    /// Insured person's credential number.
    #[redact(level = "secret")]
    pub credential_number: String,

    /// Medical-category dictionary entry.
    pub medical_category: DictEntryInfo,

    /// Historical claim base.
    #[model(money(scale = 4))]
    pub claim_base: BigDecimal,

    /// Historical deductible.
    #[model(money(scale = 4))]
    pub deductible: BigDecimal,

    /// Historical pooled-fund amount.
    #[model(money(scale = 4))]
    pub overall_fund_amount: BigDecimal,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
}
