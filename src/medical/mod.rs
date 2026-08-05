//! Medical-domain values required by person models.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// The source-domain medical-insurance classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MedicareType {
    /// Urban employee basic medical insurance.
    Employee,
    /// Urban resident basic medical insurance.
    Resident,
    /// New rural cooperative medical insurance.
    NewRuralCooperative,
    /// Another medical-insurance classification.
    Other,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MedicalInvoiceType {
    ClinicSeriousIllness,
    ClinicSpecial,
    Hospital,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MedicalType {
    Registration,
    Clinic,
    SpecificClinic,
    EmergentClinic,
    Hospitalization,
    Admission,
    Discharge,
    Examination,
    Internet,
    Unknown,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MedicareItemType {
    Drug,
    Item,
    Material,
    Service,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrescriptionAction {
    Create,
    AuditorAccept,
    AuditorReject,
    InspectorAccept,
    InspectorReject,
    PatientAccept,
    PatientReject,
    Transfer,
    Prepare,
    ReviewerAccept,
    ReviewerReject,
    Dispatch,
    Receive,
    Cancel,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrescriptionStatus {
    Created,
    AuditorAccepted,
    AuditorRejected,
    InspectorAccepted,
    InspectorRejected,
    PatientAccepted,
    PatientRejected,
    Transferred,
    Prepared,
    ReviewerAccepted,
    ReviewerRejected,
    Dispatched,
    Received,
    Expired,
    Cancelled,
}
