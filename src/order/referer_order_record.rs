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
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::order::OpenidType;
use crate::order::RefererOrderRecordStatus;

/// Referral ancestry and payment state for one order item and client.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct RefererOrderRecord {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

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
    #[model(opaque)]
    pub order_id: Id,

    /// Persisted order-item identifier.
    #[model(opaque)]
    pub order_item_id: Id,

    /// Persisted client identifier.
    #[model(opaque)]
    pub client_id: Id,

    /// Product code.
    pub product_code: String,

    /// Persisted product-item identifier.
    #[model(opaque)]
    pub product_item_id: Id,

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
