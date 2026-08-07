// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! JSON object-key deserializer for telephone numbers.

use crate::contact::ContactCodecError;
use crate::contact::Phone;
use crate::contact::PhoneCodec;

/// Deserializes a telephone number used as a JSON object key.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhoneJsonKeyDeserializer;

impl PhoneJsonKeyDeserializer {
    /// Decodes a non-null JSON object key.
    pub fn deserialize_key(value: &str) -> Result<Phone, ContactCodecError> {
        PhoneCodec::decode(Some(value))?.ok_or(ContactCodecError::InvalidPhone)
    }
}
