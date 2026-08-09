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

use crate::organization::Organization;
use crate::shipping::ConsignInfo;

/// A carrier shipment from a consignor to a consignee.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Shipping {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Carrier organization information.
    #[model(reference(target = Organization, target_field = info), opaque)]
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
