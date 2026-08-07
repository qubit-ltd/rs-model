// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Setting values and stable setting names.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// The value type declared by a setting.
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
pub enum DataType {
    /// Boolean value.
    Bool,
    /// Unicode scalar value.
    Char,
    /// Signed byte value.
    Byte,
    /// Signed 16-bit integer value.
    Short,
    /// Signed 32-bit integer value.
    Int,
    /// Signed 64-bit integer value.
    Long,
    /// 32-bit floating-point value.
    Float,
    /// 64-bit floating-point value.
    Double,
    /// UTF-8 string value.
    #[default]
    String,
    /// Calendar date.
    Date,
    /// Local clock time.
    Time,
    /// Local date and time.
    Datetime,
    /// UTC instant.
    Instant,
    /// Timestamp value.
    Timestamp,
    /// Byte array.
    ByteArray,
    /// Class or type name.
    Class,
    /// Arbitrary-precision integer.
    BigInteger,
    /// Arbitrary-precision decimal.
    BigDecimal,
    /// String array.
    StringArray,
    /// Enumeration value.
    Enum,
    /// Enumeration array.
    EnumArray,
}
