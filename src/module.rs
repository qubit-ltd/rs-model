// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Functional modules used to group system capabilities and permissions.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies the functional area that owns an operation or permission.
#[derive(Model, Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Module {
    /// Account registration, authentication, and profile basics.
    #[default]
    BasicOperation,
    /// Platform configuration and reference-data administration.
    SystemManagement,
    /// User, role, and permission administration.
    UserManagement,
    /// Key-pair and digital-signature administration.
    SignatureManagement,
    /// Product-catalog administration.
    ProductManagement,
    /// Order, fulfillment, and transaction administration.
    OrderManagement,
    /// Prescription and related medical-record administration.
    PrescriptionManagement,
    /// Extensions supplied by an integrating business system.
    BusinessExtension,
    /// Appointment scheduling and administration.
    AppointmentManagement,
    /// Delivery and administration of medical services.
    MedicalServiceManagement,
    /// Work-schedule planning and administration.
    WorkScheduleManagement,
}
