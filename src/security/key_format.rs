// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Encoded key formats.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

/// Standard encoding of an asymmetric key.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    Model,
    PartialEq,
    Redact,
    Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KeyFormat {
    /// PKCS#8 private-key encoding.
    #[default]
    Pkcs8,
    /// X.509 public-key encoding.
    X509,
}

impl KeyFormat {
    /// Returns the JDK format code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Pkcs8 => "PKCS#8",
            Self::X509 => "X.509",
        }
    }

    /// Resolves an enum name or JDK format code without case sensitivity.
    #[must_use]
    pub fn for_name(value: &str) -> Option<Self> {
        match value.to_ascii_uppercase().as_str() {
            "PKCS8" | "PKCS#8" => Some(Self::Pkcs8),
            "X509" | "X.509" => Some(Self::X509),
            _ => None,
        }
    }
}
