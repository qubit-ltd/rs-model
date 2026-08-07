// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Claim-invoice charge breakdowns.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// A named medical charge extracted from a claim invoice.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct InsuranceClaimInvoiceCost {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Persisted claim identifier.
    pub claim_id: i64,

    /// Persisted medical-record identifier within the claim.
    pub claim_medical_id: i64,

    /// Persisted invoice identifier within the claim.
    pub claim_invoice_id: i64,

    /// Medical charge name.
    pub medical_charge_name: String,

    /// Charge amount.
    #[model(money(scale = 4))]
    pub amount: BigDecimal,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
}
