// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Accessors for last-modifier information.

use crate::person::UserInfo;

/// Provides optional information about the user that last modified an object.
pub trait WithModifier {
    /// Returns the modifier information.
    fn modifier(&self) -> Option<&UserInfo>;

    /// Replaces the modifier information.
    fn set_modifier(&mut self, modifier: Option<UserInfo>);
}
