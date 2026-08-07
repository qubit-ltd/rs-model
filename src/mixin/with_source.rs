// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rust traits for shared Java model mixins.

use qubit_mixin::InfoWithEntity;


/// Provides source information.
pub trait WithSource {
    /// Returns the source.
    fn source(&self) -> Option<&InfoWithEntity>;

    /// Replaces the source.
    fn set_source(&mut self, source: Option<InfoWithEntity>);
}
