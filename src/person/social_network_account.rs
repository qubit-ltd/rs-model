// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! User accounts maintained by external social networks.

use chrono::{
    DateTime,
    Utc,
};
use qubit_mixin::Normalizable;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

use crate::{
    commons::Payload,
    controller::RegisterUserParams,
    security::KeyValuePair,
};

use super::{
    SocialNetwork,
    User,
};

/// A user's account identity within one social-network application.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
#[serde(default)]
#[model(unique(
    name = "social_network_account_identity",
    fields(social_network, app_id, open_id),
    ignore_case(open_id)
))]
pub struct SocialNetworkAccount {
    /// Optional persisted identifier.
    #[model(identifier)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Owning platform username.
    #[model(
        reference(target = User, target_field = username),
        index,
        text(min_chars = 1, max_chars = 64, repertoire = ascii)
    )]
    pub username: String,
    /// Social-network provider.
    #[model(index)]
    pub social_network: SocialNetwork,
    /// Provider-side application identifier.
    #[model(index, text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    pub app_id: String,
    /// Provider-side open identifier.
    #[model(text(min_chars = 1, max_chars = 64, repertoire = ascii))]
    #[redact(level = "secret")]
    pub open_id: String,
    /// Optional provider nickname.
    #[model(text(min_chars = 1, max_chars = 128))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    /// Optional provider avatar URL.
    #[model(text(min_chars = 1, max_chars = 512, repertoire = ascii))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// Optional provider profile properties.
    #[model(sequence(min_items = 1, max_items = 10))]
    #[redact(nested)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<KeyValuePair>>,
    /// Optional extension payloads.
    #[model(
        reference(target = Payload, target_field = id, must_exist = false),
        sequence(min_items = 1, max_items = 10)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payloads: Option<Vec<Payload>>,
    /// UTC creation timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<DateTime<Utc>>,
    /// Optional UTC modification timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<DateTime<Utc>>,
    /// Optional UTC deletion timestamp.
    #[model(index, time(precision = second, normalization = utc))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_time: Option<DateTime<Utc>>,
}

impl Default for SocialNetworkAccount {
    fn default() -> Self {
        Self {
            id: None,
            username: String::new(),
            social_network: SocialNetwork::Wechat,
            app_id: String::new(),
            open_id: String::new(),
            nickname: None,
            avatar: None,
            profiles: None,
            payloads: None,
            create_time: None,
            modify_time: None,
            delete_time: None,
        }
    }
}

impl SocialNetworkAccount {
    /// Creates an account from the social identity in registration parameters.
    #[must_use]
    pub fn from_register_params(params: &RegisterUserParams) -> Self {
        Self {
            username: params.username.clone(),
            social_network: params
                .social_network
                .unwrap_or(SocialNetwork::Wechat),
            app_id: params.app_id.clone().unwrap_or_default(),
            open_id: params.open_id.clone().unwrap_or_default(),
            nickname: params.nickname.clone(),
            avatar: params.avatar.clone(),
            ..Self::default()
        }
    }
}

impl Normalizable for SocialNetworkAccount {
    fn normalize(&mut self) {
        self.username.normalize();
        self.app_id.normalize();
        self.open_id.normalize();
        self.nickname.normalize();
        self.avatar.normalize();
    }
}
