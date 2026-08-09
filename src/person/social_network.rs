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

/// Third-party social platform used to authenticate or identify a user.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SocialNetwork {
    /// WeChat.
    Wechat,
    /// Sina Weibo.
    Sina,
    /// Zhihu.
    Zhihu,
    /// Douyin.
    Douyin,
    /// Bilibili.
    Bilibili,
}
