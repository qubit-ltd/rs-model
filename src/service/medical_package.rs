// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Medical service packages.

use chrono::DateTime;
use chrono::Utc;
use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::mixin::StatefulInfo;
use crate::service::MedicalPackageItem;

/// A named package of medical service items belonging to an organization.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct MedicalPackage {
    /// Persistent identifier; its default value denotes a record that has not yet been stored.
    #[model(identifier)]
    #[model(opaque)]
    pub id: Id,

    /// Globally unique package code.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub code: String,

    /// Package name.
    #[model(text(min_chars = 1, max_chars = 128))]
    pub name: String,

    /// Optional package description.
    pub description: Option<String>,

    /// Optional included service items.
    pub items: Option<Vec<MedicalPackageItem>>,

    /// Organization that owns the package.
    pub organization: StatefulInfo,

    /// UTC instant at which this record was created.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,

    /// UTC instant of the most recent update, or `None` when no update has occurred.
    #[model(time(precision = second, normalization = utc))]
    pub modify_time: Option<DateTime<Utc>>,

    /// UTC soft-deletion instant, or `None` while the record remains active.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
