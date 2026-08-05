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
