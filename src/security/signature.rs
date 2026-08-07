// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted digital signatures.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::KeyValuePair;
use super::SignedInfo;

/// A signature, its owner, signer, covered information, and encoded value.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
#[serde(default)]
pub struct Signature {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Owner entity type.
    pub owner_type: String,

    /// Owner entity identifier.
    pub owner_id: i64,

    /// Signer entity type.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub signer_type: String,

    /// Signer entity identifier.
    pub signer_id: i64,

    /// Signer entity code.
    pub signer_code: String,

    /// Canonical signed information.
    #[redact(nested)]
    pub signed_info: SignedInfo,

    /// Base64-encoded signature bytes.
    #[model(text(min_chars = 1, max_chars = 2048))]
    #[redact(level = "secret")]
    pub signed_value: String,
}

impl Signature {
    /// Sets or replaces a string payload entry and returns this signature.
    pub fn set_payload(&mut self, key: &str, value: &str) -> &mut Self {
        if let Some(entry) = self
            .signed_info
            .payload
            .iter_mut()
            .find(|entry| entry.key == key)
        {
            entry.value = Some(value.to_owned());
        } else {
            self.signed_info.payload.push(KeyValuePair {
                key: key.to_owned(),
                value: Some(value.to_owned()),
            });
        }
        self
    }

    /// Replaces the covered message and returns this signature.
    pub fn set_message(&mut self, message: &str) -> &mut Self {
        self.signed_info.message = message.to_owned();
        self
    }
}
