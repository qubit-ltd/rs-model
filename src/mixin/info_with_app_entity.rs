// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Basic entity information associated with an application.

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use crate::mixin::{
    StatefulInfo,
    WithApp,
};

/// Basic information for an entity that belongs to an application.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Redact, Serialize,
)]
pub struct InfoWithAppEntity {
    /// Basic identifying information and entity discriminator.
    #[model(opaque)]
    #[redact(skip)]
    #[serde(flatten)]
    pub info: InfoWithEntity,
    /// Optional application that owns the entity.
    #[redact(skip)]
    pub app: Option<StatefulInfo>,
}

impl InfoWithAppEntity {
    /// Creates a composed information value.
    #[must_use]
    pub const fn new(info: InfoWithEntity, app: Option<StatefulInfo>) -> Self {
        Self { info, app }
    }

    /// Reports whether both the identifying information and application are
    /// present.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.info.is_complete() && self.app.is_some()
    }
}

impl WithApp for InfoWithAppEntity {
    fn app(&self) -> Option<&StatefulInfo> {
        self.app.as_ref()
    }

    fn set_app(&mut self, app: Option<StatefulInfo>) {
        self.app = app;
    }
}
