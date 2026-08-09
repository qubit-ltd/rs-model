// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! JSON-object wrappers for scalar REST response values.

use serde::Deserialize;
use serde::Serialize;

/// Wraps a response value so scalar results serialize as JSON objects.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ResultValue<T> {
    /// The response payload represented by this object.
    pub value: T,
}

/// Java-compatible alias for [`ResultValue`].
pub type Result<T> = ResultValue<T>;

impl<T> ResultValue<T> {
    /// Creates a wrapper that owns the supplied response payload.
    #[must_use]
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Consumes the wrapper and returns its response payload.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }
}
