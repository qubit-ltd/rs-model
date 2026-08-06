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
        code_for(self)
    }

    /// Returns the source-domain Chinese description.
    ///
    /// # Returns
    ///
    /// The immutable display description associated with this classification.
    #[inline(always)]
    #[must_use]
    pub const fn description(self) -> &'static str {
        description_for(self)
    }
}

/// Returns the source enterprise-insurance code for a classification.
const fn code_for(insured_type: EnterpriseInsuredType) -> &'static str {
    match insured_type {
        EnterpriseInsuredType::InService => "10",
        EnterpriseInsuredType::Retired => "11",
        EnterpriseInsuredType::Resigned => "12",
        EnterpriseInsuredType::OverSeventy => "13",
        EnterpriseInsuredType::OnlyChild => "31",
        EnterpriseInsuredType::ChildDonorGenus => "32",
        EnterpriseInsuredType::DonorGenus => "41",
    }
}

/// Returns the source Chinese description for a classification.
const fn description_for(insured_type: EnterpriseInsuredType) -> &'static str {
    match insured_type {
        EnterpriseInsuredType::InService => "在职",
        EnterpriseInsuredType::Retired => "退休",
        EnterpriseInsuredType::Resigned => "退职",
        EnterpriseInsuredType::OverSeventy => "70岁以上",
        EnterpriseInsuredType::OnlyChild => "独生子女<=16",
        EnterpriseInsuredType::ChildDonorGenus => "子女供属",
        EnterpriseInsuredType::DonorGenus => "供属",
    }
}

#[cfg(test)]
mod tests {
    use super::EnterpriseInsuredType;

    /// Exercises the public forwarding API in the library test binary.
    #[test]
    fn public_code_apis_delegate_to_the_source_mappings() {
        let code: fn(EnterpriseInsuredType) -> &'static str =
            EnterpriseInsuredType::code;
        let description: fn(EnterpriseInsuredType) -> &'static str =
            EnterpriseInsuredType::description;

        assert_eq!(code(EnterpriseInsuredType::InService), "10");
        assert_eq!(description(EnterpriseInsuredType::InService), "在职");
    }
}
