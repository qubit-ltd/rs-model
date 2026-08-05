// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for invoice-domain model migrations.

use qubit_model::invoice::{
    Invoice, InvoiceApply, InvoiceHospitalRegiste, InvoiceInfo, InvoiceItem, InvoiceNumberSegment,
    InvoicePlace,
};
use qubit_model_metadata::metadata_of;

/// Verifies invoice structs retain every Java source field.
#[test]
fn test_invoice_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<Invoice>().struct_fields().len(), 30);
    assert_eq!(metadata_of::<InvoiceApply>().struct_fields().len(), 17);
    assert_eq!(
        metadata_of::<InvoiceHospitalRegiste>()
            .struct_fields()
            .len(),
        13
    );
    assert_eq!(metadata_of::<InvoiceInfo>().struct_fields().len(), 11);
    assert_eq!(metadata_of::<InvoiceItem>().struct_fields().len(), 16);
    assert_eq!(
        metadata_of::<InvoiceNumberSegment>().struct_fields().len(),
        18
    );
    assert_eq!(metadata_of::<InvoicePlace>().struct_fields().len(), 9);
}
