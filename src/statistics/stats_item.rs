// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Statistics value objects.

#[allow(unused_imports)]
use super::{
    CategoryValue,
    StatsDataset,
    TimeDimension,
};

use bigdecimal::BigDecimal;
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// One named row in a two-dimensional statistics dataset.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct StatsItem {
    /// Item name.
    pub name: String,
    /// Values aligned by index with [`StatsDataset::series`].
    #[model(element(decimal(scale = 2)))]
    pub values: Vec<BigDecimal>,
}
