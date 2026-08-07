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

/// Describes the serializer and deserializer components used for [`Phone`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhoneTypeRegister;

impl PhoneTypeRegister {
    /// Returns the registered Rust type name.
    #[must_use]
    pub fn type_name(self) -> &'static str {
        core::any::type_name::<Phone>()
    }

    /// Returns the registered value serializer.
    #[must_use]
    pub const fn serializer(self) -> PhoneJsonSerializer {
        PhoneJsonSerializer
    }

    /// Returns the registered value deserializer.
    #[must_use]
    pub const fn deserializer(self) -> PhoneJsonDeserializer {
        PhoneJsonDeserializer
    }

    /// Returns the registered key serializer.
    #[must_use]
    pub const fn key_serializer(self) -> PhoneJsonSerializer {
        PhoneJsonSerializer
    }

    /// Returns the registered key deserializer.
    #[must_use]
    pub const fn key_deserializer(self) -> PhoneJsonKeyDeserializer {
        PhoneJsonKeyDeserializer
    }
}
