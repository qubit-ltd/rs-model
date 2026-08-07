// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Counters summarizing task execution.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Aggregate counters for a task executor.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct TaskStatistics {
    /// Number of currently active tasks.
    pub active_count: i32,

    /// Number of queued tasks.
    pub waiting_count: i32,

    /// Number of completed tasks.
    pub completed_count: i64,

    /// Total number of submitted tasks.
    pub submitted_count: i64,
}
