//! Organization domain models.

mod department;
mod employee;
mod employee_info;
#[allow(clippy::module_inception)]
mod organization;

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

pub use department::Department;
pub use employee::Employee;
pub use employee_info::EmployeeInfo;
pub use organization::Organization;

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
