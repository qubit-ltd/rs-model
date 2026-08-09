// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Address-bearing domain-object contracts.

use crate::contact::Address;

/// Gives a model an optional postal delivery address.
pub trait WithAddress {
    /// Returns the address, or `None` when no delivery address is recorded.
    fn address(&self) -> Option<&Address>;

    /// Replaces the address; pass `None` to clear the recorded delivery address.
    fn set_address(&mut self, address: Option<Address>);
}
