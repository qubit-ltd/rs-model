// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Error value returned when a task's domain work cannot complete.

use thiserror::Error;

/// Context supplied by a task implementation when its domain work fails.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("task execution failed: {message}")]
pub struct TaskExecutionError {
    /// Human-readable explanation of the execution failure.
    pub message: String,
}
