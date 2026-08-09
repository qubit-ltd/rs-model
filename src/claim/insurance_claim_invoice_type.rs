// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Care-setting classifications for invoices imported into a claim.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies the encounter category represented by a claim invoice.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuranceClaimInvoiceType {
    /// Serious-illness clinic invoice.
    ClinicSeriousIllness,
    /// Special outpatient clinic invoice.
    ClinicSpecial,
    /// Hospitalization invoice.
    Hospitalization,
}
