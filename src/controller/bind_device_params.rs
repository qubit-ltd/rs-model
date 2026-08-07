// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Device-binding parameters.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

/// Associates a device identifier with a patient identifier.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct BindDeviceParams {
    /// Device identifier.
    #[redact(level = "secret")]
    pub udid: String,

    /// Optional patient identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patient_id: Option<i64>,
}
