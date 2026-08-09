// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for order-domain model migrations.

use qubit_model::order::Buyer;
use qubit_model::order::Client;
use qubit_model::order::ClientOrder;
use qubit_model::order::ClientRefundSubmitRequest;
use qubit_model::order::ConfirmStatus;
use qubit_model::order::Consignee;
use qubit_model::order::Order;
use qubit_model::order::OrderDetail;
use qubit_model::order::OrderInfo;
use qubit_model::order::OrderItem;
use qubit_model::order::OrderSubmitRequest;
use qubit_model::order::OrderSubmitResponse;
use qubit_model::order::RefererInfo;
use qubit_model::order::RefererOrderRecord;
use qubit_model::order::Replacement;
use qubit_model::order::Return;
use qubit_model_metadata::metadata_of;

/// Verifies order structs retain every Java source field.
#[test]
fn test_order_structs_expose_all_source_fields() {
    let actual = [
        metadata_of::<Buyer>().struct_fields().len(),
        metadata_of::<Client>().struct_fields().len(),
        metadata_of::<ClientOrder>().struct_fields().len(),
        metadata_of::<ClientRefundSubmitRequest>()
            .struct_fields()
            .len(),
        metadata_of::<Consignee>().struct_fields().len(),
        metadata_of::<Order>().struct_fields().len(),
        metadata_of::<OrderDetail>().struct_fields().len(),
        metadata_of::<OrderInfo>().struct_fields().len(),
        metadata_of::<OrderItem>().struct_fields().len(),
        metadata_of::<OrderSubmitRequest>().struct_fields().len(),
        metadata_of::<OrderSubmitResponse>().struct_fields().len(),
        metadata_of::<RefererInfo>().struct_fields().len(),
        metadata_of::<RefererOrderRecord>().struct_fields().len(),
        metadata_of::<Replacement>().struct_fields().len(),
        metadata_of::<Return>().struct_fields().len(),
    ];

    assert_eq!(
        actual,
        [8, 18, 31, 4, 12, 36, 3, 19, 12, 3, 2, 8, 15, 0, 27]
    );
}

/// Verifies every Java confirmation result remains available on the Rust wire.
#[test]
fn test_confirm_status_preserves_rejected_wire_value() {
    assert_eq!(
        serde_json::to_string(&ConfirmStatus::Rejected)
            .expect("the rejected status should serialize"),
        "\"REJECTED\""
    );
}

/// Verifies order metadata retains Java relation, uniqueness, and text constraints.
#[test]
fn test_order_metadata_preserves_source_constraints() {
    for (model, field) in [
        (metadata_of::<Buyer>(), "user_id"),
        (metadata_of::<Consignee>(), "user_id"),
        (metadata_of::<Order>(), "user_id"),
        (metadata_of::<Order>(), "app"),
        (metadata_of::<OrderItem>(), "order_id"),
        (metadata_of::<OrderItem>(), "product"),
        (metadata_of::<RefererOrderRecord>(), "order_id"),
        (metadata_of::<RefererOrderRecord>(), "order_item_id"),
        (metadata_of::<RefererOrderRecord>(), "client_id"),
        (metadata_of::<RefererOrderRecord>(), "product_code"),
        (metadata_of::<RefererOrderRecord>(), "product_item_id"),
        (metadata_of::<Return>(), "order_id"),
        (metadata_of::<Return>(), "order_item_id"),
        (metadata_of::<Return>(), "transaction_id"),
        (metadata_of::<Return>(), "product"),
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

    for (model, field, max_chars) in [
        (metadata_of::<Buyer>(), "email", 512),
        (metadata_of::<Client>(), "email", 512),
        (metadata_of::<ClientOrder>(), "pay_number", 128),
        (metadata_of::<ClientOrder>(), "pay_channel_number", 128),
        (metadata_of::<ClientOrder>(), "refund_number", 128),
        (metadata_of::<ClientOrder>(), "refund_channel_number", 128),
        (metadata_of::<Consignee>(), "email", 512),
        (metadata_of::<Order>(), "discount_reason", 256),
        (metadata_of::<Order>(), "shipping_number", 64),
        (metadata_of::<OrderItem>(), "discount_reason", 256),
        (metadata_of::<RefererInfo>(), "openid", 128),
        (metadata_of::<RefererOrderRecord>(), "openid", 128),
        (metadata_of::<RefererOrderRecord>(), "root_openid", 128),
        (metadata_of::<Return>(), "shipping_number", 64),
    ] {
        assert_eq!(
            model
                .field(field)
                .expect("the Java source field should exist")
                .text_constraint()
                .expect("the Java size constraint should exist")
                .max_chars(),
            Some(max_chars),
            "missing Java size metadata for {field}"
        );
    }

    assert!(
        metadata_of::<OrderInfo>()
            .unique_constraints()
            .any(|unique| unique.contains("id")),
        "the Java unique order identifier should remain unique"
    );
}
