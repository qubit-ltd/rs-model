// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! System module classifications.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Identifies a functional module in the domain system.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Module {
    /// Basic account and profile operations.
    #[default]
    BasicOperation,
    /// System administration.
    SystemManagement,
    /// User administration.
    UserManagement,
    /// Signature administration.
    SignatureManagement,
    /// Product administration.
    ProductManagement,
    /// Order administration.
    OrderManagement,
    /// Prescription administration.
    PrescriptionManagement,
    /// Business-system extensions.
    BusinessExtension,
    /// Appointment administration.
    AppointmentManagement,
    /// Medical-service administration.
    MedicalServiceManagement,
    /// Work-schedule administration.
    WorkScheduleManagement,
}
