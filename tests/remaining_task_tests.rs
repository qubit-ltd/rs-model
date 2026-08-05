// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::task::{Task, TaskInfo, TaskPipeline, TaskPipelineStatus};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

/// Requires a type implementing the source task interface.
fn assert_task<T: Task + ?Sized>() {}

/// Requires a type implementing the source pipeline interface.
fn assert_pipeline<T: TaskPipeline + ?Sized>() {}

/// Requires diagnostic redaction for persisted task information.
fn assert_redact<T: Redact>() {}

#[test]
fn test_task_interfaces_are_public_and_task_info_is_redactable() {
    assert_task::<dyn Task>();
    assert_pipeline::<dyn TaskPipeline>();
    assert_redact::<TaskInfo>();
    assert_eq!(metadata_of::<TaskInfo>().struct_fields().len(), 16);
}

#[test]
fn test_task_pipeline_status_preserves_source_state_queries() {
    assert!(TaskPipelineStatus::Idle.is_idle());
    assert!(!TaskPipelineStatus::Idle.is_started());
    assert!(TaskPipelineStatus::Running.is_running());
    assert!(TaskPipelineStatus::Paused.is_paused());
    assert!(TaskPipelineStatus::Finished.is_finished());
    assert!(TaskPipelineStatus::Failed.is_failed());
    assert!(TaskPipelineStatus::Cancelled.is_cancelled());
}
