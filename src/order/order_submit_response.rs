// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Redirect instructions produced by an order-submission flow.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Provider redirect target and its opaque request parameters.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct OrderSubmitResponse {
    /// URL to which the client must redirect to continue checkout.
    pub url: String,

    /// Provider-defined parameters to send with the redirect request.
    pub params: String,
}
