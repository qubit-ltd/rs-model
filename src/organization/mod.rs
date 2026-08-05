// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Organization domain models.

mod department;
mod employee;
mod employee_info;
#[allow(clippy::module_inception)]
mod organization;
mod tax_payer_type;

pub use department::Department;
pub use employee::Employee;
pub use employee_info::EmployeeInfo;
pub use organization::Organization;
pub use tax_payer_type::TaxPayerType;
