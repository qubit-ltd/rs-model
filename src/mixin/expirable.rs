// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Expiration-time contracts for domain models.

use chrono::DateTime;
use chrono::Utc;

/// Gives a model an optional UTC expiry boundary.
pub trait Expirable {
    /// Returns the UTC expiry time, or `None` when no expiry has been set.
    fn expired(&self) -> Option<DateTime<Utc>>;

    /// Sets the UTC expiry time; `None` removes the expiry boundary.
    fn set_expired(&mut self, expired: Option<DateTime<Utc>>);
}
