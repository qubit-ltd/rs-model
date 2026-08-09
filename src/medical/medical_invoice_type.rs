// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Encounter classifications used when importing medical invoices.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies the care setting to which a medical invoice belongs.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MedicalInvoiceType {
    /// Serious-illness clinic invoice.
    ClinicSeriousIllness,
    /// Special outpatient clinic invoice.
    ClinicSpecial,
    /// Hospital invoice.
    Hospital,
    /// Another medical invoice classification.
    Other,
}
