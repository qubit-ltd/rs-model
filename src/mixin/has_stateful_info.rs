// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Projection contracts for stateful identity information.

use super::Stateful;
use super::StatefulInfo;

/// Produces a value snapshot containing identity and lifecycle information.
pub trait HasStatefulInfo: Stateful {
    /// Returns the current identity-and-state projection by value.
    fn stateful_info(&self) -> StatefulInfo;
}
