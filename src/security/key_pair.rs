// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persisted asymmetric key pairs.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::KeyFormat;
use super::Signature;
use super::SignatureAlgorithm;
use super::SignedInfo;
use crate::commons::State;

/// Versioned asymmetric key material owned by a domain entity.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct KeyPair {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Owner entity type.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub owner_type: String,

    /// Owner entity identifier.
    #[model(opaque)]
    pub owner_id: Id,

    /// Optional owner code.
    pub owner_code: Option<String>,

    /// Signature algorithm.
    pub algorithm: SignatureAlgorithm,

    /// Encoded-key format.
    pub format: KeyFormat,

    /// Key version.
    #[model(text(min_chars = 1, max_chars = 64))]
    pub version: String,

    /// Encoded public key.
    #[model(text(min_chars = 1, max_chars = 4096))]
    pub public_key: String,

    /// Optional encoded private key.
    #[model(text(min_chars = 1, max_chars = 4096))]
    #[redact(level = "secret")]
    pub private_key: Option<String>,

    /// Lifecycle state.
    #[serde(default)]
    pub state: State,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl KeyPair {
    /// Reports whether this key pair identifies the signer and signed key.
    #[must_use]
    pub fn matches_signature(&self, signature: &Signature) -> bool {
        self.owner_type == signature.signer_type
            && self.owner_id == signature.signer_id
            && self.matches_signed_info(&signature.signed_info)
    }

    /// Reports whether algorithm, version, and public key match.
    #[must_use]
    pub fn matches_signed_info(&self, signed_info: &SignedInfo) -> bool {
        self.algorithm == signed_info.algorithm
            && self.version == signed_info.key_version
            && self.public_key == signed_info.public_key
    }
}
