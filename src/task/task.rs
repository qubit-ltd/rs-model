// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Contract implemented by domain tasks that participate in the standard lifecycle.

use qubit_mixin::InfoWithEntity;

use super::TaskAction;
use super::TaskExecutionError;
use super::TaskInfo;
use super::TaskStatusTransitionError;
use super::TaskStatusTransitionRule;

/// Domain work plus the metadata needed to persist and transition a task.
pub trait Task {
    /// Returns the task's persisted metadata and current lifecycle state.
    fn info(&self) -> &TaskInfo;

    /// Returns mutable task metadata for implementations that update task state.
    fn info_mut(&mut self) -> &mut TaskInfo;

    /// Returns the task identifier as an `i64`; the default identifier is returned for an unsaved task.
    fn id(&self) -> Option<i64> {
        Some(self.info().id.value() as i64)
    }

    /// Returns the category name used as this task's source-facing name.
    fn name(&self) -> &str {
        self.info().category.info.name.as_str()
    }

    /// Returns the category reference that classifies this task.
    fn category(&self) -> &InfoWithEntity {
        &self.info().category
    }

    /// Returns the target entity type and identifier; the identifier may be the default value.
    fn target(&self) -> (&str, Option<i64>) {
        (
            self.info().target_entity.as_str(),
            Some(self.info().target_id.value() as i64),
        )
    }

    /// Returns the result entity type and identifier, or `None` before a result exists.
    fn result(&self) -> Option<(&str, Option<i64>)> {
        self.info()
            .result_entity
            .as_deref()
            .map(|entity| (entity, Some(self.info().result_id.value() as i64)))
    }

    /// Executes this task's domain work.
    ///
    /// Implementations update their result before returning. Failures are
    /// represented by [`TaskExecutionError`].
    fn run(&mut self) -> Result<(), TaskExecutionError>;

    /// Applies a lifecycle command and replaces the task's status message.
    ///
    /// Returns [`TaskStatusTransitionError`] when the action is invalid for
    /// the current task state.
    fn update_status(
        &mut self,
        action: TaskAction,
        message: Option<String>,
    ) -> Result<(), TaskStatusTransitionError> {
        let info = self.info_mut();
        info.status = TaskStatusTransitionRule::next(info.status, action)?;
        info.message = message;
        Ok(())
    }

    /// Stores the latest human-readable status message for the task.
    fn update_message(&mut self, message: String) {
        self.info_mut().message = Some(message);
    }
}
