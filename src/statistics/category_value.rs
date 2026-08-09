// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Values grouped by a category and, optionally, a UTC time interval.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// A decimal aggregate for one category, optionally bounded by UTC instants.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CategoryValue {
    /// Label of the category represented by this aggregate.
    pub category: String,

    /// Decimal value accumulated for the category.
    pub value: BigDecimal,

    /// UTC start of the aggregation interval, or `None` when no start is recorded.
    #[model(time(precision = second, normalization = utc))]
    pub start_time: Option<DateTime<Utc>>,

    /// UTC end of the aggregation interval, or `None` when no end is recorded.
    #[model(time(precision = second, normalization = utc))]
    pub end_time: Option<DateTime<Utc>>,
}
