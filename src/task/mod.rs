//! Task execution models and transition rules.

mod task_info;
#[allow(clippy::module_inception)]
mod task;
mod task_pipeline;
mod task_statistics;
mod task_status_transition_rule;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

pub use task_info::TaskInfo;
pub use task::Task;
pub use task_pipeline::TaskPipeline;
pub use task_statistics::TaskStatistics;
pub use task_status_transition_rule::{TaskStatusTransitionError, TaskStatusTransitionRule};

/// An operation applied to a task.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
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
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskPipelineStatus {
    Idle,
    Running,
    Paused,
    Failed,
    Finished,
    Cancelled,
}

impl TaskPipelineStatus {
    /// Reports whether this status is idle.
    #[must_use]
    pub const fn is_idle(self) -> bool {
        matches!(self, Self::Idle)
    }

    /// Reports whether this status has left idle.
    #[must_use]
    pub const fn is_started(self) -> bool {
        !self.is_idle()
    }

    /// Reports whether this status is running.
    #[must_use]
    pub const fn is_running(self) -> bool {
        matches!(self, Self::Running)
    }

    /// Reports whether this status is paused.
    #[must_use]
    pub const fn is_paused(self) -> bool {
        matches!(self, Self::Paused)
    }

    /// Reports whether this status is finished.
    #[must_use]
    pub const fn is_finished(self) -> bool {
        matches!(self, Self::Finished)
    }

    /// Reports whether this status is failed.
    #[must_use]
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::Failed)
    }

    /// Reports whether this status is cancelled.
    #[must_use]
    pub const fn is_cancelled(self) -> bool {
        matches!(self, Self::Cancelled)
    }
}

/// An execution failure reported by a task implementation.
#[derive(Clone, Debug, thiserror::Error, PartialEq, Eq)]
#[error("task execution failed: {message}")]
pub struct TaskExecutionError {
    /// Human-readable failure message.
    pub message: String,
}
