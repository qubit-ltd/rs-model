// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted employee schedules.

use chrono::DateTime;
use chrono::NaiveDate;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::LocalTimeRange;

/// Work intervals assigned to an employee on a calendar date.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct WorkSchedule {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Employee identifier.
    #[model(opaque)]
    pub employee_id: Id,

    /// Schedule date.
    pub date: Option<NaiveDate>,

    /// Half-open working intervals.
    #[redact(nested)]
    pub intervals: Vec<LocalTimeRange>,

    /// UTC instant at which this record was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// UTC instant of the most recent update, or `None` when no update has occurred.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the record remains active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
