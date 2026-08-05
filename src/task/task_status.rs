// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Task lifecycle vocabularies and errors.

#[allow(unused_imports)]
use super::{
    TaskAction,
    TaskExecutionError,
    TaskPipelineStatus,
    TaskStatusTransitionError,
    TaskStatusTransitionRule,
};

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

/// A task lifecycle state.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskStatus {
    /// The Created classification.
    Created,
    /// The Submitted classification.
    Submitted,
    /// The Initializing classification.
    Initializing,
    /// The Running classification.
    Running,
    /// The Failed classification.
    Failed,
    /// The Completed classification.
    Completed,
    /// The Cancelled classification.
    Cancelled,
}
