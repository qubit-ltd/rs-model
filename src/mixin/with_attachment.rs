// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Accessors for a single attachment.

use crate::upload::Attachment;

/// Provides an optional attachment.
pub trait WithAttachment {
    /// Returns the current attachment.
    fn attachment(&self) -> Option<&Attachment>;

    /// Replaces the current attachment.
    fn set_attachment(&mut self, attachment: Option<Attachment>);
}
