//! Insurance-claim leaf classifications.
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccidentReason {
    Disease,
    Accident,
    Birth,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuranceClaimInvoiceStatus {
    Saved,
    IgnoredGt,
    IgnoredLt,
    IgnoredNone,
    IgnoredMedicareProhibited,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuranceClaimInvoiceType {
    ClinicSeriousIllness,
    ClinicSpecial,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuranceClaimStatusGroup {
    NotSubmitted,
    PendingCase,
    Registed,
    Unreached,
    UnderReview,
    AuditRejection,
    Rejected,
    Completed,
    Canceld,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InsuredStatus {
    Recovery,
    UnderTreatment,
    Death,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QuickCompensationState {
    Fetching,
    Success,
}
