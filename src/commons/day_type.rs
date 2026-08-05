// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared enumerations from the Java commons model package.

#[allow(unused_imports)]
use super::{
    AuthorizeRecord,
    Category,
    Credential,
    CredentialInfo,
    CredentialType,
    Currency,
    Kinship,
    MqType,
    Owner,
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

/// Describes which calendar dates a schedule accepts.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DayType {
    /// Any date.
    Any,
    /// Working days.
    WorkingDay,
    /// Holidays.
    Holiday,
    /// Weekends.
    Weekend,
}
