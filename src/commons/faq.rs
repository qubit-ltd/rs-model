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
use qubit_id::Id;
use serde::Deserialize;

use qubit_mixin::Emptyful;
use qubit_mixin::Info;
use qubit_mixin::InfoWithEntity;
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use super::App;
use super::Category;
use super::State;
use crate::mixin::StatefulInfo;

/// A frequently asked question associated with an application and product.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
#[serde(default)]
pub struct Faq {
    /// Platform-assigned identifier of this frequently asked question.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Owning application information.
    #[model(reference(target = App, target_field = info), opaque)]
    pub app: Option<StatefulInfo>,

    /// Optional category information.
    #[model(reference(target = Category, target_field = info), opaque)]
    pub category: Option<InfoWithEntity>,

    /// Product information.
    #[model(opaque)]
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
    pub create_time: Option<DateTime<Utc>>,

    /// Optional UTC modification timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// Optional UTC deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Faq {
    /// Returns whether every source property has its empty representation.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.id == Id::default()
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
