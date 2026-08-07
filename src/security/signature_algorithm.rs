// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Supported signature algorithms.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// JDK-compatible digital-signature algorithm.
#[derive(Model, Redact, Clone, Copy, Default, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SignatureAlgorithm {
    /// MD2 with RSA.
    Md2WithRsa,
    /// MD5 with RSA.
    Md5WithRsa,
    /// SHA-1 with RSA.
    Sha1WithRsa,
    /// SHA-224 with RSA.
    Sha224WithRsa,
    /// SHA-256 with RSA.
    #[default]
    Sha256WithRsa,
    /// SHA-384 with RSA.
    Sha384WithRsa,
    /// SHA-512 with RSA.
    Sha512WithRsa,
    /// SHA-1 with DSA.
    Sha1WithDsa,
    /// SHA-224 with DSA.
    Sha224WithDsa,
    /// SHA-256 with DSA.
    Sha256WithDsa,
    /// SHA-1 with ECDSA.
    Sha1WithEcdsa,
    /// SHA-224 with ECDSA.
    Sha224WithEcdsa,
    /// SHA-256 with ECDSA.
    Sha256WithEcdsa,
    /// SHA-384 with ECDSA.
    Sha384WithEcdsa,
    /// SHA-512 with ECDSA.
    Sha512WithEcdsa,
}

impl SignatureAlgorithm {
    /// Returns the JDK algorithm code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Md2WithRsa => "MD2withRSA",
            Self::Md5WithRsa => "MD5withRSA",
            Self::Sha1WithRsa => "SHA1withRSA",
            Self::Sha224WithRsa => "SHA224withRSA",
            Self::Sha256WithRsa => "SHA256withRSA",
            Self::Sha384WithRsa => "SHA384withRSA",
            Self::Sha512WithRsa => "SHA512withRSA",
            Self::Sha1WithDsa => "SHA1withDSA",
            Self::Sha224WithDsa => "SHA224withDSA",
            Self::Sha256WithDsa => "SHA256withDSA",
            Self::Sha1WithEcdsa => "SHA1withECDSA",
            Self::Sha224WithEcdsa => "SHA224withECDSA",
            Self::Sha256WithEcdsa => "SHA256withECDSA",
            Self::Sha384WithEcdsa => "SHA384withECDSA",
            Self::Sha512WithEcdsa => "SHA512withECDSA",
        }
    }
}
