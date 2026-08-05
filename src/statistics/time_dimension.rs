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
    StatsItem,
};

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Time granularity used by a statistics aggregation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeDimension {
    /// Calendar year.
    Year,
    /// Calendar quarter.
    Quarter,
    /// Calendar month.
    Month,
    /// Calendar week.
    Week,
    /// Calendar day.
    Day,
    /// Hour.
    Hour,
    /// Minute.
    Minute,
    /// Second.
    Second,
}
