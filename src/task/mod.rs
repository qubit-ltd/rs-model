// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Task execution models and transition rules.

#[allow(clippy::module_inception)]
mod task;
mod task_action;
mod task_execution_error;
mod task_info;
mod task_pipeline;
mod task_pipeline_status;
mod task_statistics;
mod task_status;
mod task_status_transition_error;
mod task_status_transition_rule;

pub use task::Task;
pub use task_action::TaskAction;
pub use task_execution_error::TaskExecutionError;
pub use task_info::TaskInfo;
pub use task_pipeline::TaskPipeline;
pub use task_pipeline_status::TaskPipelineStatus;
pub use task_statistics::TaskStatistics;
pub use task_status::TaskStatus;
pub use task_status_transition_error::TaskStatusTransitionError;
pub use task_status_transition_rule::TaskStatusTransitionRule;
