// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Behavioral coverage for organization aggregate roots.

use qubit_mixin::Emptyful;
use qubit_mixin::Normalizable;
use qubit_model::contact::Address;
use qubit_model::contact::Phone;
use qubit_model::organization::Organization;
use qubit_model::product::Seller;

/// Verifies source seller assignment and the organization value operations.
#[test]
fn test_organization_assigns_seller_and_normalizes_text() {
    let mut organization = Organization::default();
    assert!(organization.is_empty());
    assert!(Emptyful::is_empty(&organization));

    let seller = Seller {
        id: Some(7),
        code: "SELLER".into(),
        name: "Seller name".into(),
        phone: Some(Phone::from("010-123")),
        mobile: Some(Phone::from("13800138000")),
        email: Some("seller@example.test".into()),
        url: Some("https://seller.example.test".into()),
        credential: None,
        address: Some(Address::default()),
    };
    organization.assign_seller(&seller);
    let info = organization.info();
    assert_eq!(info.id, Some(7));
    assert_eq!(info.code, "SELLER");
    assert_eq!(
        organization
            .contact
            .as_ref()
            .and_then(|contact| contact.email.as_deref()),
        Some("seller@example.test")
    );

    organization.code = "  seller  ".into();
    organization.name = "  Seller name  ".into();
    organization.normalize();
    assert_eq!(organization.code, "seller");
    assert_eq!(organization.name, "Seller name");
    assert!(!organization.is_empty());
}
