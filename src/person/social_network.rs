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
    Politics,
    Religion,
    SexOrientation,
};

use qubit_model_derive::Model;
use serde::{
    Deserialize,
    Serialize,
};

/// Source-domain SocialNetwork classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SocialNetwork {
    /// Source variant `WECHAT`.
    Wechat,
    /// Source variant `SINA`.
    Sina,
    /// Source variant `ZHIHU`.
    Zhihu,
    /// Source variant `DOUYIN`.
    Douyin,
    /// Source variant `BILIBILI`.
    Bilibili,
}
