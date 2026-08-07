// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Rust traits for shared Java model mixins.

use chrono::DateTime;
use chrono::Utc;


/// Provides an optional expiration timestamp.
pub trait Expirable {
    /// Returns the expiration timestamp.
    fn expired(&self) -> Option<DateTime<Utc>>;

    /// Replaces the expiration timestamp.
    fn set_expired(&mut self, expired: Option<DateTime<Utc>>);
}
