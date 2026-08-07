// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order submission responses.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Redirect target and encoded parameters returned after order submission.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrderSubmitResponse {
    /// Redirect URL.
    pub url: String,

    /// Provider-defined request parameters.
    pub params: String,
}
