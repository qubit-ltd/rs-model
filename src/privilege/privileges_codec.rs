// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Java-compatible privilege wire adapter.

use crate::privilege::Privileges;

/// Java-compatible comma-separated wire adapter for [`Privileges`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PrivilegesCodec;

impl PrivilegesCodec {
    /// Decodes the legacy comma-separated privilege representation.
    ///
    /// `None` remains `None`, while `Some("")` becomes `Some(Privileges::default())`. For a
    /// non-empty value, surrounding whitespace and empty comma-separated segments are discarded.
    /// This is intentionally lossy: original spacing, blank segments, and delimiter placement
    /// cannot be recovered.
    #[must_use]
    pub fn decode(value: Option<&str>) -> Option<Privileges> {
        value.map(|value| {
            if value.is_empty() {
                return Privileges::default();
            }
            Privileges(
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|part| !part.is_empty())
                    .map(ToOwned::to_owned)
                    .collect(),
            )
        })
    }

    /// Encodes a privilege list with comma separators without validation.
    ///
    /// `None` remains `None`; an empty collection becomes `Some("")`. This adapter does not
    /// escape commas or preserve surrounding whitespace, so lists containing commas, blank names,
    /// or whitespace-significant names cannot round-trip through [`Self::decode`]. Use
    /// [`Privileges::encode`] when reversible, validated encoding is required.
    #[must_use]
    pub fn encode(value: Option<&Privileges>) -> Option<String> {
        value.map(|privileges| privileges.0.join(","))
    }
}
