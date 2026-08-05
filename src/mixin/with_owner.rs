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
    WithContact,
    WithCredential,
    WithLocation,
    WithMobile,
    WithOrganization,
    WithPayloads,
    WithSource,
    WithToken,
};

use crate::commons::Owner;

/// Provides an owner.
pub trait WithOwner {
    /// Returns the owner.
    fn owner(&self) -> Option<&Owner>;

    /// Replaces the owner.
    fn set_owner(&mut self, owner: Option<Owner>);
}
