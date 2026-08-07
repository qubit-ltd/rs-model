// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioral coverage for complete employee records.

use qubit_mixin::Emptyful;
use qubit_mixin::Normalizable;
use qubit_model::contact::Phone;
use qubit_model::organization::Employee;

/// Verifies employee projection, assignment, empty checks, and normalization.
#[test]
fn test_employee_info_assignment_empty_and_normalization() {
    let mut employee = Employee::default();
    assert!(employee.is_empty());
    assert!(Emptyful::is_empty(&employee));

    employee.code = "  EMP-1  ".into();
    employee.name = "  Alice  ".into();
    employee.mobile = Phone::from(" 13800138000 ");
    employee.email = Some("  alice@example.test  ".into());
    employee.comment = Some("  note  ".into());
    employee.normalize();
    assert_eq!(employee.code, "EMP-1");
    assert_eq!(employee.name, "Alice");
    assert_eq!(employee.mobile.number, "13800138000");
    assert_eq!(employee.email.as_deref(), Some("alice@example.test"));
    assert_eq!(employee.comment.as_deref(), Some("note"));

    let info = employee.info();
    let mut replacement = Employee::default();
    replacement.assign_info(&info);
    assert_eq!(replacement.info(), info);
    assert!(!replacement.is_empty());
}
