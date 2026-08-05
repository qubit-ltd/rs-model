// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Return initiator classifications.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Identifies the party that initiated a return.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReturnIssuer {
    /// The buyer initiated the return.
    Buyer,
    /// The seller initiated the return.
    Seller,
    /// The platform initiated the return.
    Platform,
    /// Medicare initiated the return.
    Medicare,
}
