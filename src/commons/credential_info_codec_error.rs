// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Credential codec failures.

use thiserror::Error;

/// Errors produced while decoding credential information.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CredentialInfoCodecError {
    /// The encoded value does not contain the required type separator.
    #[error("invalid credential format")]
    InvalidFormat,
    /// The encoded credential type is not supported.
    #[error("unsupported credential type: {0}")]
    UnsupportedType(String),
}
