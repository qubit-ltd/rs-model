// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Client environment snapshots.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{contact::Location, system::Platform};

/// Network, location, platform, and device context captured for a request.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Environment {
    /// Optional ASCII client IP address.
    #[model(text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    pub ip: Option<String>,
    /// Optional geographic location.
    pub location: Option<Location>,
    /// Optional operating-system platform.
    pub platform: Option<Platform>,
    /// Optional unique device identifier.
    #[model(sensitive(token), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub udid: Option<String>,
    /// Optional push-notification token.
    #[model(sensitive(token), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub push_token: Option<String>,
}
