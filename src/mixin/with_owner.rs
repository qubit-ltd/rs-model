// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Polymorphic-owner contracts.

use crate::commons::Owner;

/// Gives a model an optional polymorphic owner reference.
pub trait WithOwner {
    /// Returns the owner reference, or `None` when ownership is unassigned.
    fn owner(&self) -> Option<&Owner>;

    /// Sets the owner reference; `None` removes ownership.
    fn set_owner(&mut self, owner: Option<Owner>);
}
