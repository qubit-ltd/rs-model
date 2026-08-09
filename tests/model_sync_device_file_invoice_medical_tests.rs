// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Metadata regressions for device, file, invoice, and medical model sync.

use qubit_model::device::Device;
use qubit_model::device::Hardware;
use qubit_model::invoice::Invoice;
use qubit_model::invoice::InvoiceItem;
use qubit_model::medical::Disease;
use qubit_model::medical::Drug;
use qubit_model::medical::MedicalSettlementItem;
use qubit_model::medical::Patient;
use qubit_model::medical::PatientInfo;
use qubit_model::upload::FileInfo as UploadFileInfo;
use qubit_model_metadata::DecimalSemantic;
use qubit_model_metadata::UniqueComparison;
use qubit_model_metadata::metadata_of;

/// Verifies high-risk Java field constraints are retained by their Rust models.
#[test]
fn test_scoped_models_preserve_java_metadata_constraints() {
    let device = metadata_of::<Device>();
    assert_eq!(
        device
            .field("code")
            .expect("device code metadata")
            .text_constraint()
            .expect("device code text metadata")
            .max_chars(),
        Some(128)
    );
    assert_eq!(
        device
            .unique_constraints()
            .find(|constraint| constraint.contains("code"))
            .expect("device code uniqueness")
            .comparison_of("code"),
        Some(UniqueComparison::Exact)
    );
    for field in ["app", "owner", "binder", "payloads"] {
        assert!(
            device
                .field(field)
                .expect("device reference field metadata")
                .reference()
                .is_some(),
            "missing device reference for {field}"
        );
    }
    assert!(
        device
            .indexes()
            .any(|index| index.contains("deploy_address"))
    );
    for field in ["name", "device_type", "state", "create_time"] {
        assert!(
            device.indexes().any(|index| index.contains(field)),
            "missing device index for {field}"
        );
    }

    let hardware = metadata_of::<Hardware>();
    assert_eq!(
        hardware
            .field("udid")
            .expect("UDID metadata")
            .text_constraint()
            .expect("UDID text metadata")
            .max_chars(),
        Some(128)
    );
    assert_eq!(
        hardware
            .unique_constraints()
            .find(|constraint| constraint.contains("udid"))
            .expect("UDID uniqueness")
            .comparison_of("udid"),
        Some(UniqueComparison::Exact)
    );

    let file = metadata_of::<UploadFileInfo>();
    assert_eq!(
        file.unique_constraints()
            .find(|constraint| constraint.contains("path"))
            .expect("file path uniqueness")
            .comparison_of("path"),
        Some(UniqueComparison::IgnoreCase)
    );
    assert_eq!(
        file.field("quality")
            .expect("file quality metadata")
            .decimal_constraint()
            .expect("file quality decimal metadata")
            .scale(),
        2
    );

    let invoice = metadata_of::<Invoice>();
    for field in [
        "app",
        "organization",
        "place",
        "related_invoice",
        "items",
        "settlement",
    ] {
        assert!(
            invoice
                .field(field)
                .expect("invoice reference field metadata")
                .reference()
                .is_some(),
            "missing invoice reference for {field}"
        );
    }
    for field in ["exchange_rate"] {
        assert_eq!(
            invoice
                .field(field)
                .expect("invoice monetary field metadata")
                .decimal_constraint()
                .expect("invoice monetary decimal metadata")
                .semantic(),
            DecimalSemantic::Money,
            "{field} must retain money semantics"
        );
    }
    let invoice_item = metadata_of::<InvoiceItem>();
    for field in ["amount", "tax_rate"] {
        assert_eq!(
            invoice_item
                .field(field)
                .expect("invoice item monetary field metadata")
                .decimal_constraint()
                .expect("invoice item monetary decimal metadata")
                .semantic(),
            DecimalSemantic::Money,
            "{field} must retain money semantics"
        );
    }

    for metadata in [
        metadata_of::<Disease>(),
        metadata_of::<Drug>(),
        metadata_of::<Patient>(),
        metadata_of::<PatientInfo>(),
    ] {
        assert!(
            metadata
                .unique_constraints()
                .any(|constraint| constraint.contains("code")),
            "missing code uniqueness for {}",
            metadata.identity().type_name()
        );
    }
    assert!(
        metadata_of::<Disease>()
            .field("category")
            .expect("disease category metadata")
            .reference()
            .is_some()
    );
    assert_eq!(
        metadata_of::<MedicalSettlementItem>()
            .field("self_paid_rate")
            .expect("self-paid rate metadata")
            .decimal_constraint()
            .expect("self-paid rate decimal metadata")
            .semantic(),
        DecimalSemantic::Money
    );
}
