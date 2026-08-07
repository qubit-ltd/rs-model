// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Basic information carrying an access token.

use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::Token;
use crate::mixin::WithToken;

/// Basic identifying information together with an optional access token.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct InfoWithToken {
    /// Basic identifying information.
    #[model(opaque)]
    #[redact(skip)]
    #[serde(flatten)]
    pub info: Info,

    /// Optional access token.
    #[redact(nested)]
    pub token: Option<Token>,
}

impl InfoWithToken {
    /// Creates a composed information value.
    #[must_use]
    pub const fn new(info: Info, token: Option<Token>) -> Self {
        Self { info, token }
    }

    /// Reports whether the identifying information and token are present.
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
