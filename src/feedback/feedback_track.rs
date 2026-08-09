// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Audit-trail entries for actions applied to feedback cases.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::feedback::Feedback;
use crate::feedback::FeedbackAction;
use crate::feedback::FeedbackRating;
use crate::feedback::FeedbackStatus;
use crate::person::User;
use crate::person::UserInfo;
use crate::upload::Attachment;

/// A history entry describing one action and its feedback state transition.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct FeedbackTrack {
    /// Database identifier; the default value denotes an unpersisted history entry.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Identifier of the [`Feedback`] case affected by this action.
    #[model(reference(target = Feedback, target_field = id), opaque)]
    pub feedback_id: Id,

    /// Lifecycle action that produced this history entry.
    #[model(index)]
    pub action: FeedbackAction,

    /// User who submitted or performed the action.
    #[model(reference(target = User, target_field = info, must_exist = true), opaque)]
    pub submitter: UserInfo,

    /// Written explanation accompanying the action, normally used instead of [`Self::record`].
    pub description: Option<String>,

    /// Voice explanation accompanying the action, normally used instead of [`Self::description`].
    #[model(reference(target = Attachment, target_field = id, must_exist = false), opaque)]
    pub record: Option<Attachment>,

    /// Supporting files for this action, or `None` if no files were supplied.
    #[model(reference(target = Attachment, target_field = id, must_exist = false), opaque)]
    pub attachments: Option<Vec<Attachment>>,

    /// Feedback lifecycle state immediately before the action.
    #[model(index)]
    pub status_before: FeedbackStatus,

    /// Feedback lifecycle state immediately after the action.
    #[model(index)]
    pub status_after: FeedbackStatus,

    /// Submitter rating for actions that request it, or `None` otherwise.
    #[model(index)]
    pub rating: Option<FeedbackRating>,

    /// Administrator comment attached to this action, or `None` when absent.
    pub comment: Option<String>,

    /// UTC instant, rounded to seconds, when the action was recorded.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the latest history-entry change, or `None` when unchanged.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the entry is retained.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
