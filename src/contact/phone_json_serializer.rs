// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! JSON serializer for telephone-number wire values.

use crate::contact::{Phone, PhoneCodec};

/// Serializes a telephone number as a JSON string instead of an object.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhoneJsonSerializer;

impl PhoneJsonSerializer {
    /// Serializes an optional telephone number as a JSON string or null.
    pub fn serialize(phone: Option<&Phone>) -> Result<String, serde_json::Error> {
        serde_json::to_string(&PhoneCodec::encode(phone))
    }
}
