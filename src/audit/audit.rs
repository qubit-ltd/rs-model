// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Review requests for persisted domain objects.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::audit::AuditStatus;
use crate::organization::Employee;
use crate::organization::EmployeeInfo;

/// A review request targeting a persisted object in another domain.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Audit {
    /// Database identifier; the default value denotes a request not yet persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Domain type that identifies how to interpret [`Self::objective_id`].
    #[model(text(min_chars = 1, max_chars = 64))]
    pub objective_type: String,

    /// Identifier of the object selected for review.
    #[model(opaque)]
    pub objective_id: Id,

    /// Current state in the audit-request lifecycle.
    pub status: AuditStatus,

    /// Employee responsible for the review, or `None` until one is assigned.
    #[model(reference(target = Employee, target_field = info), opaque)]
    pub auditor: Option<EmployeeInfo>,

    /// UTC creation instant, recorded with second precision.
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the latest modification, or `None` when unchanged.
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the request is retained.
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
