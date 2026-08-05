// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Cron-based execution schedules.

use chrono::{
    DateTime,
    Utc,
};
use qubit_mixin::{
    Emptyful,
    Normalizable,
};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

/// A bounded schedule described by Spring-style cron expressions.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
pub struct Schedule {
    /// UTC schedule start timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,
    /// Optional UTC schedule end timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
    /// Optional list of one to ten cron expressions.
    #[model(sequence(min_items = 1, max_items = 10))]
    #[serde(skip_serializing_if = "Option::is_none")]
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
