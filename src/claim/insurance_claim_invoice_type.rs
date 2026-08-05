// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Claim-invoice classifications.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Identifies the medical encounter represented by a claim invoice.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuranceClaimInvoiceType {
    /// Serious-illness clinic invoice.
    ClinicSeriousIllness,
    /// Special outpatient clinic invoice.
    ClinicSpecial,
    /// Hospitalization invoice.
    Hospitalization,
}
