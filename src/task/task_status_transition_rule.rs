// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! The complete legal task-state transition table.

use super::TaskAction;
use super::TaskStatus;
use super::TaskStatusTransitionError;

/// Evaluates the fixed task lifecycle transition table.
#[derive(Clone, Copy, Debug, Default)]
pub struct TaskStatusTransitionRule;
impl TaskStatusTransitionRule {
    /// Returns the status reached by applying `action` to `status`.
    ///
    /// Returns [`TaskStatusTransitionError`] when this pair is not listed in
    /// the Java task lifecycle; terminal states therefore have no outgoing
    /// transitions.
    pub fn next(
        status: TaskStatus,
        action: TaskAction,
    ) -> Result<TaskStatus, TaskStatusTransitionError> {
        let next = match (status, action) {
            (TaskStatus::Created, TaskAction::Submit) => TaskStatus::Submitted,
            (TaskStatus::Created, TaskAction::Fail) => TaskStatus::Failed,
            (TaskStatus::Submitted, TaskAction::Init) => TaskStatus::Initializing,
            (TaskStatus::Submitted, TaskAction::Fail) => TaskStatus::Failed,
            (TaskStatus::Initializing, TaskAction::Start) => TaskStatus::Running,
            (TaskStatus::Initializing, TaskAction::Cancel) => TaskStatus::Cancelled,
            (TaskStatus::Initializing, TaskAction::Fail) => TaskStatus::Failed,
            (TaskStatus::Running, TaskAction::Cancel) => TaskStatus::Cancelled,
            (TaskStatus::Running, TaskAction::Fail) => TaskStatus::Failed,
            (TaskStatus::Running, TaskAction::Success) => TaskStatus::Completed,
            _ => return Err(TaskStatusTransitionError { status, action }),
        };
        Ok(next)
    }
}
