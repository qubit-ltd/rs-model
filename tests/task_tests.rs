use qubit_model::task::{TaskAction, TaskStatus, TaskStatusTransitionRule};

#[test]
fn test_next_accepts_complete_success_path() {
    let submitted = TaskStatusTransitionRule::next(TaskStatus::Created, TaskAction::Submit)
        .expect("a created task can be submitted");
    let initializing = TaskStatusTransitionRule::next(submitted, TaskAction::Init)
        .expect("a submitted task can initialize");
    let running = TaskStatusTransitionRule::next(initializing, TaskAction::Start)
        .expect("an initializing task can start");
    let completed = TaskStatusTransitionRule::next(running, TaskAction::Success)
        .expect("a running task can complete");

    assert_eq!(completed, TaskStatus::Completed);
}

#[test]
fn test_next_rejects_invalid_and_terminal_transition() {
    let invalid = TaskStatusTransitionRule::next(TaskStatus::Created, TaskAction::Success)
        .expect_err("a created task cannot complete directly");
    assert_eq!(invalid.status, TaskStatus::Created);
    assert_eq!(invalid.action, TaskAction::Success);

    assert!(TaskStatusTransitionRule::next(TaskStatus::Completed, TaskAction::Fail).is_err());
}
