// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Source-provenance contracts.

use qubit_mixin::InfoWithEntity;

/// Gives a model an optional source-system classification.
pub trait WithSource {
    /// Returns the source snapshot, or `None` when provenance is not known.
    fn source(&self) -> Option<&InfoWithEntity>;

    /// Sets the source snapshot; `None` clears provenance.
    fn set_source(&mut self, source: Option<InfoWithEntity>);
}
