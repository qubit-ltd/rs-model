// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Stateful identity snapshots paired with access tokens.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::commons::Token;
use crate::mixin::StatefulInfo;

/// Stateful identity projection paired with the entity's optional access token.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StatefulInfoWithToken {
    /// Identity, lifecycle, and soft-deletion information for the referenced entity.
    pub info: StatefulInfo,

    /// Access token issued for the entity, or `None` when no token is available.
    pub token: Option<Token>,
}

impl StatefulInfoWithToken {
    /// Creates a stateful identity-and-token projection.
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
