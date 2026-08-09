// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Device-binding parameters.

use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Associates a device identifier with a patient identifier.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct BindDeviceParams {
    /// Device identifier.
    #[redact(level = "secret")]
    pub udid: String,

    /// Identifier of the patient whose record will own the bound device.
    #[model(opaque)]
    pub patient_id: Id,
}
