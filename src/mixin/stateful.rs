// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rust traits for shared Java model mixins.

use crate::commons::State;

/// Provides a lifecycle state.
pub trait Stateful {
    /// Returns the current state.
    fn state(&self) -> Option<State>;

    /// Replaces the current state.
    fn set_state(&mut self, state: Option<State>);
}
