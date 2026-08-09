// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Category-classification contracts.

use qubit_mixin::InfoWithEntity;

/// Gives a model an optional category reference.
pub trait WithCategory {
    /// Returns the category snapshot, or `None` when the model is uncategorized.
    fn category(&self) -> Option<&InfoWithEntity>;

    /// Sets the category snapshot; `None` removes the classification.
    fn set_category(&mut self, category: Option<InfoWithEntity>);
}
