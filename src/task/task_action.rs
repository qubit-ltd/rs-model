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
    TaskExecutionError,
    TaskPipelineStatus,
    TaskStatus,
    TaskStatusTransitionError,
    TaskStatusTransitionRule,
};

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

/// An operation applied to a task.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskAction {
    /// The Submit classification.
    Submit,
    /// The Init classification.
    Init,
    /// The Start classification.
    Start,
    /// The Cancel classification.
    Cancel,
    /// The Fail classification.
    Fail,
    /// The Success classification.
    Success,
}
