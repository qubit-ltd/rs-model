// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rows used in tabular statistics datasets.

use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// A named row of decimal values in a [`StatsDataset`].
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StatsItem {
    /// Label identifying this row.
    pub name: String,

    /// Decimal cells aligned by index with [`StatsDataset::series`].
    #[model(element(decimal(scale = 2)))]
    pub values: Vec<BigDecimal>,
}
