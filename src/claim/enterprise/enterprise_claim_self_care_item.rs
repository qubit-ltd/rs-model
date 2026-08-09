// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Self-care charge components extracted from enterprise-claim invoices.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// A charge for which the insured person bears a defined self-care share.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct EnterpriseClaimSelfCareItem {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted enterprise claim-invoice identifier.
    #[model(opaque)]
    pub claim_invoice_id: Id,

    /// Name of the Class-B service, material, or medicine charge.
    pub name: String,

    /// Medical-insurance catalogue code for the charge.
    pub medicare_charge_code: String,

    /// Full billed amount before applying the patient's self-care ratio.
    #[model(money(scale = 4))]
    pub amount: BigDecimal,

    /// Patient-borne share of `amount`, expressed as an inclusive ratio from
    /// zero to one.
    pub ratio: f64,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
