// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Prescription order submission messages.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::{medical::Prescription, order::Order};

/// A prescription and the order submitted for its products.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct PrescriptionOrderRequest {
    /// Prescription content and workflow record.
    pub prescription: Prescription,

    /// Corresponding product order.
    pub order: Order,
}
