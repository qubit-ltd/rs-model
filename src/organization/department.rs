// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Department domain models.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

use crate::commons::Category;
use crate::commons::Payload;
use crate::commons::State;
use crate::contact::Contact;
use crate::mixin::StatefulInfo;
use crate::organization::Organization;

/// Represents an organizational department and its hierarchy, contact, and payload data.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
#[model(
    unique(
        name = "department_organization_internal_code",
        fields(organization, internal_code),
        ignore_case(internal_code)
    ),
    unique(
        name = "department_organization_name",
        fields(organization, name),
        ignore_case(name)
    )
)]
pub struct Department {
    /// Database identifier for this department; default denotes an unsaved record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique ASCII department code.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Organization-unique code used to address the department in integrations.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub internal_code: Option<String>,

    /// Organization-scoped department name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Category used to group this department in organizational navigation and reporting.
    #[model(reference(target = Category, target_field = info), index, opaque)]
    pub category: Option<InfoWithEntity>,

    /// Parent department that establishes this department's place in the hierarchy.
    #[model(reference(target = Department, target_field = info), index)]
    pub parent: Option<StatefulInfo>,

    /// Organization information.
    #[model(reference(target = Organization, target_field = info), index)]
    pub organization: StatefulInfo,

    /// Lifecycle state.
    #[model(index)]
    pub state: State,

    /// Icon URI used when presenting the department in organizational interfaces.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,

    /// Description of the department's responsibilities or service scope.
    pub description: Option<String>,

    /// Contact channels for the department rather than an individual employee.
    #[model(index, opaque)]
    pub contact: Option<Contact>,

    /// Application-defined extension values associated with this department.
    #[model(
        reference(target = Payload, target_field = id, must_exist = false),
        sequence(min_items = 1, max_items = 10)
    )]
    pub payloads: Option<Vec<Payload>>,

    /// Marks a platform-defined department that is protected from ordinary maintenance.
    pub predefined: bool,

    /// Marks non-production departmental data excluded from live operations.
    #[model(index)]
    pub test: bool,

    /// UTC instant when this department record was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the most recent department update.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC instant of soft deletion; absent while the department is active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
