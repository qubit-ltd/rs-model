// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rust traits for shared Java model mixins.

use super::Stateful;
use super::StatefulInfo;

/// Exposes stateful identifying information.
pub trait HasStatefulInfo: Stateful {
    /// Returns the stateful information projection.
    fn stateful_info(&self) -> StatefulInfo;
}
