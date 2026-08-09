// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Parties allowed to initiate a product return.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The party responsible for opening a return workflow.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnIssuer {
    /// The purchasing party opened the return.
    Buyer,
    /// The selling party opened the return.
    Seller,
    /// The platform opened the return.
    Platform,
    /// The Medicare program opened the return.
    Medicare,
}
