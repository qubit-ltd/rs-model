// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Individually purchasable product specifications.

use bigdecimal::BigDecimal;
use chrono::DateTime;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::TimeDelta;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::Entity;
use crate::product::Product;
use crate::upload::Attachment;

/// A priced specification belonging to a product.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[model(unique(
    name = "product_item_product_specification",
    fields(product_id, specification),
    ignore_case(specification)
))]
pub struct ProductItem {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted identifier of the owning product.
    #[model(reference(target = Product, target_field = id), opaque)]
    pub product_id: Id,

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
    #[model(opaque)]
    pub service_id: Id,

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
