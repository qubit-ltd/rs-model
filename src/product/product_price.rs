// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Source-specific product prices.

use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    NaiveDateTime,
    Utc,
};
use qubit_mixin::Info;
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// A product-item price supplied by a particular source.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct ProductPrice {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Persisted product identifier.
    pub product_id: i64,
    /// Persisted product-item identifier.
    pub product_item_id: i64,
    /// Globally unique product price code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
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
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
