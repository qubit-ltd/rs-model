// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Organization tax-payer classifications.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// The tax-payer classification of an organization.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxPayerType {
    /// A small-scale taxpayer.
    SmallScale,
    /// A general taxpayer.
    General,
    /// Any other taxpayer classification.
    Other,
}
