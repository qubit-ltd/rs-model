// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order submission responses.

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Redirect target and encoded parameters returned after order submission.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct OrderSubmitResponse {
    /// Redirect URL.
    pub url: String,
    /// Provider-defined request parameters.
    pub params: String,
}
