// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for medical-service models.

use qubit_model::service::EmployeeMedicalItem;
use qubit_model::service::MedicalItem;
use qubit_model::service::MedicalItemUseRecord;
use qubit_model::service::MedicalPackage;
use qubit_model::service::MedicalPackageItem;
use qubit_model::service::UserMedicalItem;
use qubit_model::service::UserMedicalPackage;
use qubit_model_metadata::metadata_of;

/// Verifies service structs retain every Java source field.
#[test]
fn test_service_structs_expose_all_source_fields() {
    assert_eq!(
        metadata_of::<EmployeeMedicalItem>().struct_fields().len(),
        3
    );
    assert_eq!(metadata_of::<MedicalItem>().struct_fields().len(), 7);
    assert_eq!(
        metadata_of::<MedicalItemUseRecord>().struct_fields().len(),
        6
    );
    assert_eq!(metadata_of::<MedicalPackage>().struct_fields().len(), 9);
    assert_eq!(metadata_of::<MedicalPackageItem>().struct_fields().len(), 3);
    assert_eq!(metadata_of::<UserMedicalItem>().struct_fields().len(), 4);
    assert_eq!(metadata_of::<UserMedicalPackage>().struct_fields().len(), 6);
}

/// Verifies service metadata retains Java uniqueness and references.
#[test]
fn test_service_metadata_preserves_source_constraints() {
    for model in [
        metadata_of::<MedicalItem>(),
        metadata_of::<MedicalPackage>(),
    ] {
        assert!(
            model
                .unique_constraints()
                .any(|unique| unique.contains("code")),
            "the Java service code should remain unique"
        );
    }

    for (model, field) in [
        (metadata_of::<MedicalPackage>(), "organization"),
        (metadata_of::<MedicalPackageItem>(), "package_id"),
        (metadata_of::<MedicalPackageItem>(), "item"),
        (metadata_of::<UserMedicalItem>(), "medical_item_id"),
        (metadata_of::<UserMedicalPackage>(), "user_id"),
        (metadata_of::<UserMedicalPackage>(), "medical_package_id"),
    ] {
        assert!(
            model
                .field(field)
                .expect("the Java source field should exist")
                .reference()
                .is_some(),
            "missing Java reference metadata for {field}"
        );
    }
}
