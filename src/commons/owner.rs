// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared records used across the migrated model domains.

#[allow(unused_imports)]
use super::{
    AuthorizeRecord,
    Category,
    Credential,
    CredentialInfo,
    CredentialType,
    Currency,
    DayType,
    Kinship,
    MqType,
    Owners,
    Payload,
    RequestStatus,
    Source,
    VerifyState,
};

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Identifies the owner of a domain object.
#[derive(
    Clone, Debug, Default, Deserialize, Model, PartialEq, Eq, Serialize,
)]
pub struct Owner {
    /// Owning entity name.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub entity: String,
    /// Owner's persisted identifier.
    #[model(identifier)]
    pub id: Option<i64>,
}
