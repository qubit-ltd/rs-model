// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! External test mirror for setting values.

/// Confirms the setting-value integration test module is discoverable.
#[test]
fn test_setting_value_mirror() {
    assert!(!module_path!().is_empty(), "the test module path must be available");
}
