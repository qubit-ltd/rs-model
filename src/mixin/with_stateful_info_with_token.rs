// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Projection of stateful information with an access token.

use crate::mixin::{
    HasStatefulInfo,
    StatefulInfoWithToken,
    WithToken,
};

/// Provides a stateful information projection carrying the current token.
pub trait WithStatefulInfoWithToken: HasStatefulInfo + WithToken {
    /// Returns the composed stateful information and token projection.
    fn stateful_info_with_token(&self) -> StatefulInfoWithToken {
        StatefulInfoWithToken::new(self.stateful_info(), self.token().cloned())
    }
}
