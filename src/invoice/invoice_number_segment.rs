// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Allocated invoice-number segments.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::DictEntryInfo;
use crate::invoice::InvoiceStockStatus;
use crate::mixin::StatefulInfo;

/// A contiguous invoice-number segment allocated for use by an organization.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InvoiceNumberSegment {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Application that owns this segment.
    pub app: StatefulInfo,

    /// Organization that owns this segment.
    pub organization: StatefulInfo,

    /// Persisted identifier of the allocation application.
    #[model(opaque)]
    pub apply_id: Id,

    /// Provincial-platform allocation application number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub apply_number: String,

    /// Electronic-invoice-type dictionary entry.
    pub r#type: DictEntryInfo,

    /// Electronic-invoice code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Electronic-invoice name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Number of invoices in the segment.
    pub count: i32,

    /// Inclusive first invoice number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub start: String,

    /// Inclusive last invoice number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub end: String,

    /// Current stock state.
    pub status: InvoiceStockStatus,

    /// Optional remark.
    pub remark: Option<String>,

    /// UTC allocation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub dispatch_time: DateTime<Utc>,

    /// UTC invalidation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub cancel_time: DateTime<Utc>,

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
