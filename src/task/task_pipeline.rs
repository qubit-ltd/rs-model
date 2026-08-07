// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Task-pipeline execution interface.

use serde_json::Value;

use super::Task;
use super::TaskPipelineStatus;

/// An ordered pipeline of executable tasks.
///
/// Target and configuration values use JSON values because the Java source
/// accepts arbitrary `Identifiable` and `Config` implementations that have no
/// single concrete Rust representation.
pub trait TaskPipeline {
    /// Returns the pipeline's current execution status.
    fn status(&self) -> TaskPipelineStatus;

    /// Returns the pipeline target value.
    fn target(&self) -> &Value;

    /// Replaces the pipeline target value.
    fn set_target(&mut self, target: Value);

    /// Returns the pipeline configuration.
    fn config(&self) -> &Value;

    /// Appends a task to the pipeline.
    fn add(&mut self, task: Box<dyn Task>);

    /// Reports whether the pipeline has not started.
    fn is_idle(&self) -> bool {
        self.status().is_idle()
    }

    /// Reports whether the pipeline has left its idle state.
    fn is_started(&self) -> bool {
        self.status().is_started()
    }

    /// Reports whether the pipeline is running.
    fn is_running(&self) -> bool {
        self.status().is_running()
    }

    /// Reports whether the pipeline is paused.
    fn is_paused(&self) -> bool {
        self.status().is_paused()
    }

    /// Reports whether the pipeline completed successfully.
    fn is_finished(&self) -> bool {
        self.status().is_finished()
    }

    /// Reports whether the pipeline failed.
    fn is_failed(&self) -> bool {
        self.status().is_failed()
    }

    /// Reports whether the pipeline was cancelled.
    fn is_cancelled(&self) -> bool {
        self.status().is_cancelled()
    }

    /// Starts execution at the first task.
    fn start(&mut self);

    /// Returns the currently executing task, or `None` when no task is active.
    fn current(&self) -> Option<&dyn Task>;

    /// Advances execution to the next task.
    fn next(&mut self);

    /// Pauses pipeline execution.
    fn pause(&mut self);

    /// Resumes a paused pipeline.
    fn resume(&mut self);

    /// Cancels pipeline execution.
    fn cancel(&mut self);
}
