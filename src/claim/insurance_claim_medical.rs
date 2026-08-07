// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Individual claim medical encounters.

use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDate, Utc};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{claim::InsuranceClaimInvoice, medical::MedicalType};

/// A medical encounter and its invoices within an individual claim.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct InsuranceClaimMedical {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Persisted claim identifier.
    pub claim_id: i64,

    /// Optional insurer-side claim application identifier.
    pub claim_apply_id: Option<String>,

    /// Treatment date.
    pub treatment_date: NaiveDate,

    /// Medical record or invoice number.
    pub number: String,

    /// Medical encounter classification.
    pub medical_type: MedicalType,

    /// Whether the encounter describes a pre-existing symptom.
    pub past_symptom: bool,

    /// Treating hospital.
    #[model(opaque)]
    pub hospital: Info,

    /// Treating department name.
    pub department: String,

    /// Total encounter amount.
    #[model(money(scale = 4))]
    pub amount: BigDecimal,

    /// Primary diagnosis.
    pub primary_diagnosis: String,

    /// Whether the encounter has an invoice.
    pub has_invoice: bool,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,

    /// Invoices belonging to this encounter.
    pub invoice_list: Vec<InsuranceClaimInvoice>,
}
