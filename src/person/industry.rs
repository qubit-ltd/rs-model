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

/// Source-domain Industry classification.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Industry {
    /// Source variant `AGRICULTURE_FORESTRY_PASTORAL_FISHERY`.
    AgricultureForestryPastoralFishery,
    /// Source variant `MINING`.
    Mining,
    /// Source variant `MANUFACTURING`.
    Manufacturing,
    /// Source variant `ELECTRICITY_HEAT_GAS_AND_WATER`.
    ElectricityHeatGasAndWater,
    /// Source variant `CONSTRUCTION`.
    Construction,
    /// Source variant `SALES`.
    Sales,
    /// Source variant `TRANSPORTATION_WAREHOUSING_AND_POSTAL`.
    TransportationWarehousingAndPostal,
    /// Source variant `ACCOMMODATION_AND_CATERING`.
    AccommodationAndCatering,
    /// Source variant `INFORMATION`.
    Information,
    /// Source variant `FINANCE`.
    Finance,
    /// Source variant `REAL_ESTATE`.
    RealEstate,
    /// Source variant `LEASING_AND_BUSINESS_SERVICES`.
    LeasingAndBusinessServices,
    /// Source variant `RESEARCH_AND_TECHNOLOGY`.
    ResearchAndTechnology,
    /// Source variant `CONSERVANCY_ENVIRONMENT_AND_PUBLIC_FACILITIES`.
    ConservancyEnvironmentAndPublicFacilities,
    /// Source variant `RESIDENTIAL_SERVICES`.
    ResidentialServices,
    /// Source variant `EDUCATION`.
    Education,
    /// Source variant `HEALTH_AND_SOCIAL_WORK`.
    HealthAndSocialWork,
    /// Source variant `CULTURE_SPORTS_AND_ENTERTAINMENT`.
    CultureSportsAndEntertainment,
    /// Source variant `PUBLIC_ADMIN_SOCIAL_SECURITY_AND_SOCIAL_ORGANIZATIONS`.
    PublicAdminSocialSecurityAndSocialOrganizations,
    /// Source variant `INTERNATIONAL_ORGANIZATIONS`.
    InternationalOrganizations,
}
