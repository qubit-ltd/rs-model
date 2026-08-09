// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Roles, user assignments, and serialized permission collections.

mod privileges;
mod privileges_codec;
mod privileges_codec_error;
mod role;
mod user_role;

pub use privileges::Privileges;
pub use privileges_codec::PrivilegesCodec;
pub use privileges_codec_error::PrivilegesCodecError;
pub use role::Role;
pub use user_role::UserRole;
