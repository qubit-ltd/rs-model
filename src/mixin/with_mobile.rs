// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rust traits for shared Java model mixins.

use crate::contact::Phone;

/// Provides a mobile phone number.
pub trait WithMobile {
    /// Returns the mobile phone.
    fn mobile(&self) -> Option<&Phone>;

    /// Replaces the mobile phone.
    fn set_mobile(&mut self, mobile: Option<Phone>);
}
