// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::audit::Audit;
use qubit_model::audit::AuditStatus;
use qubit_model::organization::Employee;
use qubit_model_metadata::metadata_of;

#[test]
fn test_audit_status_preserves_source_wire_value() {
    assert_eq!(
        serde_json::to_string(&AuditStatus::Submitted).expect("audit status serializes"),
        "\"SUBMITTED\""
    );
}

/// Verifies the audit request retains the Java identifier, text, and reviewer
/// reference metadata.
#[test]
fn test_audit_request_metadata_matches_java_contract() {
    let metadata = metadata_of::<Audit>();
    assert_eq!(
        metadata
            .primary_key()
            .expect("audit request primary key")
            .fields()
            .iter()
            .map(|field| field.name())
            .collect::<Vec<_>>(),
        ["id"]
    );
    assert_eq!(
        metadata
            .field("objective_type")
            .expect("objective type field")
            .text_constraint()
            .expect("objective type size constraint")
            .repertoire(),
        qubit_model_metadata::TextRepertoire::Unicode
    );
    let auditor = metadata
        .field("auditor")
        .expect("auditor field")
        .reference()
        .expect("auditor reference");
    assert_eq!(
        auditor.target().identity().type_name(),
        core::any::type_name::<Employee>()
    );
    assert_eq!(auditor.target_field().segments(), ["info"]);
    assert!(auditor.must_exist());
}
