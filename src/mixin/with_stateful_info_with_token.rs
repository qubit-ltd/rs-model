// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Projections that combine stateful identity information with an access token.

use crate::mixin::HasStatefulInfo;
use crate::mixin::StatefulInfoWithToken;
use crate::mixin::WithToken;

/// Projects a model into stateful identity information plus its current token.
pub trait WithStatefulInfoWithToken: HasStatefulInfo + WithToken {
    /// Returns a value snapshot; the token is cloned and is `None` when the model has none.
    fn stateful_info_with_token(&self) -> StatefulInfoWithToken {
        StatefulInfoWithToken::new(self.stateful_info(), self.token().cloned())
    }
}
