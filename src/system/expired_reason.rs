// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Session expiration reasons.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Reason that a session expired.
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
pub enum ExpiredReason {
    /// Explicit logout.
    Logout,
    /// Inactivity timeout.
    Timeout,
    /// Another single-session login replaced this session.
    SingleSession,
    /// System maintenance.
    Maintenance,
    /// Session is not expired.
    #[default]
    None,
}

impl ExpiredReason {
    /// Returns the stable lowercase source identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::Logout => "logout",
            Self::Timeout => "timeout",
            Self::SingleSession => "single_session",
            Self::Maintenance => "maintenance",
            Self::None => "none",
        }
    }
}
