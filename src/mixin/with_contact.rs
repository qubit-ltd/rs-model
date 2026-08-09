// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Contact-detail contracts.

use crate::contact::Contact;

/// Gives a model optional communication and address details.
pub trait WithContact {
    /// Returns the contact bundle, or `None` when no contact details are recorded.
    fn contact(&self) -> Option<&Contact>;

    /// Sets the contact bundle; `None` removes all recorded contact details.
    fn set_contact(&mut self, contact: Option<Contact>);
}
