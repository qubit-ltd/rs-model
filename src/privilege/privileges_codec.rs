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
    /// Decodes a comma-separated privilege list, ignoring blank elements.
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

    /// Encodes a privilege list with comma separators.
    #[must_use]
    pub fn encode(value: Option<&Privileges>) -> Option<String> {
        value.map(|privileges| privileges.0.join(","))
    }
}
