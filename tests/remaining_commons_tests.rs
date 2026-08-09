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
use qubit_model::commons::Category;
use qubit_model::commons::Code;
use qubit_model::commons::CodeMap;
use qubit_model::commons::Credential;
use qubit_model::commons::Faq;
use qubit_model::commons::MqFailedTask;
use qubit_model::commons::Payload;
use qubit_model::commons::Schedule;
use qubit_model::commons::Source;
use qubit_model::commons::State;
use qubit_model::commons::Token;
use qubit_model_metadata::UniqueComparison;
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
fn test_common_model_metadata_preserves_java_keys_indexes_and_references() {
    let app = metadata_of::<App>();
    assert!(
        app.unique_constraints()
            .any(|unique| unique.contains("code"))
    );
    assert!(
        app.unique_constraints()
            .any(|unique| unique.contains("organization") && unique.contains("name"))
    );
    for field in [
        "name",
        "organization",
        "category",
        "state",
        "last_authorize",
        "predefined",
        "create_time",
        "modify_time",
        "delete_time",
    ] {
        assert!(app.indexes().any(|index| index.contains(field)));
    }
    assert!(app.field("organization").unwrap().reference().is_some());
    assert!(app.field("category").unwrap().reference().is_some());
    assert!(app.field("default_user").unwrap().reference().is_some());

    let category = metadata_of::<Category>();
    assert!(
        category
            .unique_constraints()
            .any(|unique| unique.contains("code"))
    );
    assert!(
        category
            .unique_constraints()
            .any(|unique| unique.contains("entity") && unique.contains("name"))
    );
    assert!(category.field("parent").unwrap().reference().is_some());
    for field in [
        "entity",
        "name",
        "parent",
        "predefined",
        "create_time",
        "modify_time",
        "delete_time",
    ] {
        assert!(category.indexes().any(|index| index.contains(field)));
    }

    let credential = metadata_of::<Credential>();
    let credential_number = credential
        .unique_constraints()
        .find(|unique| {
            unique.contains("owner") && unique.contains("type") && unique.contains("number")
        })
        .expect("credential number must be unique within owner and type");
    assert_eq!(
        credential_number.comparison_of("number"),
        Some(UniqueComparison::IgnoreCase)
    );
    for field in [
        "owner",
        "type",
        "verified",
        "index",
        "title",
        "create_time",
        "modify_time",
        "delete_time",
    ] {
        assert!(credential.indexes().any(|index| index.contains(field)));
    }
    let attachments = credential
        .field("attachments")
        .expect("credential attachments metadata");
    assert!(attachments.reference().is_some());
    let attachment_size = attachments
        .sequence_constraint()
        .expect("credential attachment size metadata");
    assert_eq!(attachment_size.min_items(), Some(1));
    assert_eq!(attachment_size.max_items(), Some(16));

    let payload = metadata_of::<Payload>();
    assert!(
        payload
            .unique_constraints()
            .any(|unique| unique.contains("owner") && unique.contains("key"))
    );

    let source = metadata_of::<Source>();
    assert!(
        source
            .unique_constraints()
            .any(|unique| unique.contains("code"))
    );
    assert!(source.unique_constraints().any(|unique| {
        unique.contains("app") && unique.contains("entity") && unique.contains("name")
    }));
    for field in [
        "app",
        "entity",
        "predefined",
        "create_time",
        "modify_time",
        "delete_time",
    ] {
        assert!(source.indexes().any(|index| index.contains(field)));
    }
    for field in ["app", "category", "provider_app", "provider_organization"] {
        assert!(source.field(field).unwrap().reference().is_some());
    }

    let token = metadata_of::<Token>();
    assert!(
        token
            .unique_constraints()
            .any(|unique| unique.contains("value"))
    );
    assert!(
        token
            .unique_constraints()
            .any(|unique| unique.contains("previous_value"))
    );
    for field in ["create_time", "max_age"] {
        assert!(token.indexes().any(|index| index.contains(field)));
    }
}

#[test]
fn test_field_and_faq_preserve_source_values_and_defaults() {
    assert_eq!(
        Field::ActualCredentialNumber.code(),
        "actual_credential_number"
    );
    assert_eq!(
        serde_json::to_string(&Field::MedicalChargeName).expect("serialize field"),
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
