// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Accessors for soft-deletion actor information.

use crate::person::UserInfo;

/// Gives a model the optional user identity that performed its soft deletion.
pub trait WithDeleter {
    /// Returns the deleter snapshot, or `None` when no soft deletion is recorded.
    fn deleter(&self) -> Option<&UserInfo>;

    /// Sets the deleter snapshot; `None` clears the soft-deletion actor.
    fn set_deleter(&mut self, deleter: Option<UserInfo>);
}
