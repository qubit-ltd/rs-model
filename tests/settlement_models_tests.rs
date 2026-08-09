// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for settlement-domain model migrations.

use qubit_model::settlement::Settlement;
use qubit_model::settlement::Transaction;
use qubit_model_metadata::metadata_of;

/// Verifies settlement structs retain every Java source field.
#[test]
fn test_settlement_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<Settlement>().struct_fields().len(), 7);
    assert_eq!(metadata_of::<Transaction>().struct_fields().len(), 25);
}

/// Verifies settlement metadata retains Java application, ownership, and workflow references.
#[test]
fn test_settlement_metadata_preserves_source_references() {
    for (model, field) in [
        (metadata_of::<Settlement>(), "app"),
        (metadata_of::<Settlement>(), "organization"),
        (metadata_of::<Transaction>(), "origin_id"),
        (metadata_of::<Transaction>(), "app"),
        (metadata_of::<Transaction>(), "source"),
        (metadata_of::<Transaction>(), "category"),
        (metadata_of::<Transaction>(), "order_id"),
        (metadata_of::<Transaction>(), "return_id"),
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
