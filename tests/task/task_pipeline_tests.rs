// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioral coverage for task-pipeline status helpers.

use qubit_model::task::{Task, TaskPipeline, TaskPipelineStatus};
use serde_json::{Value, json};

struct TestPipeline {
    status: TaskPipelineStatus,
    target: Value,
    config: Value,
}

impl TaskPipeline for TestPipeline {
    fn status(&self) -> TaskPipelineStatus {
        self.status
    }

    fn target(&self) -> &Value {
        &self.target
    }

    fn set_target(&mut self, target: Value) {
        self.target = target;
    }

    fn config(&self) -> &Value {
        &self.config
    }

    fn add(&mut self, _task: Box<dyn Task>) {}

    fn start(&mut self) {
        self.status = TaskPipelineStatus::Running;
    }

    fn current(&self) -> Option<&dyn Task> {
        None
    }

    fn next(&mut self) {}

    fn pause(&mut self) {
        self.status = TaskPipelineStatus::Paused;
    }

    fn resume(&mut self) {
        self.status = TaskPipelineStatus::Running;
    }

    fn cancel(&mut self) {
        self.status = TaskPipelineStatus::Cancelled;
    }
}

/// Delegates pipeline state predicates and implementation hooks correctly.
#[test]
fn test_task_pipeline_status_helpers_and_hooks() {
    let mut pipeline = TestPipeline {
        status: TaskPipelineStatus::Idle,
        target: json!("old"),
        config: json!({"retry": 1}),
    };
    assert!(pipeline.is_idle());
    assert!(!pipeline.is_started());
    pipeline.set_target(json!("new"));
    assert_eq!(pipeline.target(), &json!("new"));
    assert_eq!(pipeline.config(), &json!({"retry": 1}));
    assert!(pipeline.current().is_none());
    pipeline.start();
    assert!(pipeline.is_running());
    pipeline.pause();
    assert!(pipeline.is_paused());
    pipeline.resume();
    assert!(pipeline.is_running());
    pipeline.cancel();
    assert!(pipeline.is_cancelled());

    for status in [TaskPipelineStatus::Failed, TaskPipelineStatus::Finished] {
        pipeline.status = status;
        assert_eq!(pipeline.is_failed(), status == TaskPipelineStatus::Failed);
        assert_eq!(
            pipeline.is_finished(),
            status == TaskPipelineStatus::Finished
        );
        assert!(pipeline.is_started());
    }
}
