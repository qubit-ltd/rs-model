// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Commands that drive the task lifecycle.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// A command that requests a permitted task lifecycle transition.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskAction {
    /// Queue a newly created task for initialization.
    Submit,
    /// Begin preparing a submitted task for execution.
    Init,
    /// Start the prepared task's work.
    Start,
    /// Stop the task before it reaches a successful result.
    Cancel,
    /// Record an unrecoverable initialization or execution failure.
    Fail,
    /// Record successful completion of the running task.
    Success,
}
