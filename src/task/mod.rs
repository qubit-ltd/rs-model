//! Task execution models and transition rules.

mod task_info;
mod task_statistics;
mod task_status_transition_rule;

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

pub use task_info::TaskInfo;
pub use task_statistics::TaskStatistics;
pub use task_status_transition_rule::{TaskStatusTransitionError, TaskStatusTransitionRule};

/// An operation applied to a task.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskAction {
    Submit,
    Init,
    Start,
    Cancel,
    Fail,
    Success,
}

/// A task lifecycle state.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskStatus {
    Created,
    Submitted,
    Initializing,
    Running,
    Failed,
    Completed,
    Cancelled,
}

/// The execution status of a task pipeline.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskPipelineStatus {
    Idle,
    Running,
    Paused,
    Failed,
    Finished,
    Cancelled,
}

/// Behaviour required from an executable task.
pub trait Task {
    /// Returns the task metadata.
    fn info(&self) -> &TaskInfo;

    /// Mutably returns the task metadata.
    fn info_mut(&mut self) -> &mut TaskInfo;

    /// Executes the task's domain work.
    ///
    /// Implementations update their result before returning. Failures are represented by the
    /// implementation-specific error value.
    fn run(&mut self) -> Result<(), TaskExecutionError>;

    /// Applies a lifecycle action and records its optional message.
    ///
    /// Returns an error when the action is invalid for the current task state.
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
}

/// An execution failure reported by a task implementation.
#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
#[error("task execution failed: {message}")]
pub struct TaskExecutionError {
    /// Human-readable failure message.
    pub message: String,
}
