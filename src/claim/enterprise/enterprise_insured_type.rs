// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise insured-person classifications.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Identifies an insured person's employment or dependent classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnterpriseInsuredType {
    /// Active employee.
    InService,
    /// Retired employee.
    Retired,
    /// Resigned employee.
    Resigned,
    /// Insured person over seventy years old.
    OverSeventy,
    /// Only child no older than sixteen.
    OnlyChild,
    /// Child dependent.
    ChildDonorGenus,
    /// Other dependent.
    DonorGenus,
}

impl EnterpriseInsuredType {
    /// Returns the external enterprise-insurance code.
    ///
    /// # Returns
    ///
    /// The immutable source-domain code for this classification.
    #[inline(always)]
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::InService => "10",
            Self::Retired => "11",
            Self::Resigned => "12",
            Self::OverSeventy => "13",
            Self::OnlyChild => "31",
            Self::ChildDonorGenus => "32",
            Self::DonorGenus => "41",
        }
    }

    /// Returns the source-domain Chinese description.
    ///
    /// # Returns
    ///
    /// The immutable display description associated with this classification.
    #[inline(always)]
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::InService => "在职",
            Self::Retired => "退休",
            Self::Resigned => "退职",
            Self::OverSeventy => "70岁以上",
            Self::OnlyChild => "独生子女<=16",
            Self::ChildDonorGenus => "子女供属",
            Self::DonorGenus => "供属",
        }
    }
}
