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
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::commons::Category;
use crate::person::User;
use crate::person::UserInfo;
use crate::task::TaskStatus;

/// Metadata describing a task's target, result, lifecycle, and creator.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct TaskInfo {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Category that classifies the task.
    #[model(reference(target = Category, target_field = info, must_exist = true), index, opaque)]
    pub category: InfoWithEntity,

    /// ASCII entity name of the target.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub target_entity: String,

    /// Persisted target identifier.
    #[model(index)]
    pub target_id: Option<i64>,

    /// Optional ASCII entity name of the result.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub result_entity: Option<String>,

    /// Optional persisted result identifier.
    #[model(index)]
    pub result_id: Option<i64>,

    /// Optional task description.
    pub description: Option<String>,

    /// Current lifecycle state.
    #[model(index)]
    pub status: TaskStatus,

    /// Optional status message.
    pub message: Option<String>,

    /// Optional UTC submission timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub submit_time: Option<DateTime<Utc>>,

    /// Optional UTC start timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub start_time: Option<DateTime<Utc>>,

    /// Optional UTC cancellation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub cancel_time: Option<DateTime<Utc>>,

    /// Optional UTC finish timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub finish_time: Option<DateTime<Utc>>,

    /// Optional creator; absent for system-created tasks.
    #[model(reference(target = User, target_field = info, must_exist = true), opaque)]
    #[redact(nested)]
    pub creator: Option<UserInfo>,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,
}
