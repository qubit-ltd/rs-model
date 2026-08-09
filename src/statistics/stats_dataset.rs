// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tabular statistics datasets with ordered series and rows.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use super::StatsItem;

/// A two-dimensional dataset whose row values align with its ordered series.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StatsDataset {
    /// Display name for the dataset, or `None` when the response is unnamed.
    pub name: Option<String>,

    /// Human-readable explanation of the dataset, or `None` when omitted.
    pub description: Option<String>,

    /// Ordered column or series labels used to interpret every item's values.
    pub series: Vec<String>,

    /// Ordered rows; each row's values correspond by index to [`Self::series`].
    pub items: Vec<StatsItem>,
}
