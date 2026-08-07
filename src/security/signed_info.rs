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
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::KeyValuePair;
use super::SignatureAlgorithm;

/// Message, signer identity, key material, and payload covered by a signature.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
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

    /// Optional string payload entries.
    #[model(sequence(max_items = 16))]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub payload: Vec<KeyValuePair>,

    /// UTC signing timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<DateTime<Utc>>,
}
