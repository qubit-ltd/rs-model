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
    WithLocation,
    WithMobile,
    WithOrganization,
    WithOwner,
    WithPayloads,
    WithSource,
    WithToken,
};

use crate::commons::CredentialInfo;

/// Provides credential information.
pub trait WithCredential {
    /// Returns the credential information.
    fn credential(&self) -> Option<&CredentialInfo>;

    /// Replaces the credential information.
    fn set_credential(&mut self, credential: Option<CredentialInfo>);
}
