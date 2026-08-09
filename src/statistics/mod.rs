// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Tabular and category-based value objects used in statistical responses.

mod category_value;
mod stats_dataset;
mod stats_item;
mod time_dimension;

pub use category_value::CategoryValue;
pub use stats_dataset::StatsDataset;
pub use stats_item::StatsItem;
pub use time_dimension::TimeDimension;
