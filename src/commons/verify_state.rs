// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared enumerations from the Java commons model package.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Describes a verification lifecycle state.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VerifyState {
    /// Verification has not started.
    None,
    /// Verification is in progress.
    Verifying,
    /// Verification succeeded.
    Valid,
    /// Verification failed.
    Invalid,
}

impl VerifyState {
    /// Returns the Java-compatible serialized name.
    ///
    /// # Returns
    /// The screaming-snake-case verification-state name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Verifying => "VERIFYING",
            Self::Valid => "VALID",
            Self::Invalid => "INVALID",
        }
    }
}
