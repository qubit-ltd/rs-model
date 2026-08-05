// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Demographic and social classification values.

#[allow(unused_imports)]
use super::{
    Blood,
    Education,
    Ethnic,
    Gender,
    Incoming,
    Industry,
    JobTitle,
    Marriage,
    Person,
    PersonIdentity,
    Religion,
    SexOrientation,
    SocialNetwork,
};

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Source-domain Politics classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Politics {
    /// Source variant `COMMUNIST_PARTY_MEMBER`.
    CommunistPartyMember,
    /// Source variant `COMMUNIST_PARTY_PROBATIONARY_MEMBER`.
    CommunistPartyProbationaryMember,
    /// Source variant `COMMUNIST_YOUTH_LEAGUE_MEMBER`.
    CommunistYouthLeagueMember,
    /// Source variant `KUOMINTANG_MEMBER`.
    KuomintangMember,
    /// Source variant `DEMOCRATIC_LEAGUE_MEMBER`.
    DemocraticLeagueMember,
    /// Source variant `NATIONAL_DEMOCRATIC_CONSTRUCTION_ASSOCIATION_MEMBER`.
    NationalDemocraticConstructionAssociationMember,
    /// Source variant `PROMOTING_DEMOCRACY_ASSOCIATION_MEMBER`.
    PromotingDemocracyAssociationMember,
    /// Source variant `PEASANTS_WORKERS_DEMOCRATIC_PARTY_MEMBER`.
    PeasantsWorkersDemocraticPartyMember,
    /// Source variant `ZHI_GONG_PARTY_MEMBER`.
    ZhiGongPartyMember,
    /// Source variant `NINE_THREE_ACADEMIC_SOCIETY_MEMBER`.
    NineThreeAcademicSocietyMember,
    /// Source variant `TAIWAN_DEMOCRATIC_SELF_GOVERNMENT_LEAGUE_MEMBER`.
    TaiwanDemocraticSelfGovernmentLeagueMember,
    /// Source variant `INDEPENDENT_POLITICIAN`.
    IndependentPolitician,
    /// Source variant `MASSES`.
    Masses,
}
