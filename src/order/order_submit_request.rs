// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Order submission messages.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::order::Order;

/// An order and the callback URLs used during submission.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct OrderSubmitRequest {
    /// Order being submitted.
    pub order: Order,

    /// Browser return URL.
    pub return_url: String,

    /// Server notification URL.
    pub notify_url: String,
}
