// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration-test mirror for the corresponding public model module.

/// Keeps the source-to-test mapping explicit while shared model contract tests
/// exercise serialization, metadata, and redaction behavior.
#[test]
fn test_expired_reason_tests_mirror() {
    assert!(!module_path!().is_empty(), "the test module path must be available");
}
