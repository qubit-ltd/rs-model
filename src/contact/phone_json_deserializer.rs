// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! JSON deserializer for telephone-number wire values.

use crate::contact::ContactCodecError;
use crate::contact::Phone;
use crate::contact::PhoneCodec;

/// Deserializes a telephone number from a JSON string or null.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhoneJsonDeserializer;

impl PhoneJsonDeserializer {
    /// Deserializes an optional telephone number from JSON.
    pub fn deserialize(value: &str) -> Result<Option<Phone>, ContactCodecError> {
        let encoded: Option<String> =
            serde_json::from_str(value).map_err(ContactCodecError::InvalidJson)?;
        PhoneCodec::decode(encoded.as_deref())
    }
}
