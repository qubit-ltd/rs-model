// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! External test mirror for the corresponding public model source.

/// Confirms this mirrored integration test module is discoverable.
#[test]
fn test_task_pipeline_status_tests_mirror() {
    assert!(!module_path!().is_empty(), "the test module path must be available");
}
