// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shipment records.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

use crate::shipping::ConsignInfo;

/// A carrier shipment from a consignor to a consignee.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Shipping {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Carrier organization information.
    #[model(opaque)]
    pub organization: Info,

    /// Carrier tracking number.
    pub number: String,

    /// UTC shipment timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub ship_time: DateTime<Utc>,

    /// Sender information.
    pub consignor: ConsignInfo,

    /// Recipient information.
    pub consignee: ConsignInfo,

    /// Optional shipment comment.
    pub comment: Option<String>,

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
