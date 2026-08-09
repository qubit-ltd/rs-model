// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Persistable metadata for one task.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::Category;
use crate::person::User;
use crate::person::UserInfo;
use crate::task::TaskStatus;

/// Metadata describing a task's target, result, lifecycle, and creator.
#[derive(Model, Redact, Clone, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct TaskInfo {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Category that classifies the task.
    #[model(reference(target = Category, target_field = info, must_exist = true), index, opaque)]
    pub category: InfoWithEntity,

    /// ASCII entity name of the target.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub target_entity: String,

    /// Persisted target identifier.
    #[model(index, opaque)]
    pub target_id: Id,

    /// Optional ASCII entity name of the result.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub result_entity: Option<String>,

    /// Identifier of the result; its default value means that no related record is stored.
    #[model(index, opaque)]
    pub result_id: Id,

    /// Optional task description.
    pub description: Option<String>,

    /// Current lifecycle state.
    #[model(index)]
    pub status: TaskStatus,

    /// Optional status message.
    pub message: Option<String>,

    /// UTC instant at which it was submitted, or `None` before submission.
    #[model(index, time(precision = second, normalization = utc))]
    pub submit_time: Option<DateTime<Utc>>,

    /// UTC instant at which execution started, or `None` before it starts.
    #[model(index, time(precision = second, normalization = utc))]
    pub start_time: Option<DateTime<Utc>>,

    /// UTC instant at which it was cancelled, or `None` unless cancellation occurred.
    #[model(index, time(precision = second, normalization = utc))]
    pub cancel_time: Option<DateTime<Utc>>,

    /// UTC instant at which execution finished, or `None` until it reaches a terminal state.
    #[model(index, time(precision = second, normalization = utc))]
    pub finish_time: Option<DateTime<Utc>>,

    /// Optional creator; absent for system-created tasks.
    #[model(reference(target = User, target_field = info, must_exist = true), opaque)]
    #[redact(nested)]
    pub creator: Option<UserInfo>,

    /// UTC instant at which this record was created.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the most recent update, or `None` when no update has occurred.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
}
