// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Canonical information covered by a signature.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::KeyValuePair;
use super::SignatureAlgorithm;

/// Message, signer identity, key material, and payload covered by a signature.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct SignedInfo {
    /// Message to sign.
    pub message: String,

    /// Signature algorithm.
    pub algorithm: SignatureAlgorithm,

    /// Version of the signing key.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub key_version: String,

    /// Public key encoded as text.
    #[model(text(min_chars = 1, max_chars = 4096))]
    pub public_key: String,

    /// Signer name.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub name: String,

    /// Signer credential type.
    pub credential_type: String,

    /// Signer credential number.
    #[model(text(min_chars = 1, max_chars = 128))]
    #[redact(level = "secret")]
    pub credential_number: String,

    /// Application-defined key-value claims protected together with the signature.
    #[model(sequence(max_items = 16))]
    #[redact(nested)]
    pub payload: Vec<KeyValuePair>,

    /// UTC instant at which the signer produced this signed-information envelope.
    #[model(time(precision = second, normalization = utc))]
    pub timestamp: Option<DateTime<Utc>>,
}
