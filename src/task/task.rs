// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Executable task interface.

use qubit_mixin::InfoWithEntity;

use super::TaskAction;
use super::TaskExecutionError;
use super::TaskInfo;
use super::TaskStatusTransitionError;
use super::TaskStatusTransitionRule;

/// Behaviour required from an executable task.
pub trait Task {
    /// Returns the task metadata.
    fn info(&self) -> &TaskInfo;

    /// Mutably returns the task metadata.
    fn info_mut(&mut self) -> &mut TaskInfo;

    /// Returns the persisted task identifier, or `None` before persistence.
    fn id(&self) -> Option<i64> {
        Some(self.info().id.value() as i64)
    }

    /// Returns the source task name derived from its category.
    fn name(&self) -> &str {
        self.info().category.info.name.as_str()
    }

    /// Returns the category classifying this task.
    fn category(&self) -> &InfoWithEntity {
        &self.info().category
    }

    /// Returns the target entity name and optional identifier.
    fn target(&self) -> (&str, Option<i64>) {
        (
            self.info().target_entity.as_str(),
            Some(self.info().target_id.value() as i64),
        )
    }

    /// Returns the optional result entity name and identifier.
    fn result(&self) -> Option<(&str, Option<i64>)> {
        self.info()
            .result_entity
            .as_deref()
            .map(|entity| (entity, Some(self.info().result_id.value() as i64)))
    }

    /// Executes the task's domain work.
    ///
    /// Implementations update their result before returning. Failures are
    /// represented by [`TaskExecutionError`].
    fn run(&mut self) -> Result<(), TaskExecutionError>;

    /// Applies a lifecycle action and records its optional message.
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

    /// Replaces the task's latest status message.
    fn update_message(&mut self, message: String) {
        self.info_mut().message = Some(message);
    }
}
