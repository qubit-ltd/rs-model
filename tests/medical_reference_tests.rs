// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for reusable medical reference models.

use qubit_model::medical::{
    Disease, Dosage, DrugInfo, DrugProduct, HospitalDrugstore, PatientInfo,
};
use qubit_model_metadata::metadata_of;

/// Verifies medical reference structs retain every Java source field.
#[test]
fn test_medical_reference_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<Disease>().struct_fields().len(), 11);
    assert_eq!(metadata_of::<Dosage>().struct_fields().len(), 10);
    assert_eq!(metadata_of::<DrugInfo>().struct_fields().len(), 16);
    assert_eq!(metadata_of::<DrugProduct>().struct_fields().len(), 2);
    assert_eq!(metadata_of::<HospitalDrugstore>().struct_fields().len(), 6);
    assert_eq!(metadata_of::<PatientInfo>().struct_fields().len(), 8);
}
