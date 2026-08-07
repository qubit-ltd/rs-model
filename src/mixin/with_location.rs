// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rust traits for shared Java model mixins.

use crate::contact::Location;

/// Provides a geographic location.
pub trait WithLocation {
    /// Returns the location.
    fn location(&self) -> Option<&Location>;

    /// Replaces the location.
    fn set_location(&mut self, location: Option<Location>);
}
