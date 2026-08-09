// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Accessors for attachment collections.

use crate::upload::Attachment;

/// Gives a model an optional, ordered collection of attachments.
pub trait WithAttachments {
    /// Returns the attachments in stored order, or `None` when the collection is absent.
    fn attachments(&self) -> Option<&[Attachment]>;

    /// Replaces the full ordered collection; `None` clears the collection.
    fn set_attachments(&mut self, attachments: Option<Vec<Attachment>>);
}
