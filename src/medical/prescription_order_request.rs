// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Requests that submit a prescription together with its product order.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::medical::Prescription;
use crate::order::Order;

/// The prescription and the order generated to obtain its prescribed products.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct PrescriptionOrderRequest {
    /// Prescription content and workflow record.
    pub prescription: Prescription,

    /// Corresponding product order.
    pub order: Order,
}
