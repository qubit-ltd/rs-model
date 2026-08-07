// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Invoice-number applications.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{
    commons::DictEntryInfo, invoice::InvoiceApplyStatus, mixin::StatefulInfo, person::UserInfo,
};

/// An application for an invoice-number allocation from a provincial platform.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct InvoiceApply {
    /// Optional persisted application identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Application that owns this request.
    pub app: StatefulInfo,

    /// Organization that owns this request.
    pub organization: StatefulInfo,

    /// User who submitted the request.
    pub applicant: UserInfo,

    /// Application-type dictionary entry.
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

    /// Optional UTC cancellation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub cancel_time: Option<DateTime<Utc>>,

    /// Optional UTC approval timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub approve_time: Option<DateTime<Utc>>,

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
