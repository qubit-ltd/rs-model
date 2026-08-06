// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::Utc;
use qubit_mixin::InfoWithEntity;
use qubit_model::task::{
    Task,
    TaskAction,
    TaskExecutionError,
    TaskInfo,
    TaskPipeline,
    TaskPipelineStatus,
    TaskStatus,
    TaskStatusTransitionRule,
};
use serde_json::{
    Value,
    json,
};

struct TestTask {
    info: TaskInfo,
}

impl Task for TestTask {
    fn info(&self) -> &TaskInfo {
        &self.info
    }

    fn info_mut(&mut self) -> &mut TaskInfo {
        &mut self.info
    }

    fn run(&mut self) -> Result<(), TaskExecutionError> {
        Ok(())
    }
}

fn task_info() -> TaskInfo {
    TaskInfo {
        id: Some(3),
        category: InfoWithEntity::default(),
        target_entity: "Order".into(),
        target_id: Some(5),
        result_entity: Some("Invoice".into()),
        result_id: Some(8),
        description: None,
        status: TaskStatus::Created,
        message: None,
        submit_time: None,
        start_time: None,
        cancel_time: None,
        finish_time: None,
        creator: None,
        create_time: Utc::now(),
        modify_time: None,
    }
}

#[test]
fn test_next_accepts_complete_success_path() {
    let submitted =
        TaskStatusTransitionRule::next(TaskStatus::Created, TaskAction::Submit)
            .expect("a created task can be submitted");
    let initializing =
        TaskStatusTransitionRule::next(submitted, TaskAction::Init)
            .expect("a submitted task can initialize");
    let running =
        TaskStatusTransitionRule::next(initializing, TaskAction::Start)
            .expect("an initializing task can start");
    let completed =
        TaskStatusTransitionRule::next(running, TaskAction::Success)
            .expect("a running task can complete");

    assert_eq!(completed, TaskStatus::Completed);
}

#[test]
fn test_next_rejects_invalid_and_terminal_transition() {
    let invalid = TaskStatusTransitionRule::next(
        TaskStatus::Created,
        TaskAction::Success,
    )
    .expect_err("a created task cannot complete directly");
    assert_eq!(invalid.status, TaskStatus::Created);
    assert_eq!(invalid.action, TaskAction::Success);

    assert!(
        TaskStatusTransitionRule::next(TaskStatus::Completed, TaskAction::Fail)
            .is_err()
    );
}

#[test]
fn test_task_default_methods_project_metadata_and_update_status() {
    let mut task = TestTask { info: task_info() };
    assert_eq!(task.id(), Some(3));
    assert_eq!(task.name(), "");
    assert_eq!(task.category(), &InfoWithEntity::default());
    assert_eq!(task.target(), ("Order", Some(5)));
    assert_eq!(task.result(), Some(("Invoice", Some(8))));
    task.update_status(TaskAction::Submit, Some("queued".into()))
        .expect("created task accepts submission");
    assert_eq!(task.info.status, TaskStatus::Submitted);
    assert_eq!(task.info.message.as_deref(), Some("queued"));
    task.update_message("initialized".into());
    assert_eq!(task.info.message.as_deref(), Some("initialized"));
    task.info.result_entity = None;
    assert_eq!(task.result(), None);
    assert!(task.run().is_ok());
}

#[test]
fn test_next_covers_all_legal_failure_and_cancellation_transitions() {
    for (status, action, expected) in [
        (TaskStatus::Created, TaskAction::Fail, TaskStatus::Failed),
        (TaskStatus::Submitted, TaskAction::Fail, TaskStatus::Failed),
        (
            TaskStatus::Initializing,
            TaskAction::Cancel,
            TaskStatus::Cancelled,
        ),
        (
            TaskStatus::Initializing,
            TaskAction::Fail,
            TaskStatus::Failed,
        ),
        (
            TaskStatus::Running,
            TaskAction::Cancel,
            TaskStatus::Cancelled,
        ),
        (TaskStatus::Running, TaskAction::Fail, TaskStatus::Failed),
    ] {
        assert_eq!(
            TaskStatusTransitionRule::next(status, action)
                .expect("listed transition must be legal"),
            expected
        );
    }
}

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
    for status in [
        TaskPipelineStatus::Idle,
        TaskPipelineStatus::Running,
        TaskPipelineStatus::Paused,
        TaskPipelineStatus::Failed,
        TaskPipelineStatus::Finished,
        TaskPipelineStatus::Cancelled,
    ] {
        pipeline.status = status;
        assert_eq!(
            pipeline.is_finished(),
            status == TaskPipelineStatus::Finished
        );
        assert_eq!(pipeline.is_failed(), status == TaskPipelineStatus::Failed);
        assert_eq!(status.is_idle(), status == TaskPipelineStatus::Idle);
        assert_eq!(status.is_started(), status != TaskPipelineStatus::Idle);
        assert_eq!(status.is_running(), status == TaskPipelineStatus::Running);
        assert_eq!(status.is_paused(), status == TaskPipelineStatus::Paused);
        assert_eq!(status.is_failed(), status == TaskPipelineStatus::Failed);
        assert_eq!(
            status.is_finished(),
            status == TaskPipelineStatus::Finished
        );
        assert_eq!(
            status.is_cancelled(),
            status == TaskPipelineStatus::Cancelled
        );
    }
}
