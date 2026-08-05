// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! The complete legal task-state transition table.

#[allow(unused_imports)]
use super::{
    TaskAction,
    TaskExecutionError,
    TaskPipelineStatus,
    TaskStatus,
    TaskStatusTransitionRule,
};

use thiserror::Error;

/// An attempted task transition that the lifecycle does not permit.
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
#[error("cannot apply {action:?} to task status {status:?}")]
pub struct TaskStatusTransitionError {
    /// State before the invalid action.
    pub status: TaskStatus,
    /// Action that is invalid for `status`.
    pub action: TaskAction,
}
