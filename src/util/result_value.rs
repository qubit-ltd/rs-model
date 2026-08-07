// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! REST response value wrappers.

use serde::Deserialize;
use serde::Serialize;

/// Wraps one REST response value so it serializes as a JSON object.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResultValue<T> {
    /// Wrapped response value.
    pub value: T,
}

/// Java-compatible public name for [`ResultValue`].
pub type Result<T> = ResultValue<T>;

impl<T> ResultValue<T> {
    /// Creates a response wrapper for `value`.
    ///
    /// # Parameters
    ///
    /// * `value` - The response value to wrap.
    ///
    /// # Returns
    ///
    /// A response wrapper that owns `value`.
    #[must_use]
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Consumes this wrapper and returns its response value.
    ///
    /// # Returns
    /// The value previously held by this wrapper.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }
}
