// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! XML adapter for telephone-number text values.

use crate::contact::ContactCodecError;
use crate::contact::Phone;
use crate::contact::PhoneCodec;

/// Marshals and unmarshals optional telephone numbers as XML text.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhoneXmlAdapter;

impl PhoneXmlAdapter {
    /// Decodes an optional XML text value.
    ///
    /// Returns [`ContactCodecError::InvalidPhone`] when present text has an unsupported phone
    /// form.
    pub fn unmarshal(value: Option<&str>) -> Result<Option<Phone>, ContactCodecError> {
        PhoneCodec::decode(value)
    }

    /// Encodes an optional telephone number as XML text.
    #[must_use]
    pub fn marshal(value: Option<&Phone>) -> Option<String> {
        PhoneCodec::encode(value)
    }
}
