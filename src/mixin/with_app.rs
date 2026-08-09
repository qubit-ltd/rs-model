// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Application-ownership contracts.

use super::StatefulInfo;

/// Gives a model an optional reference to its owning application.
pub trait WithApp {
    /// Returns the owning application snapshot, or `None` when ownership is not assigned.
    fn app(&self) -> Option<&StatefulInfo>;

    /// Sets the owning application snapshot; `None` removes the ownership reference.
    fn set_app(&mut self, app: Option<StatefulInfo>);
}
