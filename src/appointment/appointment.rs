// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Appointment records for services offered by other domain objects.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::audit::AuditStatus;
use crate::commons::App;
use crate::mixin::StatefulInfo;
use crate::person::PersonInfo;

/// A person's booking for a service associated with a domain-object target.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Appointment {
    /// Database identifier; its default value means the appointment is not persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Stateful reference to the application responsible for the appointment.
    #[model(reference(target = App, target_field = info))]
    pub app: StatefulInfo,

    /// Domain type that interprets [`Self::objective_id`] as the service target.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub objective_type: String,

    /// Identifier of the service target whose type is [`Self::objective_type`].
    #[model(opaque)]
    pub objective_id: Id,

    /// Person requesting the booking.
    #[redact(nested)]
    pub applicant: PersonInfo,

    /// UTC instant, rounded to seconds, at which the booked service begins.
    #[model(time(precision = second, normalization = utc))]
    pub start_time: DateTime<Utc>,

    /// UTC instant, rounded to seconds, at which the booked service ends.
    #[model(time(precision = second, normalization = utc))]
    pub end_time: DateTime<Utc>,

    /// Current review state that determines whether the booking has been accepted.
    pub audit_status: AuditStatus,

    /// UTC instant, rounded to seconds, when the booking was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the latest update, or `None` if the booking is unchanged.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the booking remains retained.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
