// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Historical enterprise claim amounts.

use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Historical claim base, deductible, and pooled-fund totals.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct HistoryClaimAmount {
    /// Historical claim base.
    #[model(money(scale = 4))]
    pub claim_base: BigDecimal,

    /// Historical deductible.
    #[model(money(scale = 4))]
    pub deductible: BigDecimal,

    /// Historical pooled-fund amount.
    #[model(money(scale = 4))]
    pub overall_fund_amount: BigDecimal,
}
