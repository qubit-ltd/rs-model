// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for prescription and medical-settlement models.

use qubit_model::medical::Diagnosis;
use qubit_model::medical::MedicalSettlement;
use qubit_model::medical::Prescription;
use qubit_model::medical::PrescriptionActionParams;
use qubit_model::medical::PrescriptionContent;
use qubit_model::medical::PrescriptionItem;
use qubit_model::medical::PrescriptionOrderRequest;
use qubit_model_metadata::metadata_of;

/// Verifies prescription and settlement structs retain every Java source field.
#[test]
fn test_prescription_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<Diagnosis>().struct_fields().len(), 9);
    assert_eq!(metadata_of::<MedicalSettlement>().struct_fields().len(), 13);
    assert_eq!(metadata_of::<Prescription>().struct_fields().len(), 13);
    assert_eq!(
        metadata_of::<PrescriptionActionParams>()
            .struct_fields()
            .len(),
        7
    );
    assert_eq!(
        metadata_of::<PrescriptionContent>().struct_fields().len(),
        24
    );
    assert_eq!(metadata_of::<PrescriptionItem>().struct_fields().len(), 7);
    assert_eq!(
        metadata_of::<PrescriptionOrderRequest>()
            .struct_fields()
            .len(),
        2
    );

    let payload = metadata_of::<PrescriptionActionParams>()
        .field("payload")
        .expect("prescription action payload field");
    assert_eq!(
        payload
            .sequence_constraint()
            .expect("prescription payload size constraint")
            .max_items(),
        Some(16)
    );
}
