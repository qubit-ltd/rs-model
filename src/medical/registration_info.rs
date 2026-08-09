// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! HIS details captured when a patient registers for care.

use chrono::DateTime;
use chrono::Utc;
use serde::Deserialize;
use serde::Serialize;

use qubit_mixin::Info;
use qubit_model_derive::Model;

/// Source-system record for a patient's registration with a department.
#[derive(Model, Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct RegistrationInfo {
    /// Source-system business sequence number.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub number: String,

    /// Source-system registration note, absent when no remark was recorded.
    pub remark: Option<String>,

    /// Registered department.
    #[model(opaque)]
    pub department: Info,

    /// UTC registration timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub register_time: DateTime<Utc>,
}
