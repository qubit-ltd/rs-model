// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Demographic and social classification values.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

/// Political affiliation recorded for a person in the Chinese context.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Politics {
    /// Full member of the Communist Party of China.
    CommunistPartyMember,
    /// Probationary member of the Communist Party of China.
    CommunistPartyProbationaryMember,
    /// Member of the Communist Youth League of China.
    CommunistYouthLeagueMember,
    /// Member of the Revolutionary Committee of the Chinese Kuomintang.
    KuomintangMember,
    /// Member of the China Democratic League.
    DemocraticLeagueMember,
    /// Member of the China National Democratic Construction Association.
    NationalDemocraticConstructionAssociationMember,
    /// Member of the China Association for Promoting Democracy.
    PromotingDemocracyAssociationMember,
    /// Member of the Chinese Peasants' and Workers' Democratic Party.
    PeasantsWorkersDemocraticPartyMember,
    /// Member of the China Zhi Gong Party.
    ZhiGongPartyMember,
    /// Member of the Jiusan Society.
    NineThreeAcademicSocietyMember,
    /// Member of the Taiwan Democratic Self-Government League.
    TaiwanDemocraticSelfGovernmentLeagueMember,
    /// Politically independent person.
    IndependentPolitician,
    /// Person with no party affiliation.
    Masses,
}
