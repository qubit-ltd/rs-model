// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Accessors for attachment collections.

use crate::upload::Attachment;

/// Provides an optional ordered attachment collection.
pub trait WithAttachments {
    /// Returns the current attachments.
    fn attachments(&self) -> Option<&[Attachment]>;

    /// Replaces the current attachments.
    fn set_attachments(&mut self, attachments: Option<Vec<Attachment>>);
}
