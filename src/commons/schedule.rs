// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Cron-based execution schedules.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;

use qubit_mixin::Emptyful;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// A bounded schedule described by Spring-style cron expressions.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct Schedule {
    /// UTC schedule start timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub start_time: Option<DateTime<Utc>>,

    /// Optional UTC schedule end timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub end_time: Option<DateTime<Utc>>,

    /// Optional list of one to ten cron expressions.
    #[model(sequence(min_items = 1, max_items = 10))]
    pub crontabs: Option<Vec<String>>,
}

impl Schedule {
    /// Returns whether the schedule has no bounds or cron expressions.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.start_time.is_none()
            && self.end_time.is_none()
            && self.crontabs.as_ref().is_none_or(Vec::is_empty)
    }
}

impl Emptyful for Schedule {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for Schedule {
    fn normalize(&mut self) {
        self.crontabs.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}
