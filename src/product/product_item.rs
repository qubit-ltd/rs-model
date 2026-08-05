// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Individually purchasable product specifications.

use bigdecimal::BigDecimal;
use chrono::{
    DateTime,
    NaiveDate,
    NaiveDateTime,
    TimeDelta,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    Entity,
    upload::Attachment,
};

/// A priced specification belonging to a product.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct ProductItem {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Persisted identifier of the owning product.
    pub product_id: i64,
    /// Position in the owning product's item list.
    pub index: i32,
    /// Product-scoped unique specification.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub specification: String,
    /// Pricing unit.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub unit: String,
    /// Optional shipping weight with four fractional digits.
    #[model(decimal(scale = 4))]
    pub weight: Option<BigDecimal>,
    /// Optional production lot or serial number.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub production_number: Option<String>,
    /// Optional production date.
    pub production_date: Option<NaiveDate>,
    /// Optional shelf life as an elapsed duration.
    #[model(opaque)]
    pub shelf_life: Option<TimeDelta>,
    /// Optional item image.
    pub image: Option<Attachment>,
    /// Optional item description.
    pub description: Option<String>,
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
    /// Current inventory; `None` means unlimited.
    pub inventory: Option<i32>,
    /// Optional entity classification of a linked service.
    pub service_type: Option<Entity>,
    /// Optional persisted identifier of the linked service.
    pub service_id: Option<i64>,
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
