// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================
//! Person, user, and demographic domain models.

mod blood;
mod education;
mod ethnic;
mod gender;
mod incoming;
mod industry;
mod job_title;
mod marriage;
#[allow(clippy::module_inception)]
mod person;
mod person_identity;
mod person_info;
mod politics;
mod religion;
mod sex_orientation;
mod social_network;
mod social_network_account;
mod user;
mod user_info;

pub use blood::Blood;
pub use education::Education;
pub use ethnic::Ethnic;
pub use gender::Gender;
pub use incoming::Incoming;
pub use industry::Industry;
pub use job_title::JobTitle;
pub use marriage::Marriage;
pub use person::Person;
pub use person_identity::PersonIdentity;
pub use person_info::PersonInfo;
pub use politics::Politics;
pub use religion::Religion;
pub use sex_orientation::SexOrientation;
pub use social_network::SocialNetwork;
pub use social_network_account::SocialNetworkAccount;
pub use user::User;
pub use user_info::UserInfo;
