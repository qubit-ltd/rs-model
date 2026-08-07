// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Domain operations grouped by their owning module.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::Module;

/// Identifies a domain operation that can be authorized independently.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Operation {
    /// Registers an account.
    Register,
    /// Authenticates an account.
    Login,
    /// Ends the current account session.
    Logout,
    /// Lists system settings.
    ListSetting,
    /// Lists users.
    ListUser,
    /// Lists products.
    ListProduct,
    /// Lists orders.
    ListOrder,
    /// Lists prescriptions.
    ListPrescription,
    /// Lists appointments.
    ListAppointment,
    /// Lists work schedules.
    ListWorkSchedule,
}

impl Operation {
    /// Returns the system module that owns this operation.
    ///
    /// # Returns
    /// The owning functional module.
    #[must_use]
    pub const fn module(self) -> Module {
        match self {
            Self::Register | Self::Login | Self::Logout => {
                Module::BasicOperation
            }
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
