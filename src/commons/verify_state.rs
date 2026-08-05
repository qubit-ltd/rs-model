// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared enumerations from the Java commons model package.

#[allow(unused_imports)]
use super::{
    AuthorizeRecord,
    Category,
    Credential,
    CredentialInfo,
    CredentialType,
    Currency,
    DayType,
    Kinship,
    MqType,
    Owner,
    Owners,
    Payload,
    RequestStatus,
    Source,
};

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Describes a verification lifecycle state.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
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
