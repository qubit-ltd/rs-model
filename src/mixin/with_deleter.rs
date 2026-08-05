// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Accessors for soft-deletion actor information.

use crate::person::UserInfo;

/// Provides optional information about the user that deleted an object.
pub trait WithDeleter {
    /// Returns the deleter information.
    fn deleter(&self) -> Option<&UserInfo>;

    /// Replaces the deleter information.
    fn set_deleter(&mut self, deleter: Option<UserInfo>);
}
