// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Values bundled with their digital signature.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};
use serde_json::Value;

use super::Signature;

/// A domain value and the signature covering it.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
pub struct SignedData {
    /// Covered data.
    #[model(opaque)]
    pub data: Value,
    /// Digital signature.
    #[redact(nested)]
    pub signature: Signature,
}
