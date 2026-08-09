// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Namespaces used to identify referral sources.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// The system in which a referral identifier is meaningful.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OpenidType {
    /// A Weixin open identifier.
    Weixin,
    /// A mobile telephone number.
    Mobile,
    /// An electronic-signature identity.
    Esign,
}
