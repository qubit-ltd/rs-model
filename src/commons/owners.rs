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
    AuthorizeRecord, Category, Credential, CredentialInfo, CredentialType, Currency, DayType,
    Kinship, MqType, Owner, Payload, RequestStatus, Source, VerifyState,
};

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// A set of owners represented by their entity and identifier pairs.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Eq, Serialize)]
pub struct Owners {
    /// Owners in source order.
    pub values: Vec<Owner>,
}
