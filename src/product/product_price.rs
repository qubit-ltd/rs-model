// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Source-specific product prices.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

use crate::product::Product;
use crate::product::ProductItem;

/// A product-item price supplied by a particular source.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[model(unique(
    name = "product_price_product_specification",
    fields(product_id, specification),
    ignore_case(specification)
))]
pub struct ProductPrice {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted product identifier.
    #[model(reference(target = Product, target_field = id), opaque)]
    pub product_id: Id,

    /// Persisted product-item identifier.
    #[model(reference(target = ProductItem, target_field = id), opaque)]
    pub product_item_id: Id,

    /// Globally unique product price code.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64))]
    pub code: String,

    /// Product name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Product-scoped unique specification.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub specification: String,

    /// Information about the source that supplied this price.
    #[model(opaque)]
    pub source: Info,

    /// Unit price with four fractional digits.
    #[model(money(scale = 4))]
    pub price: BigDecimal,

    /// Discount amount with four fractional digits.
    #[model(money(scale = 4))]
    pub discount: BigDecimal,

    /// Optional discount reason.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub discount_reason: Option<String>,

    /// Optional local discount start time.
    pub discount_from: Option<NaiveDateTime>,

    /// Optional local discount end time.
    pub discount_until: Option<NaiveDateTime>,

    /// UTC instant at which this record was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the most recent update, or `None` when no update has occurred.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the record remains active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
