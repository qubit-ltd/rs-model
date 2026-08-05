// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Query parameters for auditable entities.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use super::{NullSortOption, SortOrder};

/// Paging, sorting, deletion, and audit-time filters.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct AuditableQueryParams {
    /// Zero-based page index; defaults to zero.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_index: Option<i32>,
    /// Optional page size.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// Whether to ignore paging and request all rows.
    pub request_all: bool,
    /// Optional CamelCase sort field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_field: Option<String>,
    /// Optional sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortOrder>,
    /// Optional null placement policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_sort_option: Option<NullSortOption>,
    /// Optional soft-deletion filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    /// Inclusive UTC creation-time lower bound.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_start: Option<DateTime<Utc>>,
    /// Inclusive UTC creation-time upper bound.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_end: Option<DateTime<Utc>>,
    /// Inclusive UTC modification-time lower bound.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time_start: Option<DateTime<Utc>>,
    /// Inclusive UTC modification-time upper bound.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time_end: Option<DateTime<Utc>>,
    /// Inclusive UTC deletion-time lower bound.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time_start: Option<DateTime<Utc>>,
    /// Inclusive UTC deletion-time upper bound.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time_end: Option<DateTime<Utc>>,
}

impl Default for AuditableQueryParams {
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
            modify_time_start: None,
            modify_time_end: None,
            delete_time_start: None,
            delete_time_end: None,
        }
    }
}
