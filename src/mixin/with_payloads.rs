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
    WithOwner,
    WithSource,
    WithToken,
};

use crate::commons::Payload;

/// Provides payloads.
pub trait WithPayloads {
    /// Returns the payloads.
    fn payloads(&self) -> Option<&[Payload]>;

    /// Replaces the payloads.
    fn set_payloads(&mut self, payloads: Option<Vec<Payload>>);
}
