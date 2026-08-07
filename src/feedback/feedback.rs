// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Persisted user feedback records.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::feedback::{FeedbackStatus, FeedbackType};
use crate::mixin::StatefulInfo;
use crate::system::Environment;
use crate::upload::Attachment;

/// A user's feedback, including complaints, reports, and suggestions.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Feedback {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Tenant that owns the application receiving the feedback.
    pub tenant: StatefulInfo,

    /// Application receiving the feedback.
    pub app: StatefulInfo,

    /// Optional environment captured when the feedback was submitted.
    pub environment: Option<Environment>,

    /// User-facing feedback type.
    pub r#type: FeedbackType,

    /// Specific feedback category, such as service or product quality.
    pub category: StatefulInfo,

    /// Optional principal that submitted the feedback.
    pub submitter: Option<StatefulInfo>,

    /// Optional contact details supplied by the user.
    #[model(text(max_chars = 128))]
    pub contact: Option<String>,

    /// Optional feedback title.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub title: Option<String>,

    /// Optional textual description; either this or `voice` is expected.
    pub description: Option<String>,

    /// Optional voice recording; either this or `description` is expected.
    pub voice: Option<Attachment>,

    /// Optional transcription of `voice`.
    pub transcript: Option<String>,

    /// Optional supporting attachments.
    pub attachments: Option<Vec<Attachment>>,

    /// Current processing state.
    pub status: FeedbackStatus,

    /// Optional administrative comment.
    pub comment: Option<String>,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
