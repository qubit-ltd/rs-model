// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Employee medical-item assignments.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Assigns a medical service item to an employee within an organization.
#[derive(Clone, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
pub struct EmployeeMedicalItem {
    /// Persisted organization identifier.
    pub organization_id: i64,
    /// Persisted employee identifier.
    pub employee_id: i64,
    /// Persisted medical-item identifier.
    pub medical_item_id: i64,
}
