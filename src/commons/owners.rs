// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared records used across the migrated model domains.

use qubit_id::Id;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// A set of owners represented by their entity and identifier pairs.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
#[model(key(name = "owners_key", fields(type, ids, property)))]
pub struct Owners {
    /// Owning entity type name.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub r#type: String,

    /// Persisted identifiers of the owned entities.
    #[model(opaque)]
    pub ids: Vec<Id>,

    /// Optional property owned on the target entities.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub property: Option<String>,
}
