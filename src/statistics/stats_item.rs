// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Statistics value objects.

use bigdecimal::BigDecimal;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// One named row in a two-dimensional statistics dataset.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StatsItem {
    /// Item name.
    pub name: String,

    /// Values aligned by index with [`StatsDataset::series`].
    #[model(element(decimal(scale = 2)))]
    pub values: Vec<BigDecimal>,
}
