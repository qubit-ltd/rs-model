// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted digital signatures.

use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::KeyValuePair;
use super::SignedInfo;

/// Represents a signature, its owner, signer, covered information, and encoded value.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct Signature {
    /// Database identifier for this signature record; default denotes an unsaved signature.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Owner entity type.
    pub owner_type: String,

    /// Owner entity identifier.
    #[model(opaque)]
    pub owner_id: Id,

    /// Signer entity type.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub signer_type: String,

    /// Signer entity identifier.
    #[model(opaque)]
    pub signer_id: Id,

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
    /// Sets or replaces a string payload claim and returns this signature for chaining.
    ///
    /// This mutates the data covered by [`Self::signed_value`]. The existing signature bytes
    /// are therefore invalid until the caller signs the updated [`Self::signed_info`] again.
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

    /// Replaces the covered message and returns this signature for chaining.
    ///
    /// This mutates the data covered by [`Self::signed_value`]. The existing signature bytes
    /// are therefore invalid until the caller signs the updated [`Self::signed_info`] again.
    pub fn set_message(&mut self, message: &str) -> &mut Self {
        self.signed_info.message = message.to_owned();
        self
    }
}
