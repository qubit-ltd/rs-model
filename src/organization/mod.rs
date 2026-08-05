//! Organization domain models.

mod department;
mod employee_info;

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

pub use department::Department;
pub use employee_info::EmployeeInfo;

/// The tax-payer classification of an organization.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaxPayerType {
    /// A small-scale taxpayer.
    SmallScale,
    /// A general taxpayer.
    General,
    /// Any other taxpayer classification.
    Other,
}
