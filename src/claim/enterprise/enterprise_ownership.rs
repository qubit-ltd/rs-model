// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Enterprise ownership classifications.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Identifies the ownership program associated with an enterprise claim.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EnterpriseOwnership {
    /// Yangtze program.
    Yangtze,
    /// Reformed-enterprise program.
    Reform,
    /// Co-solution program.
    CoSolution,
    /// Test program.
    Test,
}

impl EnterpriseOwnership {
    /// Returns the external ownership code.
    ///
    /// # Returns
    ///
    /// The immutable source-domain code for this ownership classification.
    #[inline(always)]
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Yangtze => "1",
            Self::Reform => "0",
            Self::CoSolution => "2",
            Self::Test => "z",
        }
    }

    /// Returns the source-domain Chinese description.
    ///
    /// # Returns
    ///
    /// The immutable display description associated with this ownership.
    #[inline(always)]
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::Yangtze => "扬子",
            Self::Reform => "改制",
            Self::CoSolution => "协解",
            Self::Test => "测试",
        }
    }
}
