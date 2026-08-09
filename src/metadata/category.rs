// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Hierarchical metadata categories.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Emptyful;
use qubit_mixin::InfoWithEntity;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::Scope;

/// A hierarchical category associated with an entity type.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
#[model(unique(name = "category_entity_name", fields(entity, name), ignore_case(name)))]
pub struct Category {
    /// Platform-assigned identifier of this metadata category.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique ASCII code.
    #[model(unique(ignore_case), text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Name unique within the associated entity.
    #[model(index, text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Ownership scope.
    #[model(index)]
    pub scope: Option<Scope>,

    /// Associated entity name.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,

    /// Optional icon URI or key.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    pub icon: Option<String>,

    /// Optional explanatory text shown with this metadata category.
    pub description: Option<String>,

    /// Query-computed hierarchical title.
    #[model(text(min_chars = 1, max_chars = 4096))]
    pub title: Option<String>,

    /// Optional parent-category information.
    #[model(reference(target = Category, target_field = info), index, opaque)]
    pub parent: Option<InfoWithEntity>,

    /// Whether the category is predefined.
    #[model(index)]
    pub predefined: bool,

    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Category {
    /// Separator used to join names into a hierarchical title.
    pub const TITLE_JOINER: &'static str = " - ";

    /// Returns whether the category has no identifying content.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id == Id::default()
            && self.code.is_empty()
            && self.name.is_empty()
            && self.scope.is_none()
            && self.entity.is_empty()
            && self.icon.as_ref().is_none_or(String::is_empty)
            && self.description.as_ref().is_none_or(String::is_empty)
            && self.title.as_ref().is_none_or(String::is_empty)
            && self.parent.as_ref().is_none_or(Emptyful::is_empty)
            && !self.predefined
            && self.create_time.is_none()
            && self.modify_time.is_none()
            && self.delete_time.is_none()
    }
}

impl Emptyful for Category {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for Category {
    fn normalize(&mut self) {
        self.code.normalize();
        self.name.normalize();
        self.entity.normalize();
        self.icon.normalize();
        self.description.normalize();
        self.title.normalize();
        self.parent.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}
