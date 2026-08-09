// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Named medical charges extracted from individual-claim invoices.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// One extracted medical charge within a claim invoice's billed-cost breakdown.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InsuranceClaimInvoiceCost {
    /// Typed identifier used when this invoice charge line is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted claim identifier.
    #[model(opaque)]
    pub claim_id: Id,

    /// Persisted medical-record identifier within the claim.
    #[model(opaque)]
    pub claim_medical_id: Id,

    /// Persisted invoice identifier within the claim.
    #[model(opaque)]
    pub claim_invoice_id: Id,

    /// Label of the billed medical service or charge category.
    pub medical_charge_name: String,

    /// Amount charged for this named component of the source invoice.
    #[model(money(scale = 4))]
    pub amount: BigDecimal,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
}
