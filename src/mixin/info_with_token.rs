// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Identity projections paired with issued access tokens.

use serde::Deserialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::Token;
use crate::mixin::WithToken;

/// Compact entity identity paired with the optional token issued for that entity.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct InfoWithToken {
    /// Compact identity of the referenced entity.
    #[model(opaque)]
    #[redact(plain)]
    pub info: Info,

    /// Current issued token, or `None` when no token is available.
    #[redact(nested)]
    pub token: Option<Token>,
}

impl InfoWithToken {
    /// Creates an identity-and-token projection.
    #[must_use]
    pub const fn new(info: Info, token: Option<Token>) -> Self {
        Self { info, token }
    }

    /// Returns `true` only when the identity is complete and a token is present.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.info.is_complete() && self.token.is_some()
    }
}

impl WithToken for InfoWithToken {
    fn token(&self) -> Option<&Token> {
        self.token.as_ref()
    }

    fn set_token(&mut self, token: Option<Token>) {
        self.token = token;
    }
}
