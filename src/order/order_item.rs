// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order line-item records.

use bigdecimal::BigDecimal;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::order::Client;
use crate::product::ProductInfo;

/// Quantity, pricing, fulfillment, and client data for one order line.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrderItem {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted owning-order identifier.
    #[model(opaque)]
    pub order_id: Id,

    /// Position within the order.
    pub index: i32,

    /// Purchased product snapshot.
    pub product: ProductInfo,

    /// Purchased quantity.
    pub count: i32,

    /// Extended line price.
    #[model(money(scale = 4))]
    pub total_price: BigDecimal,

    /// Line discount.
    #[model(money(scale = 4))]
    pub discount: BigDecimal,

    /// Optional discount reason.
    pub discount_reason: Option<String>,

    /// Line shipping cost.
    #[model(money(scale = 4))]
    pub shipping_cost: BigDecimal,

    /// Amount payable for the line.
    #[model(money(scale = 4))]
    pub payable: BigDecimal,

    /// Optional linked service identifier.
    #[model(opaque)]
    pub service_id: Id,

    /// Optional clients receiving the product or service.
    pub clients: Option<Vec<Client>>,
}
