// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Accessors for last-modifier information.

use crate::person::UserInfo;

/// Gives a model the optional user identity that last changed it.
pub trait WithModifier {
    /// Returns the last modifier snapshot, or `None` when the actor is unknown.
    fn modifier(&self) -> Option<&UserInfo>;

    /// Sets the last modifier snapshot; `None` clears it.
    fn set_modifier(&mut self, modifier: Option<UserInfo>);
}
