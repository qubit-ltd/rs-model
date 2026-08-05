// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted employee schedules.

use chrono::{DateTime, NaiveDate, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use super::LocalTimeRange;

/// Work intervals assigned to an employee on a calendar date.
#[derive(Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct WorkSchedule {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Employee identifier.
    pub employee_id: i64,
    /// Schedule date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<NaiveDate>,
    /// Half-open working intervals.
    #[redact(nested)]
    pub intervals: Vec<LocalTimeRange>,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}
