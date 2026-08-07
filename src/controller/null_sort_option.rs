// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Null placement in sorted query results.

use serde::Deserialize;
use serde::Serialize;
use std::cmp::Ordering;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::SortOrder;

/// Policy controlling how null values compare with non-null values.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    Model,
    PartialEq,
    Redact,
    Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NullSortOption {
    /// Null values always sort first.
    NullFirst,
    /// Null values always sort last.
    NullLast,
    /// Null values behave as the smallest value.
    #[default]
    NullSmallest,
    /// Null values behave as the largest value.
    NullLargest,
}

impl NullSortOption {
    /// Compares two null-presence flags under this policy and sort direction.
    ///
    /// At least one flag must be `true`; `true` denotes a null operand.
    ///
    /// # Panics
    ///
    /// Panics when neither operand is null, matching the Java source error.
    #[must_use]
    pub fn compare_none(
        self,
        lhs_none: bool,
        rhs_none: bool,
        order: SortOrder,
    ) -> Ordering {
        assert!(lhs_none || rhs_none, "either operand must be null");
        if !lhs_none {
            return self.compare_none(true, false, order).reverse();
        }
        if rhs_none {
            return Ordering::Equal;
        }
        match self {
            Self::NullFirst => Ordering::Less,
            Self::NullLast => Ordering::Greater,
            Self::NullSmallest if order == SortOrder::Asc => Ordering::Less,
            Self::NullSmallest => Ordering::Greater,
            Self::NullLargest if order == SortOrder::Asc => Ordering::Greater,
            Self::NullLargest => Ordering::Less,
        }
    }
}
