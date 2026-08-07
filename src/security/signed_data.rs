// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Values bundled with their digital signature.

use serde::Deserialize;
use serde_json::Value;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::Signature;

/// A domain value and the signature covering it.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct SignedData {
    /// Covered data.
    #[model(opaque)]
    pub data: Value,

    /// Digital signature.
    #[redact(nested)]
    pub signature: Signature,
}
