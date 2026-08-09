// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Lightweight formatting and response-wrapper utilities for model APIs.

mod message_formatter;
mod result_value;

/// Formatter for indexed message templates and message-key prefixes.
pub use message_formatter::MessageFormatter;
/// Java-compatible alias for a single-value REST response wrapper.
pub use result_value::Result;
/// Wrapper that serializes one response value as a JSON object.
pub use result_value::ResultValue;
