// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice-number applications.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::App;
use crate::commons::DictEntry;
use crate::commons::DictEntryInfo;
use crate::invoice::InvoiceApplyStatus;
use crate::mixin::StatefulInfo;
use crate::organization::Organization;
use crate::person::User;
use crate::person::UserInfo;

/// An application for an invoice-number allocation from a provincial platform.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InvoiceApply {
    /// Identifier of the application; its default value means that no related record is stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Application that owns this request.
    #[model(reference(target = App, target_field = info))]
    pub app: StatefulInfo,

    /// Organization that owns this request.
    #[model(reference(target = Organization, target_field = info))]
    pub organization: StatefulInfo,

    /// User who submitted the request.
    #[model(reference(target = User, target_field = info))]
    pub applicant: UserInfo,

    /// Application-type dictionary entry.
    #[model(reference(target = DictEntry, target_field = info))]
    pub r#type: DictEntryInfo,

    /// Provincial-platform application number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub number: String,

    /// Number of invoices requested.
    pub count: i32,

    /// Current application state.
    pub status: InvoiceApplyStatus,

    /// Optional submission remark.
    pub apply_remark: Option<String>,

    /// Optional cancellation remark.
    pub cancel_remark: Option<String>,

    /// Optional approval remark.
    pub approve_remark: Option<String>,

    /// UTC submission timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub apply_time: DateTime<Utc>,

    /// UTC instant at which it was cancelled, or `None` unless cancellation occurred.
    #[model(time(precision = second, normalization = utc))]
    pub cancel_time: Option<DateTime<Utc>>,

    /// UTC instant at which approval was recorded, or `None` before approval.
    #[model(time(precision = second, normalization = utc))]
    pub approve_time: Option<DateTime<Utc>>,

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
