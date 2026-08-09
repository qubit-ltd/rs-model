// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Calendar and clock granularities for statistics aggregation.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Calendar or clock granularity used to bucket an aggregate.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeDimension {
    /// Calendar-year bucket.
    Year,
    /// Calendar-quarter bucket.
    Quarter,
    /// Calendar-month bucket.
    Month,
    /// Calendar-week bucket.
    Week,
    /// Calendar-day bucket.
    Day,
    /// Hourly bucket.
    Hour,
    /// Minute-level bucket.
    Minute,
    /// Second-level bucket.
    Second,
}
