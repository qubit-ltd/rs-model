// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Client environment snapshots.

use qubit_mixin::{Emptyful, Normalizable};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::{contact::Location, system::Platform};

/// Network, location, platform, and device context captured for a request.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct Environment {
    /// Optional ASCII client IP address.
    #[model(index, text(min_chars = 1, max_chars = 128, repertoire = ascii))]
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// Optional geographic location.
    #[model(index)]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// Optional operating-system platform.
    #[model(index)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<Platform>,

    /// Optional unique device identifier.
    #[model(index, sensitive(token), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,

    /// Optional push-notification token.
    #[model(index, sensitive(token), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_token: Option<String>,
}

impl Environment {
    /// Returns whether every environment property is absent.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.ip.is_none()
            && self.location.is_none()
            && self.platform.is_none()
            && self.udid.is_none()
            && self.push_token.is_none()
    }
}

impl Emptyful for Environment {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for Environment {
    fn normalize(&mut self) {
        self.ip.normalize();
        self.udid.normalize();
        self.push_token.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}
