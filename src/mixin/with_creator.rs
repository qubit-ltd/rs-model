// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Accessors for creator information.

use crate::person::UserInfo;

/// Provides optional information about the user that created an object.
pub trait WithCreator {
    /// Returns the creator information.
    fn creator(&self) -> Option<&UserInfo>;

    /// Replaces the creator information.
    fn set_creator(&mut self, creator: Option<UserInfo>);
}
