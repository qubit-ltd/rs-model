// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Feedback records submitted by users about an application or service.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::App;
use crate::feedback::FeedbackStatus;
use crate::feedback::FeedbackType;
use crate::mixin::StatefulInfo;
use crate::person::User;
use crate::system::Environment;
use crate::upload::Attachment;

/// A user's complaint, report, or suggestion and its processing context.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct Feedback {
    /// Database identifier; the default value denotes feedback not yet persisted.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Tenant reference for the application receiving this submission.
    pub tenant: StatefulInfo,

    /// Application reference that receives and processes the feedback.
    #[model(reference(target = App, target_field = info), opaque)]
    pub app: StatefulInfo,

    /// Client environment captured at submission time, or `None` when unavailable.
    pub environment: Option<Environment>,

    /// Kind of issue or request selected by the submitter.
    #[model(index)]
    pub r#type: FeedbackType,

    /// Stateful category used to classify the feedback, such as service quality.
    #[model(index, opaque)]
    pub category: StatefulInfo,

    /// Submitting principal, or `None` for anonymous feedback.
    #[model(reference(target = User, target_field = info, must_exist = true), opaque)]
    pub submitter: Option<StatefulInfo>,

    /// Contact details volunteered by the submitter, or `None` if none were given.
    #[model(text(max_chars = 128))]
    pub contact: Option<String>,

    /// Short subject supplied by the user, or `None` if the submission is untitled.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub title: Option<String>,

    /// Written account of the issue; normally supplied instead of [`Self::voice`].
    pub description: Option<String>,

    /// Voice account of the issue; normally supplied instead of [`Self::description`].
    #[model(reference(target = Attachment, target_field = id, must_exist = false), opaque)]
    pub voice: Option<Attachment>,

    /// Text transcribed from [`Self::voice`], or `None` when no transcription exists.
    pub transcript: Option<String>,

    /// Supporting files, or `None` when the submitter supplied no attachments.
    #[model(reference(target = Attachment, target_field = id, must_exist = false), opaque)]
    pub attachments: Option<Vec<Attachment>>,

    /// Current stage in the feedback processing lifecycle.
    #[model(index)]
    pub status: FeedbackStatus,

    /// Administrator's internal or user-facing comment, or `None` when absent.
    pub comment: Option<String>,

    /// UTC instant, rounded to seconds, when the feedback was submitted.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the latest update, or `None` when it has not changed.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the record is retained.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
