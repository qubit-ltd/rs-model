// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Prescription workflow records.

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    medical::{
        PrescriptionContent,
        PrescriptionStatus,
    },
    organization::EmployeeInfo,
};

/// A prescription and the clinicians, signatures, order, and lifecycle around
/// it.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Prescription {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
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
    pub order_id: Option<i64>,
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
