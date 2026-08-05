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
    WithCategory,
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

use super::StatefulInfo;

/// Provides an owning application.
pub trait WithApp {
    /// Returns the application information.
    fn app(&self) -> Option<&StatefulInfo>;

    /// Replaces the application information.
    fn set_app(&mut self, app: Option<StatefulInfo>);
}
