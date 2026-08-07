// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Department domain models.

use chrono::{DateTime, Utc};
use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

use crate::commons::{Payload, State};
use crate::contact::Contact;
use crate::mixin::StatefulInfo;

/// An organizational department and its hierarchy, contact, and payload data.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct Department {
    /// Optional persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,

    /// Globally unique ASCII department code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Optional organization-scoped ASCII code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub internal_code: Option<String>,

    /// Organization-scoped department name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional category reference information.
    #[model(opaque)]
    pub category: Option<InfoWithEntity>,

    /// Optional parent-department information.
    pub parent: Option<StatefulInfo>,

    /// Organization information.
    pub organization: StatefulInfo,

    /// Lifecycle state.
    pub state: State,

    /// Optional ASCII icon URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,

    /// Optional description.
    pub description: Option<String>,

    /// Optional contact methods.
    pub contact: Option<Contact>,

    /// Optional payload values.
    pub payloads: Option<Vec<Payload>>,

    /// Whether this is predefined reference data.
    pub predefined: bool,

    /// Whether this is a test department.
    pub test: bool,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
