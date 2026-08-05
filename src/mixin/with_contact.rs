// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rust traits for shared Java model mixins.

#[allow(unused_imports)]
use super::{
    Expirable,
    HasStatefulInfo,
    Stateful,
    WithAddress,
    WithApp,
    WithCategory,
    WithCredential,
    WithLocation,
    WithMobile,
    WithOrganization,
    WithOwner,
    WithPayloads,
    WithSource,
    WithToken,
};

use crate::contact::Contact;

/// Provides contact details.
pub trait WithContact {
    /// Returns the contact details.
    fn contact(&self) -> Option<&Contact>;

    /// Replaces the contact details.
    fn set_contact(&mut self, contact: Option<Contact>);
}
