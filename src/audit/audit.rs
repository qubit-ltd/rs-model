// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Audit request model.

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
    audit::AuditStatus,
    organization::EmployeeInfo,
};

/// Describes a request to audit a persisted objective.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Audit {
    /// Persistent audit identifier when assigned.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Source type of the audited objective.
    #[model(text(min_chars=1,max_chars=64,repertoire=ascii))]
    pub objective_type: String,
    /// Persistent identifier of the audited objective when assigned.
    #[model(identifier)]
    pub objective_id: Option<i64>,
    /// Current lifecycle state of the request.
    pub status: AuditStatus,
    /// Employee assigned to perform the audit, when assigned.
    pub auditor: Option<EmployeeInfo>,
    /// UTC time when the request was created.
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,
    /// UTC time when the request was last modified, when modified.
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// UTC time when the request was deleted, when deleted.
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
