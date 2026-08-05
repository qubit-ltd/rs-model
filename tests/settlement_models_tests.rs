// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for settlement-domain model migrations.

use qubit_model::settlement::{Settlement, Transaction};
use qubit_model_metadata::metadata_of;

/// Verifies settlement structs retain every Java source field.
#[test]
fn test_settlement_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<Settlement>().struct_fields().len(), 7);
    assert_eq!(metadata_of::<Transaction>().struct_fields().len(), 25);
}
