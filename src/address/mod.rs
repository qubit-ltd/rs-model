// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Address-domain models, builders, mixins, and errors.

mod address_builder;
mod address_error;

pub use crate::contact::{Address, City, Contact, Country, District, Province, Region, Street};
pub use crate::mixin::{WithAddress, WithContact};
pub use address_builder::AddressBuilder;
pub use address_error::{AddressErrorCode, MismatchMobileException};
