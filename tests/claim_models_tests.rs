// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for individual insurance-claim models.

use qubit_model::claim::{
    InsuranceClaim,
    InsuranceClaimAmount,
    InsuranceClaimEvent,
    InsuranceClaimInvoice,
    InsuranceClaimInvoiceCost,
    InsuranceClaimMedical,
    InsuranceProductRule,
};
use qubit_model_metadata::metadata_of;

/// Verifies individual claim structs retain every Java source field.
#[test]
fn test_claim_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<InsuranceClaim>().struct_fields().len(), 39);
    assert_eq!(
        metadata_of::<InsuranceClaimAmount>().struct_fields().len(),
        22
    );
    assert_eq!(
        metadata_of::<InsuranceClaimEvent>().struct_fields().len(),
        8
    );
    assert_eq!(
        metadata_of::<InsuranceClaimInvoice>().struct_fields().len(),
        24
    );
    assert_eq!(
        metadata_of::<InsuranceClaimInvoiceCost>()
            .struct_fields()
            .len(),
        8
    );
    assert_eq!(
        metadata_of::<InsuranceClaimMedical>().struct_fields().len(),
        16
    );
    assert_eq!(
        metadata_of::<InsuranceProductRule>().struct_fields().len(),
        8
    );
}
