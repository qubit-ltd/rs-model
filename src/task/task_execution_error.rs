// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Task lifecycle vocabularies and errors.

use thiserror::Error;


/// An execution failure reported by a task implementation.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
#[error("task execution failed: {message}")]
pub struct TaskExecutionError {
    /// Human-readable failure message.
    pub message: String,
}
