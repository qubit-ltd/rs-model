// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Lifecycle-state contracts for domain models.

use crate::commons::State;

/// Gives a model an optional lifecycle state.
pub trait Stateful {
    /// Returns the lifecycle state, or `None` when no state is assigned.
    fn state(&self) -> Option<State>;

    /// Sets the lifecycle state; `None` clears it.
    fn set_state(&mut self, state: Option<State>);
}
