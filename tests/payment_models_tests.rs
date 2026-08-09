// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for payment account and record migrations.

use qubit_model::payment::Account;
use qubit_model::payment::Participant;
use qubit_model::payment::Payment;
use qubit_model_metadata::metadata_of;

/// Verifies payment structs retain every Java source field.
#[test]
fn test_payment_core_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<Account>().struct_fields().len(), 11);
    assert_eq!(metadata_of::<Participant>().struct_fields().len(), 9);
    assert_eq!(metadata_of::<Payment>().struct_fields().len(), 22);
}

/// Verifies payment relations and participant email constraints mirror Java metadata.
#[test]
fn test_payment_metadata_preserves_source_references_and_text_constraints() {
    let payment = metadata_of::<Payment>();
    for field in ["order_id", "transaction_id"] {
        assert!(
            payment.field(field).unwrap().reference().is_some(),
            "missing payment reference for {field}"
        );
    }

    let email = metadata_of::<Participant>()
        .field("email")
        .unwrap()
        .text_constraint()
        .unwrap();
    assert_eq!(email.min_chars(), Some(1));
    assert_eq!(email.max_chars(), Some(512));
}
