// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Referral identity classifications.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Identifies the namespace of a referral open identifier.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OpenidType {
    /// Weixin open identifier.
    Weixin,
    /// Mobile-number identifier.
    Mobile,
    /// E-signature identity.
    Esign,
}
