// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Contract for ordered execution, suspension, and cancellation of domain tasks.

use serde_json::Value;

use super::Task;
use super::TaskPipelineStatus;

/// An ordered collection of executable tasks controlled as one pipeline.
///
/// Target and configuration values use JSON values because the Java source
/// accepts arbitrary `Identifiable` and `Config` implementations that have no
/// single concrete Rust representation.
pub trait TaskPipeline {
    /// Returns the pipeline's current lifecycle status.
    fn status(&self) -> TaskPipelineStatus;

    /// Returns the JSON value identifying the pipeline's target.
    fn target(&self) -> &Value;

    /// Replaces the pipeline target with the supplied JSON value.
    fn set_target(&mut self, target: Value);

    /// Returns the JSON configuration used by the pipeline.
    fn config(&self) -> &Value;

    /// Appends a task to the execution order.
    fn add(&mut self, task: Box<dyn Task>);

    /// Returns whether no task in the pipeline has started.
    fn is_idle(&self) -> bool {
        self.status().is_idle()
    }

    /// Returns whether the pipeline has left its initial idle state.
    fn is_started(&self) -> bool {
        self.status().is_started()
    }

    /// Returns whether the pipeline is actively executing a task.
    fn is_running(&self) -> bool {
        self.status().is_running()
    }

    /// Returns whether execution is paused.
    fn is_paused(&self) -> bool {
        self.status().is_paused()
    }

    /// Returns whether every task completed successfully.
    fn is_finished(&self) -> bool {
        self.status().is_finished()
    }

    /// Returns whether the pipeline ended because a task failed.
    fn is_failed(&self) -> bool {
        self.status().is_failed()
    }

    /// Returns whether the pipeline was explicitly cancelled.
    fn is_cancelled(&self) -> bool {
        self.status().is_cancelled()
    }

    /// Starts execution at the first queued task.
    fn start(&mut self);

    /// Returns the active task, or `None` when the pipeline is idle or terminal.
    fn current(&self) -> Option<&dyn Task>;

    /// Advances execution from the current task to the next task.
    fn next(&mut self);

    /// Suspends execution without discarding the current pipeline state.
    fn pause(&mut self);

    /// Resumes a pipeline previously suspended by `pause`.
    fn resume(&mut self);

    /// Cancels the pipeline and prevents further task execution.
    fn cancel(&mut self);
}
