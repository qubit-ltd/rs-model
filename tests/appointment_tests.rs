// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================

use qubit_model::appointment::Appointment;
use qubit_model::commons::App;
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn test_appointment_preserves_all_java_fields_and_traits() {
    assert_eq!(metadata_of::<Appointment>().struct_fields().len(), 11);
    assert_redact::<Appointment>();
}

#[test]
fn test_appointment_preserves_primary_key_and_app_reference() {
    let metadata = metadata_of::<Appointment>();
    assert_eq!(
        metadata
            .primary_key()
            .expect("appointment primary key")
            .fields()
            .iter()
            .map(|field| field.name())
            .collect::<Vec<_>>(),
        ["id"]
    );
    assert!(
        metadata
            .field("objective_id")
            .unwrap()
            .reference()
            .is_none()
    );
    let app = metadata
        .field("app")
        .unwrap()
        .reference()
        .expect("app reference");
    assert_eq!(
        app.target().identity().type_name(),
        core::any::type_name::<App>()
    );
}
