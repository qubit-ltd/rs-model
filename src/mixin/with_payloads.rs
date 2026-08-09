// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Extensible-payload contracts.

use crate::commons::Payload;

/// Gives a model an optional, ordered set of extension payloads.
pub trait WithPayloads {
    /// Returns payloads in stored order, or `None` when no payload collection was loaded.
    fn payloads(&self) -> Option<&[Payload]>;

    /// Replaces the payload collection; `None` clears it.
    fn set_payloads(&mut self, payloads: Option<Vec<Payload>>);
}
