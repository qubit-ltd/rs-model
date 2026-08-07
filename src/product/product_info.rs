// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Product snapshots embedded in purchase records.

use bigdecimal::BigDecimal;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::TimeDelta;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

use crate::commons::Currency;
use crate::product::ProductConstraint;
use crate::product::Quality;
use crate::upload::Attachment;

/// A value snapshot of the product and item selected for a purchase.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ProductInfo {
    /// Optional persisted product identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Persisted identifier of the selected product item.
    #[model(opaque)]
    pub item_id: Id,

    /// Product name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Product code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Product quality.
    pub quality: Quality,

    /// Pricing unit.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub unit: String,

    /// Selected specification.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub specification: String,

    /// Currency used for the price.
    pub currency: Currency,

    /// Unit price with four fractional digits.
    #[model(money(scale = 4))]
    pub price: BigDecimal,

    /// Optional shipping weight with four fractional digits.
    #[model(decimal(scale = 4))]
    pub weight: Option<BigDecimal>,

    /// Optional product image.
    pub image: Option<Attachment>,

    /// Optional summary description.
    pub description: Option<String>,

    /// Optional brand.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub brand: Option<String>,

    /// Optional place of origin.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub origin: Option<String>,

    /// Optional manufacturer information.
    #[model(opaque)]
    pub manufacturer: Option<Info>,

    /// Seller information captured for the purchase.
    #[model(opaque)]
    pub seller: Info,

    /// Optional production lot or serial number.
    #[model(text(min_chars = 1, max_chars = 256))]
    pub production_number: Option<String>,

    /// Optional production date.
    pub production_date: Option<NaiveDate>,

    /// Optional shelf life as an elapsed duration.
    #[model(opaque)]
    pub shelf_life: Option<TimeDelta>,

    /// Local date and time at which sale begins.
    pub sale_from: NaiveDateTime,

    /// Optional local date and time at which sale ends.
    pub sale_until: Option<NaiveDateTime>,

    /// Optional local start of the product or service validity interval.
    pub valid_from: Option<NaiveDateTime>,

    /// Optional local end of the product or service validity interval.
    pub valid_until: Option<NaiveDateTime>,

    /// Whether physical delivery is required.
    pub need_delivery: bool,

    /// Whether the purchase can be returned.
    pub allow_return: bool,

    /// Whether the purchase can be exchanged.
    pub allow_change: bool,

    /// Whether client information is required.
    pub need_client: bool,

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

    /// Query-computed inventory; `None` means unlimited.
    pub inventory: Option<i32>,

    /// Query-computed purchase constraints.
    pub constraint: Option<ProductConstraint>,
}
