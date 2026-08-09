// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for product-domain model migrations.

use chrono::TimeDelta;
use qubit_id::Id;

use qubit_model::person::Gender;
use qubit_model::product::Coupon;
use qubit_model::product::CouponRule;
use qubit_model::product::CouponType;
use qubit_model::product::PersonConstraint;
use qubit_model::product::Product;
use qubit_model::product::ProductConstraint;
use qubit_model::product::ProductInfo;
use qubit_model::product::ProductItem;
use qubit_model::product::ProductPrice;
use qubit_model::product::Quality;
use qubit_model::product::Seller;
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact as _;

/// Verifies all migrated public structs expose complete model metadata.
#[test]
fn test_product_structs_expose_all_source_fields() {
    assert_eq!(metadata_of::<Coupon>().struct_fields().len(), 15);
    assert_eq!(metadata_of::<CouponRule>().struct_fields().len(), 9);
    assert_eq!(metadata_of::<PersonConstraint>().struct_fields().len(), 10);
    assert_eq!(metadata_of::<ProductConstraint>().struct_fields().len(), 8);
    assert_eq!(metadata_of::<ProductInfo>().struct_fields().len(), 33);
    assert_eq!(metadata_of::<ProductItem>().struct_fields().len(), 22);
    assert_eq!(metadata_of::<Product>().struct_fields().len(), 27);
    assert_eq!(metadata_of::<ProductPrice>().struct_fields().len(), 15);
    assert_eq!(metadata_of::<Seller>().struct_fields().len(), 9);
}

#[test]
fn test_product_metadata_preserves_source_uniques_and_references() {
    let product = metadata_of::<Product>();
    assert_eq!(product.unique_constraints().count(), 1);
    assert!(product.field("app").unwrap().reference().is_some());
    assert!(product.field("category").unwrap().reference().is_some());

    let coupon = metadata_of::<Coupon>();
    assert_eq!(coupon.unique_constraints().count(), 1);
    assert!(coupon.field("app").unwrap().reference().is_some());

    let seller = metadata_of::<Seller>();
    assert_eq!(seller.unique_constraints().count(), 1);

    let coupon_rule = metadata_of::<CouponRule>();
    assert_eq!(coupon_rule.unique_constraints().count(), 1);
    assert!(coupon_rule.field("app").unwrap().reference().is_some());

    let product_item = metadata_of::<ProductItem>();
    assert_eq!(product_item.unique_constraints().count(), 1);
    assert!(
        product_item
            .field("product_id")
            .unwrap()
            .reference()
            .is_some()
    );

    let product_info = metadata_of::<ProductInfo>();
    assert!(product_info.field("item_id").unwrap().reference().is_some());

    let product_price = metadata_of::<ProductPrice>();
    assert_eq!(product_price.unique_constraints().count(), 2);
    assert!(
        product_price
            .field("product_id")
            .unwrap()
            .reference()
            .is_some()
    );
    assert!(
        product_price
            .field("product_item_id")
            .unwrap()
            .reference()
            .is_some()
    );
}

/// Verifies product enum JSON values remain compatible with Java names.
#[test]
fn test_product_enums_preserve_java_wire_values() {
    assert_eq!(
        serde_json::to_string(&CouponType::DirectDiscount).expect("coupon type should serialize"),
        "\"DIRECT_DISCOUNT\""
    );
    assert_eq!(
        serde_json::to_string(&Quality::BrandNew).expect("quality should serialize"),
        "\"BRAND_NEW\""
    );
}

/// Verifies optional constraints retain value ownership when cloned.
#[test]
fn test_person_constraint_clone_owns_city_lists() {
    let original = PersonConstraint {
        min_age: Some(TimeDelta::days(18 * 365)),
        max_age: None,
        adult_only: Some(true),
        gender: Some(Gender::Female),
        has_medicare: Some(true),
        has_social_security: None,
        has_medicare_or_social_security: Some(true),
        medicare_cities: Some(vec!["310000".to_owned()]),
        social_security_cities: None,
        need_guardian: Some(false),
    };
    let mut cloned = original.clone();
    cloned
        .medicare_cities
        .as_mut()
        .expect("the cloned allowlist should remain present")
        .push("320000".to_owned());

    assert_eq!(original.medicare_cities, Some(vec!["310000".to_owned()]));
    assert_ne!(original, cloned);
}

/// Verifies seller email data is protected by the redaction boundary.
#[test]
fn test_seller_redacts_email() {
    let seller = Seller {
        id: Id::from(7),
        code: "SELLER-7".to_owned(),
        name: "Example Seller".to_owned(),
        phone: None,
        mobile: None,
        email: Some("seller@example.com".to_owned()),
        url: Some("https://example.com".to_owned()),
        credential: None,
        address: None,
    };

    let redacted = format!("{:?}", seller.redacted());
    assert!(!redacted.contains("seller@example.com"));
}
