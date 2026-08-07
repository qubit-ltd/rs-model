// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Stateful information snapshots that carry an access token.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::Token;
use crate::mixin::StatefulInfo;

/// Couples a stateful information snapshot with an optional access token.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StatefulInfoWithToken {
    /// Stateful identity and lifecycle information.
    pub info: StatefulInfo,

    /// Optional access token issued for the referenced entity.
    pub token: Option<Token>,
}

impl StatefulInfoWithToken {
    /// Creates a stateful information value with an optional token.
    ///
    /// # Parameters
    ///
    /// * `info` - Stateful identity and lifecycle information.
    /// * `token` - Optional token issued for the entity.
    ///
    /// # Returns
    ///
    /// The composed stateful information value.
    #[must_use]
    pub fn new(info: StatefulInfo, token: Option<Token>) -> Self {
        Self { info, token }
    }
}
