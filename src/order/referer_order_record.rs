// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Referred-order tracking records.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::order::OpenidType;
use crate::order::RefererOrderRecordStatus;

/// Referral ancestry and payment state for one order item and client.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct RefererOrderRecord {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Direct referral identifier namespace.
    pub openid_type: OpenidType,

    /// Direct referral identifier.
    #[redact(level = "secret")]
    pub openid: String,

    /// Root referral identifier namespace.
    pub root_openid_type: OpenidType,

    /// Root referral identifier.
    #[redact(level = "secret")]
    pub root_openid: String,

    /// Referral depth from the root.
    pub root_level: i32,

    /// Persisted order identifier.
    pub order_id: i64,

    /// Persisted order-item identifier.
    pub order_item_id: i64,

    /// Persisted client identifier.
    pub client_id: i64,

    /// Product code.
    pub product_code: String,

    /// Persisted product-item identifier.
    pub product_item_id: i64,

    /// Referral order state.
    pub status: RefererOrderRecordStatus,

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
