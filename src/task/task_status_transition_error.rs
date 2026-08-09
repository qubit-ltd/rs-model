// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Error reported when a requested task lifecycle transition is not legal.

use thiserror::Error;

use super::TaskAction;
use super::TaskStatus;

/// A rejected task action together with the state in which it was attempted.
#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
#[error("cannot apply {action:?} to task status {status:?}")]
pub struct TaskStatusTransitionError {
    /// Task state from which the rejected action was requested.
    pub status: TaskStatus,

    /// Lifecycle command that has no legal transition from `status`.
    pub action: TaskAction,
}
