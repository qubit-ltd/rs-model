// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Entity identity projections that retain application ownership.

use serde::Deserialize;

use qubit_mixin::InfoWithEntity;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

use crate::mixin::StatefulInfo;
use crate::mixin::WithApp;

/// Entity identity information paired with its optional owning application.
#[derive(Model, Redact, Clone, Default, Deserialize, PartialEq)]
#[redact(debug, display, serde)]
pub struct InfoWithAppEntity {
    /// Compact identity of the referenced entity, including its entity discriminator.
    #[model(opaque)]
    #[redact(plain)]
    pub info: InfoWithEntity,

    /// Owning application snapshot, or `None` when ownership is unknown.
    #[redact(skip)]
    pub app: Option<StatefulInfo>,
}

impl InfoWithAppEntity {
    /// Creates an entity-and-application identity projection.
    #[must_use]
    pub const fn new(info: InfoWithEntity, app: Option<StatefulInfo>) -> Self {
        Self { info, app }
    }

    /// Returns `true` only when entity information is complete and ownership is present.
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
