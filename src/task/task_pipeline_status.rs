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

/// The execution status of a task pipeline.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskPipelineStatus {
    /// The Idle classification.
    Idle,
    /// The Running classification.
    Running,
    /// The Paused classification.
    Paused,
    /// The Failed classification.
    Failed,
    /// The Finished classification.
    Finished,
    /// The Cancelled classification.
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
