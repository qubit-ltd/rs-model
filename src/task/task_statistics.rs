// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Aggregate counters that describe an executor's task workload.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Snapshot of queued, active, completed, and submitted task counts.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct TaskStatistics {
    /// Number of tasks currently being initialized or executed.
    pub active_count: i32,

    /// Number of submitted tasks waiting to start.
    pub waiting_count: i32,

    /// Cumulative number of tasks that reached successful completion.
    pub completed_count: i64,

    /// Cumulative number of tasks submitted to the executor.
    pub submitted_count: i64,
}
