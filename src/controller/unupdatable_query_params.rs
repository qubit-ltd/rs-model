// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Query parameters for creatable and deletable entities.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::NullSortOption;
use super::SortOrder;

/// Paging, sorting, deletion, creation-time, and deletion-time filters.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct UnupdatableQueryParams {
    /// Zero-based page index; defaults to zero.
    pub page_index: Option<i32>,

    /// Optional page size.
    pub page_size: Option<i32>,

    /// Whether to ignore paging and request all rows.
    pub request_all: bool,

    /// Optional CamelCase sort field.
    pub sort_field: Option<String>,

    /// Optional sort direction.
    pub sort_order: Option<SortOrder>,

    /// Optional null placement policy.
    pub null_sort_option: Option<NullSortOption>,

    /// Optional soft-deletion filter.
    pub deleted: Option<bool>,

    /// Inclusive UTC creation-time lower bound.
    #[model(time(precision = second, normalization = utc))]
    pub create_time_start: Option<DateTime<Utc>>,

    /// Inclusive UTC creation-time upper bound.
    #[model(time(precision = second, normalization = utc))]
    pub create_time_end: Option<DateTime<Utc>>,

    /// Inclusive UTC deletion-time lower bound.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time_start: Option<DateTime<Utc>>,

    /// Inclusive UTC deletion-time upper bound.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time_end: Option<DateTime<Utc>>,
}

impl Default for UnupdatableQueryParams {
    fn default() -> Self {
        Self {
            page_index: Some(0),
            page_size: None,
            request_all: false,
            sort_field: None,
            sort_order: None,
            null_sort_option: None,
            deleted: None,
            create_time_start: None,
            create_time_end: None,
            delete_time_start: None,
            delete_time_end: None,
        }
    }
}
