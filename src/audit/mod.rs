//! Audit request models.
use crate::organization::EmployeeInfo;
use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuditStatus {
    Submitted,
}
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Audit {
    #[model(identifier)]
    pub id: Option<i64>,
    #[model(text(min_chars=1,max_chars=64,repertoire=ascii))]
    pub objective_type: String,
    #[model(identifier)]
    pub objective_id: Option<i64>,
    pub status: AuditStatus,
    pub auditor: Option<EmployeeInfo>,
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
