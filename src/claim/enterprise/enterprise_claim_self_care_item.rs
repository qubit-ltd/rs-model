// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise claim self-care items.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// A partially self-paid charge extracted from an enterprise claim invoice.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct EnterpriseClaimSelfCareItem {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Persisted enterprise claim-invoice identifier.
    pub claim_invoice_id: i64,

    /// Charge name.
    pub name: String,

    /// Medical-insurance charge code.
    pub medicare_charge_code: String,

    /// Charge amount.
    #[model(money(scale = 4))]
    pub amount: BigDecimal,

    /// Self-care ratio.
    pub ratio: f64,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
