// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Organization-ownership contracts.

use super::StatefulInfo;

/// Gives a model an optional reference to its owning organization.
pub trait WithOrganization {
    /// Returns the organization snapshot, or `None` when no organization owns the model.
    fn organization(&self) -> Option<&StatefulInfo>;

    /// Sets the organization snapshot; `None` removes the ownership reference.
    fn set_organization(&mut self, organization: Option<StatefulInfo>);
}
