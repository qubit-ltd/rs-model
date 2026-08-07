// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rust traits for shared Java model mixins.

use super::StatefulInfo;

/// Provides an owning organization.
pub trait WithOrganization {
    /// Returns the organization information.
    fn organization(&self) -> Option<&StatefulInfo>;

    /// Replaces the organization information.
    fn set_organization(&mut self, organization: Option<StatefulInfo>);
}
