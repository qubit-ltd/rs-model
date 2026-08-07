// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Statistics value objects.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

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
