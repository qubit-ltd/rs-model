// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for shipping-domain model migrations.

use qubit_model::{
    shipping::{ConsignInfo, Packing, Shipping, ShippingDemand, ShippingMode},
    system::Environment,
};
use qubit_model_metadata::metadata_of;

/// Verifies shipping structs retain every source field in model metadata.
#[test]
fn test_shipping_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<Environment>().struct_fields().len(), 5);
    assert_eq!(metadata_of::<ConsignInfo>().struct_fields().len(), 5);
    assert_eq!(metadata_of::<Shipping>().struct_fields().len(), 10);
    assert_eq!(metadata_of::<ShippingDemand>().struct_fields().len(), 5);
}

/// Verifies shipping enum wire names remain compatible with Java.
#[test]
fn test_shipping_enums_preserve_java_wire_values() {
    assert_eq!(
        serde_json::to_string(&Packing::WoodenFrame).expect("packing should serialize"),
        "\"WOODEN_FRAME\""
    );
    assert_eq!(
        serde_json::to_string(&ShippingMode::SelfPickup).expect("shipping mode should serialize"),
        "\"SELF\""
    );
}
