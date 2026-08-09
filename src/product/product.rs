// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Product aggregate models.

use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

use crate::commons::Currency;
use crate::commons::State;
use crate::mixin::StatefulInfo;
use crate::product::ProductConstraint;
use crate::product::ProductItem;
use crate::product::Quality;
use crate::upload::Attachment;

/// A product together with its purchasable item specifications.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Product {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// App-scoped unique product code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Product name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Application that owns this product.
    pub app: StatefulInfo,

    /// Optional product category information.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,

    /// Product quality.
    pub quality: Quality,

    /// Currency used for item prices.
    pub currency: Currency,

    /// Optional primary product image.
    pub image: Option<Attachment>,

    /// Optional summary description.
    pub description: Option<String>,

    /// Optional local start of the product or service validity interval.
    pub valid_from: Option<NaiveDateTime>,

    /// Optional local end of the product or service validity interval.
    pub valid_until: Option<NaiveDateTime>,

    /// Optional brand.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub brand: Option<String>,

    /// Optional place of origin.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub origin: Option<String>,

    /// Optional manufacturer information.
    #[model(opaque)]
    pub manufacturer: Option<Info>,

    /// Seller information.
    #[model(opaque)]
    pub seller: Info,

    /// Local date and time at which sale begins.
    pub sale_from: NaiveDateTime,

    /// Optional local date and time at which sale ends.
    pub sale_until: Option<NaiveDateTime>,

    /// Whether physical delivery is required.
    pub need_delivery: bool,

    /// Whether purchases can be returned.
    pub allow_return: bool,

    /// Whether purchases can be exchanged.
    pub allow_change: bool,

    /// Whether client information is required for purchase.
    pub need_client: bool,

    /// Optional purchase constraints.
    pub constraint: Option<ProductConstraint>,

    /// Purchasable specifications in display order.
    #[model(sequence(min_items = 1, max_items = 8))]
    pub items: Vec<ProductItem>,

    /// Product lifecycle state.
    pub state: State,

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
