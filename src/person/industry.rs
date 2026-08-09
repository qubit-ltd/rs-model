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

/// Industry in which a person works.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Industry {
    /// Agriculture, forestry, animal husbandry, or fishery.
    AgricultureForestryPastoralFishery,
    /// Mining and quarrying as the person's primary employment sector.
    Mining,
    /// Manufacturing as the person's primary employment sector.
    Manufacturing,
    /// Electricity, heat, gas, or water supply.
    ElectricityHeatGasAndWater,
    /// Construction as the person's primary employment sector.
    Construction,
    /// Wholesale or retail sales.
    Sales,
    /// Transportation, warehousing, or postal services.
    TransportationWarehousingAndPostal,
    /// Accommodation or food services.
    AccommodationAndCatering,
    /// Information services.
    Information,
    /// Financial services.
    Finance,
    /// Real-estate services as the person's primary employment sector.
    RealEstate,
    /// Leasing or business services.
    LeasingAndBusinessServices,
    /// Scientific research or technical services.
    ResearchAndTechnology,
    /// Water conservancy, environmental, or public-facility management.
    ConservancyEnvironmentAndPublicFacilities,
    /// Residential or other personal services.
    ResidentialServices,
    /// Education services as the person's primary employment sector.
    Education,
    /// Health care or social work.
    HealthAndSocialWork,
    /// Culture, sports, or entertainment.
    CultureSportsAndEntertainment,
    /// Public administration, social security, or social organizations.
    PublicAdminSocialSecurityAndSocialOrganizations,
    /// International organizations.
    InternationalOrganizations,
}
