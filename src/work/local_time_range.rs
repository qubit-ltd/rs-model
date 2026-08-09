// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Half-open wall-clock time ranges used in local work schedules.

use chrono::NaiveTime;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// A local wall-clock interval with an inclusive start and exclusive end.
#[derive(Model, Redact, Clone, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct LocalTimeRange {
    /// Inclusive local start time, or `None` when the range has no lower bound.
    pub start: Option<NaiveTime>,

    /// Exclusive local end time, or `None` when the range has no upper bound.
    pub end: Option<NaiveTime>,
}

impl LocalTimeRange {
    /// Returns `true` when neither a start nor an end bound is set.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    /// Returns whether `time` satisfies the inclusive-start, exclusive-end bounds.
    #[must_use]
    pub fn contains(&self, time: NaiveTime) -> bool {
        self.start.is_none_or(|start| time >= start) && self.end.is_none_or(|end| time < end)
    }
}
