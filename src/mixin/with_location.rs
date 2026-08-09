// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Geographic-location contracts.

use crate::contact::Location;

/// Gives a model an optional geographic coordinate.
pub trait WithLocation {
    /// Returns the location, or `None` when its position is unknown.
    fn location(&self) -> Option<&Location>;

    /// Sets the location; `None` explicitly records an unknown position.
    fn set_location(&mut self, location: Option<Location>);
}
