// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_mixin::Normalizable;
use qubit_model::Field;
use qubit_model::commons::App;
use qubit_model::commons::AppResource;
use qubit_model::commons::Code;
use qubit_model::commons::CodeMap;
use qubit_model::commons::Faq;
use qubit_model::commons::MqFailedTask;
use qubit_model::commons::Schedule;
use qubit_model::commons::State;
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

/// Requires the migrated type to expose the diagnostic redaction contract.
fn assert_redact<T: Redact>() {}

#[test]
fn test_remaining_common_models_preserve_source_shapes_and_traits() {
    assert_redact::<Field>();
    assert_redact::<AppResource>();
    assert_redact::<Code>();
    assert_redact::<CodeMap>();
    assert_redact::<Faq>();
    assert_redact::<MqFailedTask>();
    assert_redact::<Schedule>();

    assert_eq!(metadata_of::<AppResource>().struct_fields().len(), 7);
    assert_eq!(metadata_of::<Code>().struct_fields().len(), 3);
    assert_eq!(metadata_of::<CodeMap>().struct_fields().len(), 7);
    assert_eq!(metadata_of::<Faq>().struct_fields().len(), 11);
    assert_eq!(metadata_of::<MqFailedTask>().struct_fields().len(), 9);
    assert_eq!(metadata_of::<Schedule>().struct_fields().len(), 3);
}

#[test]
fn test_remaining_common_metadata_preserves_source_constraints() {
    let resource = metadata_of::<AppResource>();
    assert_eq!(
        resource.primary_key().expect("primary key").fields()[0].name(),
        "id"
    );
    assert_eq!(
        resource
            .field("app_id")
            .expect("app_id field")
            .reference()
            .expect("app reference")
            .target()
            .identity()
            .type_name(),
        core::any::type_name::<App>()
    );
    for field in [
        "resource_type",
        "resource_id",
        "create_time",
        "modify_time",
        "delete_time",
    ] {
        assert!(resource.indexes().any(|index| index.contains(field)));
    }
}

#[test]
fn test_field_and_faq_preserve_source_values_and_defaults() {
    assert_eq!(
        Field::ActualCredentialNumber.code(),
        "actual_credential_number"
    );
    assert_eq!(
        serde_json::to_string(&Field::MedicalChargeName)
            .expect("serialize field"),
        "\"MEDICAL_CHARGE_NAME\""
    );

    let faq = Faq::default();
    assert_eq!(faq.frequency, 0);
    assert_eq!(faq.state, State::Normal);
}

#[test]
fn test_code_and_schedule_preserve_source_normalization_and_emptiness() {
    let mut code = Code {
        standard: Some("  ".into()),
        code: " value ".into(),
        ..Code::default()
    };
    code.normalize();
    assert_eq!(code.standard, None);
    assert_eq!(code.code, "value");
    assert!(!code.is_empty());

    let mut schedule = Schedule {
        crontabs: Some(vec![" 0 0 * * * * ".into()]),
        ..Schedule::default()
    };
    schedule.normalize();
    assert_eq!(
        schedule.crontabs.as_deref(),
        Some(["0 0 * * * *".into()].as_slice())
    );
    assert!(!schedule.is_empty());
}

#[test]
fn test_failed_mq_task_redacts_source_message_material() {
    let task = MqFailedTask {
        message_value: "private-payload".into(),
        ..MqFailedTask::default()
    };
    assert!(!format!("{:?}", task.redacted()).contains("private-payload"));
}
