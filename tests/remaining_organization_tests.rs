// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::{
    contact::Phone,
    organization::{Employee, Organization},
    person::SocialNetworkAccount,
    product::Seller,
    system::Session,
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

/// Requires the migrated type to expose diagnostic redaction.
fn assert_redact<T: Redact>() {}

#[test]
fn test_remaining_organization_models_preserve_source_shapes_and_traits() {
    assert_redact::<Employee>();
    assert_redact::<Organization>();
    assert_redact::<SocialNetworkAccount>();

    assert_eq!(metadata_of::<Employee>().struct_fields().len(), 29);
    assert_eq!(metadata_of::<Organization>().struct_fields().len(), 21);
    assert_eq!(
        metadata_of::<SocialNetworkAccount>().struct_fields().len(),
        12
    );
}

#[test]
fn test_organization_metadata_preserves_uniques_references_and_indexes() {
    let organization = metadata_of::<Organization>();
    assert_eq!(
        organization.primary_key().expect("primary key").fields()[0].name(),
        "id"
    );
    assert_eq!(organization.unique_constraints().count(), 2);
    assert!(
        organization
            .field("parent")
            .expect("parent field")
            .reference()
            .is_some()
    );

    let employee = metadata_of::<Employee>();
    assert_eq!(employee.unique_constraints().count(), 4);
    for field in ["username", "person_id", "organization", "mobile", "state"] {
        assert!(employee.indexes().any(|index| index.contains(field)));
    }

    let session_reference = metadata_of::<Session>()
        .field("organization")
        .expect("organization field")
        .reference()
        .expect("organization reference");
    assert_eq!(
        session_reference.target().identity().type_name(),
        core::any::type_name::<Organization>()
    );
}

#[test]
fn test_employee_projects_and_assigns_source_info() {
    let mut employee = Employee {
        id: Some(7),
        code: "EMP-7".into(),
        name: "Ada".into(),
        ..Employee::default()
    };
    let info = employee.info();
    assert_eq!(info.id, Some(7));
    assert_eq!(info.code, "EMP-7");

    let mut replacement = info;
    replacement.id = Some(8);
    replacement.name = "Grace".into();
    employee.assign_info(&replacement);
    assert_eq!(employee.id, Some(8));
    assert_eq!(employee.name, "Grace");
}

#[test]
fn test_employee_and_organization_empty_checks_and_normalization() {
    use qubit_mixin::{Emptyful, Normalizable};

    let mut employee = Employee::default();
    assert!(employee.is_empty());
    assert!(Emptyful::is_empty(&employee));
    employee.code = "  EMP-1  ".into();
    employee.name = "  Alice  ".into();
    employee.mobile = Phone::from(" 13800138000 ");
    employee.email = Some("  alice@example.test  ".into());
    employee.normalize();
    assert_eq!(employee.code, "EMP-1");
    assert_eq!(employee.mobile.number, "13800138000");
    assert!(!employee.is_normalized_empty());

    let mut organization = Organization::default();
    assert!(organization.is_empty());
    assert!(Emptyful::is_empty(&organization));
    organization.code = "  seller  ".into();
    organization.name = "  Seller name  ".into();
    organization.normalize();
    assert_eq!(organization.code, "seller");
    assert_eq!(organization.name, "Seller name");
    assert_eq!(organization.info().code, "seller");
    assert!(!organization.is_normalized_empty());
}

#[test]
fn test_organization_assigns_seller_source_view() {
    let seller = Seller {
        id: Some(3),
        code: "SELLER".into(),
        name: "Seller Name".into(),
        phone: Some(Phone::from("010-123")),
        mobile: Some(Phone::from("13800138000")),
        email: Some("seller@example.test".into()),
        url: Some("https://seller.example.test".into()),
        credential: None,
        address: None,
    };
    let mut organization = Organization::default();
    organization.assign_seller(&seller);
    assert_eq!(organization.id, Some(3));
    assert_eq!(organization.code, "SELLER");
    assert_eq!(
        organization
            .contact
            .as_ref()
            .and_then(|contact| contact.email.as_deref()),
        Some("seller@example.test")
    );
}

#[test]
fn test_social_network_account_redacts_open_id_and_profiles() {
    let account = SocialNetworkAccount {
        open_id: "private-open-id".into(),
        ..SocialNetworkAccount::default()
    };
    assert!(!format!("{:?}", account.redacted()).contains("private-open-id"));
}
