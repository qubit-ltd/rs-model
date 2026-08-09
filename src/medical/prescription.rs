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
    /// Typed identifier used when this prescription is persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Signable prescription content.
    pub content: PrescriptionContent,

    /// Hospital reviewing pharmacist; `Some` after an in-hospital audit records
    /// its participant, while `None` means that audit has not occurred or does
    /// not apply.
    pub auditor: Option<EmployeeInfo>,

    /// Third-party reviewing pharmacist; `Some` records an external inspection
    /// participant, while `None` means no external inspection is recorded.
    pub inspector: Option<EmployeeInfo>,

    /// Pharmacy pharmacist who prepared the medication; `Some` records the
    /// participant after preparation, while `None` means it is not prepared.
    pub pharmacist: Option<EmployeeInfo>,

    /// Pharmacy pharmacist who reviewed prepared medication; `Some` records
    /// that participant, while `None` means pharmacy review is not complete.
    pub reviewer: Option<EmployeeInfo>,

    /// Pharmacy pharmacist who dispensed the medication; `Some` records the
    /// participant, while `None` means no dispensing event is recorded.
    pub consignor: Option<EmployeeInfo>,

    /// Digital signatures collected during workflow transitions. `Some` holds
    /// signatures from participants such as prescribers, pharmacists, and the
    /// patient; `None` means no signatures have been retained.
    ///
    /// Signature is owned by the Java security package, which is outside this
    /// migration graph, so each record remains a lossless JSON value.
    #[model(opaque)]
    pub signatures: Option<Vec<serde_json::Value>>,

    /// Current prescription state.
    pub status: PrescriptionStatus,

    /// Typed identifier linking this prescription to its product order; one
    /// order may contain multiple prescriptions.
    #[model(opaque)]
    pub order_id: Id,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC time of the latest update, absent until the persisted prescription is
    /// modified after creation.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion time, absent while the prescription remains active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
