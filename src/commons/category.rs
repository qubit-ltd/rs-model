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

/// A common category record.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[model(unique(name = "category_entity_name", fields(entity, name), ignore_case(name)))]
pub struct Category {
    /// Platform-assigned identifier of this common category.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Entity discriminator.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,

    /// Globally unique category code.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Category name.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional icon URL or key.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,

    /// Optional explanatory text for users selecting this category.
    pub description: Option<String>,

    /// Query-computed display title.
    #[model(text(min_chars = 1, max_chars = 4096))]
    pub title: Option<String>,

    /// Optional parent category information.
    #[model(reference(target = Category, target_field = info), index, opaque)]
    pub parent: Option<InfoWithEntity>,

    /// Whether this record is predefined.
    #[model(index)]
    pub predefined: bool,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
