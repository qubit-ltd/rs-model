// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Referral identity classifications.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Identifies the namespace of a referral open identifier.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OpenidType {
    /// Weixin open identifier.
    Weixin,
    /// Mobile-number identifier.
    Mobile,
    /// E-signature identity.
    Esign,
}
