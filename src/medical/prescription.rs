// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Persisted prescriptions and their dispensing-workflow context.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::medical::PrescriptionContent;
use crate::medical::PrescriptionStatus;
use crate::organization::EmployeeInfo;

/// A prescription record that binds signable clinical content to review,
/// dispensing, and order-fulfillment state.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Prescription {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Signable prescription content.
    pub content: PrescriptionContent,

    /// Optional hospital reviewing pharmacist.
    pub auditor: Option<EmployeeInfo>,

    /// Optional third-party reviewing pharmacist.
    pub inspector: Option<EmployeeInfo>,

    /// Optional dispensing pharmacist.
    pub pharmacist: Option<EmployeeInfo>,

    /// Optional pharmacy reviewing pharmacist.
    pub reviewer: Option<EmployeeInfo>,

    /// Optional consigning pharmacist.
    pub consignor: Option<EmployeeInfo>,

    /// Optional digital-signature records from workflow participants.
    ///
    /// Signature is owned by the Java security package, which is outside this
    /// migration graph, so each record remains a lossless JSON value.
    #[model(opaque)]
    pub signatures: Option<Vec<serde_json::Value>>,

    /// Current prescription state.
    pub status: PrescriptionStatus,

    /// Optional identifier of the corresponding order.
    #[model(opaque)]
    pub order_id: Id,

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
