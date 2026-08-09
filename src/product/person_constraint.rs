// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Optional eligibility requirements evaluated against a buyer or client.

use chrono::TimeDelta;
use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;

use crate::person::Gender;

/// A set of independently optional requirements for a person in a purchase.
#[derive(Model, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PersonConstraint {
    /// Inclusive lower age bound; `None` leaves age unrestricted on this side.
    #[model(opaque)]
    pub min_age: Option<TimeDelta>,

    /// Inclusive upper age bound; `None` leaves age unrestricted on this side.
    #[model(opaque)]
    pub max_age: Option<TimeDelta>,

    /// Required adult status, or `None` when adulthood is irrelevant.
    pub adult_only: Option<bool>,

    /// Required gender, or `None` when no gender restriction applies.
    pub gender: Option<Gender>,

    /// Required Medicare enrollment state, or `None` when unrestricted.
    pub has_medicare: Option<bool>,

    /// Required social-security enrollment state, or `None` when unrestricted.
    pub has_social_security: Option<bool>,

    /// Required combined Medicare-or-social-security state, or `None` when unrestricted.
    pub has_medicare_or_social_security: Option<bool>,

    /// Allowed Medicare city codes; `None` means unrestricted.
    #[model(sequence(min_items = 1, max_items = 16))]
    pub medicare_cities: Option<Vec<String>>,

    /// Allowed social-security city codes; `None` means unrestricted.
    #[model(sequence(min_items = 1, max_items = 16))]
    pub social_security_cities: Option<Vec<String>>,

    /// Whether minors must provide guardian information, or `None` when not required.
    pub need_guardian: Option<bool>,
}
