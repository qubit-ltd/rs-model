// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Address-domain models, builders, mixins, and errors.

mod address_builder;
mod address_error_code;
mod mismatch_mobile_exception;

pub use address_builder::AddressBuilder;
pub use address_error_code::AddressErrorCode;
pub use mismatch_mobile_exception::MismatchMobileException;

pub use crate::contact::Address;
pub use crate::contact::City;
pub use crate::contact::Contact;
pub use crate::contact::Country;
pub use crate::contact::District;
pub use crate::contact::Province;
pub use crate::contact::Region;
pub use crate::contact::Street;
pub use crate::mixin::WithAddress;
pub use crate::mixin::WithContact;
