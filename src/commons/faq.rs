// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Frequently asked questions.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Emptyful;
use qubit_mixin::Info;
use qubit_mixin::InfoWithEntity;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::mixin::StatefulInfo;
use super::App;
use super::Category;
use super::State;

/// A frequently asked question associated with an application and product.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
pub struct Faq {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Owning application information.
    #[model(reference(target = App, target_field = info), opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<StatefulInfo>,

    /// Optional category information.
    #[model(reference(target = Category, target_field = info), opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<InfoWithEntity>,

    /// Product information.
    #[model(opaque)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Info>,

    /// Question text.
    pub question: String,

    /// Answer text.
    pub answer: String,

    /// Number of times the question was viewed.
    pub frequency: i64,

    /// Lifecycle state.
    pub state: State,

    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Faq {
    /// Returns whether every source property has its empty representation.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id.is_none()
            && self.app.is_none()
            && self.category.is_none()
            && self.product.is_none()
            && self.question.is_empty()
            && self.answer.is_empty()
            && self.frequency == 0
            && self.state == State::Normal
            && self.create_time.is_none()
            && self.modify_time.is_none()
            && self.delete_time.is_none()
    }
}

impl Emptyful for Faq {
    fn is_empty(&self) -> bool {
        Self::is_empty(self)
    }
}

impl Normalizable for Faq {
    fn normalize(&mut self) {
        self.question.normalize();
        self.answer.normalize();
    }

    fn is_normalized_empty(&self) -> bool {
        self.is_empty()
    }
}
