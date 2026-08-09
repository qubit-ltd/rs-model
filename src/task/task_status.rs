// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! States through which an executable task progresses.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// The persisted lifecycle state of an executable task.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskStatus {
    /// Created locally but not yet submitted for processing.
    Created,
    /// Queued and awaiting initialization.
    Submitted,
    /// Being prepared to execute.
    Initializing,
    /// Actively performing its domain work.
    Running,
    /// Ended because initialization or execution failed.
    Failed,
    /// Ended successfully.
    Completed,
    /// Ended by cancellation before successful completion.
    Cancelled,
}
