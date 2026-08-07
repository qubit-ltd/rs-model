// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Statistics value objects.

#[allow(unused_imports)]
use super::{CategoryValue, StatsItem, TimeDimension};

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// A named two-dimensional statistics dataset.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct StatsDataset {
    /// Optional dataset name.
    pub name: Option<String>,

    /// Optional dataset description.
    pub description: Option<String>,

    /// Ordered series names.
    pub series: Vec<String>,

    /// Ordered data items whose values correspond to [`Self::series`].
    pub items: Vec<StatsItem>,
}
