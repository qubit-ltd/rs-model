// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Utility values migrated from the Java model utility package.

mod message_formatter;
mod result_value;

pub use message_formatter::MessageFormatter;
pub use result_value::{
    Result,
    ResultValue,
};
