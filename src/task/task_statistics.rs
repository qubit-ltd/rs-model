// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Counters summarizing task execution.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Aggregate counters for a task executor.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Eq, Serialize)]
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
