// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Privilege wire-codec failures.

use thiserror::Error;

/// Errors produced when decoding a permission list.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum PrivilegesCodecError {
    /// A list contains a privilege name that is empty after trimming.
    #[error("privilege at index {index} is empty")]
    EmptyPrivilege {
        /// Zero-based element index.
        index: usize,
    },
    /// A privilege contains the comma delimiter and cannot be encoded
    /// unambiguously.
    #[error("privilege at index {index} contains a comma")]
    ContainsSeparator {
        /// Zero-based element index.
        index: usize,
    },
}
