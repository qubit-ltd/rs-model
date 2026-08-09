// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Independently authorizable operations and their owning functional modules.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::Module;

/// Identifies a business action that can be granted as an individual permission.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operation {
    /// Creates a new account.
    Register,
    /// Authenticates an account and starts a session.
    Login,
    /// Ends the caller's current authenticated session.
    Logout,
    /// Retrieves system settings.
    ListSetting,
    /// Retrieves user accounts.
    ListUser,
    /// Retrieves catalog products.
    ListProduct,
    /// Retrieves orders.
    ListOrder,
    /// Retrieves prescriptions.
    ListPrescription,
    /// Retrieves appointments.
    ListAppointment,
    /// Retrieves work schedules.
    ListWorkSchedule,
}

impl Operation {
    /// Returns the functional module responsible for this operation.
    #[must_use]
    pub const fn module(self) -> Module {
        match self {
            Self::Register | Self::Login | Self::Logout => Module::BasicOperation,
            Self::ListSetting => Module::SystemManagement,
            Self::ListUser => Module::UserManagement,
            Self::ListProduct => Module::ProductManagement,
            Self::ListOrder => Module::OrderManagement,
            Self::ListPrescription => Module::PrescriptionManagement,
            Self::ListAppointment => Module::AppointmentManagement,
            Self::ListWorkSchedule => Module::WorkScheduleManagement,
        }
    }
}
