// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Person, user, and demographic domain models.

mod enums;
#[allow(clippy::module_inception)]
mod person;
mod person_info;
mod user;
mod user_info;

pub use enums::{
    Blood, Education, Ethnic, Gender, Incoming, Industry, JobTitle, Marriage, Politics, Religion,
    SexOrientation, SocialNetwork,
};
pub use person::{Person, PersonIdentity};
pub use person_info::PersonInfo;
pub use user::User;
pub use user_info::UserInfo;
