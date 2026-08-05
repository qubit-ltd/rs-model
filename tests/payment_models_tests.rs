// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for payment account and record migrations.

use qubit_model::payment::{
    Account,
    Participant,
    Payment,
};
use qubit_model_metadata::metadata_of;

/// Verifies payment structs retain every Java source field.
#[test]
fn test_payment_core_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<Account>().struct_fields().len(), 11);
    assert_eq!(metadata_of::<Participant>().struct_fields().len(), 9);
    assert_eq!(metadata_of::<Payment>().struct_fields().len(), 22);
}
