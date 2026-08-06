// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioral coverage for task-pipeline lifecycle predicates.

use qubit_model::task::TaskPipelineStatus;

/// Evaluates every predicate against every lifecycle state.
#[test]
fn test_task_pipeline_status_predicates_cover_every_state() {
    for status in [
        TaskPipelineStatus::Idle,
        TaskPipelineStatus::Running,
        TaskPipelineStatus::Paused,
        TaskPipelineStatus::Failed,
        TaskPipelineStatus::Finished,
        TaskPipelineStatus::Cancelled,
    ] {
        assert_eq!(status.is_idle(), status == TaskPipelineStatus::Idle);
        assert_eq!(status.is_started(), status != TaskPipelineStatus::Idle);
        assert_eq!(status.is_running(), status == TaskPipelineStatus::Running);
        assert_eq!(status.is_paused(), status == TaskPipelineStatus::Paused);
        assert_eq!(status.is_failed(), status == TaskPipelineStatus::Failed);
        assert_eq!(status.is_finished(), status == TaskPipelineStatus::Finished);
        assert_eq!(
            status.is_cancelled(),
            status == TaskPipelineStatus::Cancelled
        );
    }
}
