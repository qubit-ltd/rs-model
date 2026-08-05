// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Polymorphic hospital-information-system records.

use serde::{
    Deserialize,
    Serialize,
};

use crate::medical::{
    ClinicInfo,
    EmergentClinicInfo,
    ExaminationInfo,
    HospitalizationInfo,
    RegistrationInfo,
    SpecificClinicInfo,
};

/// A typed hospital-information-system payload.
///
/// The enum discriminator carries the Java base class's immutable `type`
/// property, while each payload carries the shared `number` and `remark`
/// properties together with its specialized fields.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum HisInfo {
    /// General clinic visit information.
    Clinic(ClinicInfo),
    /// Special outpatient clinic visit information.
    SpecificClinic(SpecificClinicInfo),
    /// Emergency clinic visit information.
    EmergentClinic(EmergentClinicInfo),
    /// Medical examination information.
    Examination(ExaminationInfo),
    /// Hospitalization information.
    Hospitalization(HospitalizationInfo),
    /// Registration information.
    Registration(RegistrationInfo),
}
