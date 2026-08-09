// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Access-token contracts.

use crate::commons::Token;

/// Gives a model an optional issued access token.
pub trait WithToken {
    /// Returns the current token, or `None` when no token has been issued.
    fn token(&self) -> Option<&Token>;

    /// Sets the current token; `None` clears the stored token reference.
    fn set_token(&mut self, token: Option<Token>);
}
