// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::person::PersonInfo;
use qubit_redact::Redact as _;

#[test]
fn test_person_info_redacts_email() {
    let person: PersonInfo = serde_json::from_value(serde_json::json!({
        "id": 1,
        "name": "Ada",
        "username": "ada",
        "gender": "FEMALE",
        "birthday": null,
        "credential": null,
        "mobile": null,
        "email": "ada@example.test",
        "photo": null,
        "test": false,
        "delete_time": null
    }))
    .expect("person information should deserialize");

    let redacted = format!("{:?}", person.redacted());
    assert!(!redacted.contains("ada@example.test"));
    assert!(redacted.contains("Ada"));
}
