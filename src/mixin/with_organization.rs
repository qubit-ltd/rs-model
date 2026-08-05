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
    WithOwner,
    WithPayloads,
    WithSource,
    WithToken,
};

use super::StatefulInfo;

/// Provides an owning organization.
pub trait WithOrganization {
    /// Returns the organization information.
    fn organization(&self) -> Option<&StatefulInfo>;

    /// Replaces the organization information.
    fn set_organization(&mut self, organization: Option<StatefulInfo>);
}
