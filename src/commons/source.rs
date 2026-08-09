// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared records used across the migrated model domains.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;

use super::App;
use super::Category;
use crate::mixin::StatefulInfo;
use crate::organization::Organization;

/// Source-system metadata for an imported record.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[model(unique(
    name = "source_app_entity_name",
    fields(app, entity, name),
    ignore_case(name)
))]
pub struct Source {
    /// Platform-assigned identifier of this import-source record.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Importing application information.
    #[model(reference(target = App, target_field = info), index)]
    pub app: StatefulInfo,

    /// Entity discriminator.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,

    /// Source code.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Source name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional source description.
    pub description: Option<String>,

    /// Optional category information.
    #[model(reference(target = Category, target_field = info), opaque)]
    pub category: Option<InfoWithEntity>,

    /// Optional provider application information.
    #[model(reference(target = App, target_field = info))]
    pub provider_app: Option<StatefulInfo>,

    /// Optional provider organization information.
    #[model(reference(target = Organization, target_field = info))]
    pub provider_org: Option<StatefulInfo>,

    /// Whether this source is predefined.
    #[model(index)]
    pub predefined: bool,

    /// Optional UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
