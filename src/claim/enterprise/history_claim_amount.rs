// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Prior-period amounts used in enterprise-claim calculations.

use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Historical calculation inputs: claim base, deductible, and pooled-fund
/// amount.
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
