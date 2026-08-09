// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Legal subject types accepted in an invoice title.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Whether the invoice title belongs to an individual or an organization.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceTitleType {
    /// A title issued to an individual.
    Person,
    /// A title issued to a legal organization.
    Organization,
}
