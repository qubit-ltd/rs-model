// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Registration of telephone-number wire adapters.

use crate::contact::Phone;
use crate::contact::PhoneJsonDeserializer;
use crate::contact::PhoneJsonKeyDeserializer;
use crate::contact::PhoneJsonSerializer;

/// Exposes the Java-compatible JSON adapters registered for [`Phone`] values and map keys.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhoneTypeRegister;

impl PhoneTypeRegister {
    /// Returns the fully qualified Rust type name registered by this adapter bundle.
    #[must_use]
    pub fn type_name(self) -> &'static str {
        core::any::type_name::<Phone>()
    }

    /// Returns the adapter that encodes a phone used as a JSON value.
    #[must_use]
    pub const fn serializer(self) -> PhoneJsonSerializer {
        PhoneJsonSerializer
    }

    /// Returns the adapter that decodes a phone used as a JSON value.
    #[must_use]
    pub const fn deserializer(self) -> PhoneJsonDeserializer {
        PhoneJsonDeserializer
    }

    /// Returns the adapter that encodes a phone used as a JSON object key.
    #[must_use]
    pub const fn key_serializer(self) -> PhoneJsonSerializer {
        PhoneJsonSerializer
    }

    /// Returns the adapter that decodes a phone used as a JSON object key.
    #[must_use]
    pub const fn key_deserializer(self) -> PhoneJsonKeyDeserializer {
        PhoneJsonKeyDeserializer
    }
}
