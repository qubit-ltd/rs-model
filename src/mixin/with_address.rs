// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rust traits for shared Java model mixins.

use crate::contact::Address;

/// Provides a postal address.
pub trait WithAddress {
    /// Returns the address.
    fn address(&self) -> Option<&Address>;

    /// Replaces the address.
    fn set_address(&mut self, address: Option<Address>);
}
