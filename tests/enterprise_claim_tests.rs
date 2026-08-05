// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for enterprise insurance-claim models.

use qubit_model::claim::enterprise::{
    EnterpriseClaimEvent, EnterpriseClaimItemMedical, EnterpriseClaimSelfCareItem,
    EnterpriseHistoryClaimAmount, EnterpriseInsuredInfo, HistoryClaimAmount,
};
use qubit_model_metadata::metadata_of;

/// Verifies enterprise claim reference structs retain every Java source field.
#[test]
fn test_enterprise_claim_reference_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<HistoryClaimAmount>().struct_fields().len(), 3);
    assert_eq!(
        metadata_of::<EnterpriseHistoryClaimAmount>()
            .struct_fields()
            .len(),
        10
    );
    assert_eq!(
        metadata_of::<EnterpriseInsuredInfo>().struct_fields().len(),
        15
    );
    assert_eq!(
        metadata_of::<EnterpriseClaimEvent>().struct_fields().len(),
        7
    );
    assert_eq!(
        metadata_of::<EnterpriseClaimItemMedical>()
            .struct_fields()
            .len(),
        4
    );
    assert_eq!(
        metadata_of::<EnterpriseClaimSelfCareItem>()
            .struct_fields()
            .len(),
        8
    );
}
