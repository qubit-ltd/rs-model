// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Employee medical-item assignments.

use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Assigns a medical service item to an employee within an organization.
#[derive(Model, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EmployeeMedicalItem {
    /// Persisted organization identifier.
    #[model(opaque)]
    pub organization_id: Id,

    /// Persisted employee identifier.
    #[model(opaque)]
    pub employee_id: Id,

    /// Persisted medical-item identifier.
    #[model(opaque)]
    pub medical_item_id: Id,
}
