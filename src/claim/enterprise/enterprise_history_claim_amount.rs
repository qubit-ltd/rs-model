// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Persisted historical enterprise claim amounts.

use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use crate::commons::DictEntryInfo;

/// Historical claim totals for one insured person, product, and medical
/// category.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct EnterpriseHistoryClaimAmount {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Persisted insurance-product identifier.
    pub product_id: i64,
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
