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
    WithContact,
    WithCredential,
    WithLocation,
    WithMobile,
    WithOrganization,
    WithOwner,
    WithPayloads,
    WithSource,
    WithToken,
};

use qubit_mixin::InfoWithEntity;

/// Provides category information.
pub trait WithCategory {
    /// Returns the category.
    fn category(&self) -> Option<&InfoWithEntity>;

    /// Replaces the category.
    fn set_category(&mut self, category: Option<InfoWithEntity>);
}
