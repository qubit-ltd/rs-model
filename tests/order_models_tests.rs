// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for order-domain model migrations.

use qubit_model::order::{
    Buyer, Client, ClientOrder, ClientRefundSubmitRequest, Consignee, Order,
    OrderDetail, OrderInfo, OrderItem, OrderSubmitRequest, OrderSubmitResponse,
    RefererInfo, RefererOrderRecord, Replacement, Return,
};
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
