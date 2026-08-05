// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order line-item records.

use bigdecimal::BigDecimal;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{
    order::Client,
    product::ProductInfo,
};

/// Quantity, pricing, fulfillment, and client data for one order line.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct OrderItem {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Persisted owning-order identifier.
    pub order_id: i64,
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
    pub service_id: Option<i64>,
    /// Optional clients receiving the product or service.
    pub clients: Option<Vec<Client>>,
}
