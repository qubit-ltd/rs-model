// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Assignment links between an organization employee and a medical service item.

use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies an employee authorized to provide a particular medical service item.
#[derive(Model, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EmployeeMedicalItem {
    /// Identifier of the organization that owns the authorization.
    #[model(opaque)]
    pub organization_id: Id,

    /// Identifier of the employee authorized to provide the item.
    #[model(opaque)]
    pub employee_id: Id,

    /// Identifier of the medical service item assigned to the employee.
    #[model(opaque)]
    pub medical_item_id: Id,
}
