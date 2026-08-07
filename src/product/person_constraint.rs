// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Eligibility constraints applied to a person.

use chrono::TimeDelta;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::person::Gender;

/// Optional eligibility restrictions for a buyer or client.
#[derive(Clone, Debug, Default, Deserialize, Model, PartialEq, Serialize)]
pub struct PersonConstraint {
    /// Inclusive minimum age, expressed as a calendar-like elapsed duration.
    #[model(opaque)]
    pub min_age: Option<TimeDelta>,

    /// Inclusive maximum age, expressed as a calendar-like elapsed duration.
    #[model(opaque)]
    pub max_age: Option<TimeDelta>,

    /// Whether the person must be an adult.
    pub adult_only: Option<bool>,

    /// Required gender.
    pub gender: Option<Gender>,

    /// Required Medicare ownership state.
    pub has_medicare: Option<bool>,

    /// Required social-security ownership state.
    pub has_social_security: Option<bool>,

    /// Required combined Medicare-or-social-security state.
    pub has_medicare_or_social_security: Option<bool>,

    /// Allowed Medicare city codes; `None` means unrestricted.
    #[model(sequence(min_items = 1, max_items = 16))]
    pub medicare_cities: Option<Vec<String>>,

    /// Allowed social-security city codes; `None` means unrestricted.
    #[model(sequence(min_items = 1, max_items = 16))]
    pub social_security_cities: Option<Vec<String>>,

    /// Whether a minor must provide guardian information.
    pub need_guardian: Option<bool>,
}
