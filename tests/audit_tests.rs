// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::audit::AuditStatus;

#[test]
fn test_audit_status_preserves_source_wire_value() {
    assert_eq!(
        serde_json::to_string(&AuditStatus::Submitted)
            .expect("audit status serializes"),
        "\"SUBMITTED\""
    );
}
