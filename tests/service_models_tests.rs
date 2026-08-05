// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for medical-service models.

use qubit_model::service::{
    EmployeeMedicalItem, MedicalItem, MedicalItemUseRecord, MedicalPackage, MedicalPackageItem,
    UserMedicalItem, UserMedicalPackage,
};
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
