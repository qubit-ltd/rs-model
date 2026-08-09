// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Accessors for creator information.

use crate::person::UserInfo;

/// Gives a model the optional user identity captured when it was created.
pub trait WithCreator {
    /// Returns the creator snapshot, or `None` when the actor was not recorded.
    fn creator(&self) -> Option<&UserInfo>;

    /// Sets the creator snapshot; `None` clears the audit reference.
    fn set_creator(&mut self, creator: Option<UserInfo>);
}
