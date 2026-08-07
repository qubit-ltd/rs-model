// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice-title classifications.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies whether an invoice title names a person or organization.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceTitleType {
    /// Personal invoice title.
    Person,
    /// Organizational invoice title.
    Organization,
}
