// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Credential-reference contracts.

use crate::commons::CredentialInfo;

/// Gives a model an optional identifying credential.
pub trait WithCredential {
    /// Returns credential information, or `None` when no credential is attached.
    fn credential(&self) -> Option<&CredentialInfo>;

    /// Sets credential information; `None` removes the credential reference.
    fn set_credential(&mut self, credential: Option<CredentialInfo>);
}
