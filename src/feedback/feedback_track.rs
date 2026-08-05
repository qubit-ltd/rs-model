// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Persisted feedback lifecycle events.

use chrono::{
    DateTime,
    Utc,
};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use crate::feedback::{
    FeedbackAction,
    FeedbackRating,
    FeedbackStatus,
};
use crate::person::UserInfo;
use crate::upload::Attachment;

/// An immutable-in-purpose record of one operation applied to feedback.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct FeedbackTrack {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
    /// Identifier of the feedback record being tracked.
    #[model(identifier)]
    pub feedback_id: i64,
    /// Operation that produced this event.
    pub action: FeedbackAction,
    /// User that submitted the event.
    pub submitter: UserInfo,
    /// Optional textual description; either this or `record` is expected.
    pub description: Option<String>,
    /// Optional voice description; either this or `description` is expected.
    pub record: Option<Attachment>,
    /// Optional supporting attachments.
    pub attachments: Option<Vec<Attachment>>,
    /// Feedback state immediately before the operation.
    pub status_before: FeedbackStatus,
    /// Feedback state immediately after the operation.
    pub status_after: FeedbackStatus,
    /// Optional user rating for applicable user actions.
    pub rating: Option<FeedbackRating>,
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
