// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Complete person records.

use crate::commons::CredentialInfo;

/// Supplies the identity fields used to compare people across projections.
pub trait PersonIdentity {
    /// Returns the persisted person identifier, if any.
    fn person_id(&self) -> Option<i64>;

    /// Returns the identity credential, if any.
    fn person_credential(&self) -> Option<&CredentialInfo>;
}
