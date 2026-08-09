// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration-test mirror for the corresponding public model module.

use qubit_model::privilege::Role;
use qubit_model::privilege::UserRole;
use qubit_model_metadata::metadata_of;

/// Keeps the source-to-test mapping explicit while shared model contract tests
/// exercise serialization, metadata, and redaction behavior.
#[test]
fn test_role_tests_mirror() {
    assert!(
        !module_path!().is_empty(),
        "the test module path must be available"
    );
}

#[test]
fn test_privilege_models_preserve_source_metadata() {
    let role = metadata_of::<Role>();
    assert_eq!(role.unique_constraints().count(), 2);
    for field in [
        "app",
        "code",
        "name",
        "guest",
        "basic",
        "state",
        "create_time",
        "modify_time",
        "delete_time",
    ] {
        assert!(
            role.indexes().any(|index| index.contains(field)),
            "missing role index for {field}"
        );
    }
    assert!(role.field("app").unwrap().reference().is_some());

    let user_role = metadata_of::<UserRole>();
    assert_eq!(user_role.unique_constraints().count(), 1);
    for field in ["user", "app", "role", "create_time"] {
        assert!(
            user_role.indexes().any(|index| index.contains(field)),
            "missing user role index for {field}"
        );
        assert!(user_role.field(field).unwrap().reference().is_some() || field == "create_time");
    }
}
