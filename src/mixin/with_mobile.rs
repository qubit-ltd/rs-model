// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Mobile-number contracts.

use crate::contact::Phone;

/// Gives a model an optional mobile telephone number.
pub trait WithMobile {
    /// Returns the mobile number, or `None` when it has not been supplied.
    fn mobile(&self) -> Option<&Phone>;

    /// Sets the mobile number; `None` clears it.
    fn set_mobile(&mut self, mobile: Option<Phone>);
}
