// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Half-open local-time ranges.

use chrono::NaiveTime;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// A half-open interval from `start` inclusive to `end` exclusive.
#[derive(
    Clone, Debug, Default, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
pub struct LocalTimeRange {
    /// Optional inclusive start time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<NaiveTime>,

    /// Optional exclusive end time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<NaiveTime>,
}

impl LocalTimeRange {
    /// Returns whether both interval bounds are absent.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.start.is_none() && self.end.is_none()
    }

    /// Reports whether `time` lies within the half-open interval.
    #[must_use]
    pub fn contains(&self, time: NaiveTime) -> bool {
        self.start.is_none_or(|start| time >= start)
            && self.end.is_none_or(|end| time < end)
    }
}
