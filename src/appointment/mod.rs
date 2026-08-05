//! User appointment records.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::audit::AuditStatus;
use crate::mixin::StatefulInfo;
use crate::person::PersonInfo;

/// A user's appointment for a service provided by another domain object.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Appointment {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Application that owns the appointment.
    pub app: StatefulInfo,
    /// Domain type of the appointment target.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub objective_type: String,
    /// Persisted identifier of the appointment target.
    #[model(identifier)]
    pub objective_id: i64,
    /// Person applying for the appointment.
    #[redact(nested)]
    pub applicant: PersonInfo,
    /// UTC service start timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub start_time: DateTime<Utc>,
    /// UTC service end timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub end_time: DateTime<Utc>,
    /// Current audit state.
    pub audit_status: AuditStatus,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
