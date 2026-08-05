// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Statistics domain models.

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// A value aggregated for one category and optional time interval.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct CategoryValue {
    /// Category name.
    pub category: String,
    /// Aggregated value for the category.
    pub value: BigDecimal,
    /// Optional inclusive start of the aggregation interval.
    #[model(time(precision = second, normalization = utc))]
    pub start_time: Option<DateTime<Utc>>,
    /// Optional inclusive end of the aggregation interval.
    #[model(time(precision = second, normalization = utc))]
    pub end_time: Option<DateTime<Utc>>,
}

/// One named row in a two-dimensional statistics dataset.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct StatsItem {
    /// Item name.
    pub name: String,
    /// Values aligned by index with [`StatsDataset::series`].
    #[model(element(decimal(scale = 2)))]
    pub values: Vec<BigDecimal>,
}

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
