// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Client operating-system platforms.

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{
    Deserialize,
    Serialize,
};

/// Client operating-system platform.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Deserialize,
    Eq,
    Model,
    PartialEq,
    Redact,
    Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Platform {
    /// Apple iOS.
    Ios,
    /// Apple iPadOS.
    IpadOs,
    /// Google Android.
    Android,
    /// Windows Phone.
    WindowsPhone,
    /// Windows desktop.
    Windows,
    /// Linux desktop.
    Linux,
    /// macOS desktop.
    Mac,
    /// Web browser.
    Web,
    /// Unknown platform.
    #[default]
    Unknown,
}
