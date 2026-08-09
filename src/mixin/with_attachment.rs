// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Accessors for a single attachment.

use crate::upload::Attachment;

/// Gives a model one optional attachment.
pub trait WithAttachment {
    /// Returns the attachment, or `None` when no file is associated.
    fn attachment(&self) -> Option<&Attachment>;

    /// Sets the attachment; `None` removes the association.
    fn set_attachment(&mut self, attachment: Option<Attachment>);
}
