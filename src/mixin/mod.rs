// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared composite values and traits from the Java model mixin package.

mod expirable;
mod has_stateful_info;
mod info_with_app_entity;
mod info_with_token;
mod stateful;
mod stateful_info;
mod stateful_info_with_token;
mod with_address;
mod with_app;
mod with_attachment;
mod with_attachments;
mod with_category;
mod with_contact;
mod with_creator;
mod with_credential;
mod with_deleter;
mod with_location;
mod with_mobile;
mod with_modifier;
mod with_organization;
mod with_owner;
mod with_payloads;
mod with_source;
mod with_stateful_info_with_token;
mod with_token;

pub use expirable::Expirable;
pub use has_stateful_info::HasStatefulInfo;
pub use info_with_app_entity::InfoWithAppEntity;
pub use info_with_token::InfoWithToken;
pub use stateful::Stateful;
pub use stateful_info::StatefulInfo;
pub use stateful_info_with_token::StatefulInfoWithToken;
pub use with_address::WithAddress;
pub use with_app::WithApp;
pub use with_attachment::WithAttachment;
pub use with_attachments::WithAttachments;
pub use with_category::WithCategory;
pub use with_contact::WithContact;
pub use with_creator::WithCreator;
pub use with_credential::WithCredential;
pub use with_deleter::WithDeleter;
pub use with_location::WithLocation;
pub use with_mobile::WithMobile;
pub use with_modifier::WithModifier;
pub use with_organization::WithOrganization;
pub use with_owner::WithOwner;
pub use with_payloads::WithPayloads;
pub use with_source::WithSource;
pub use with_stateful_info_with_token::WithStatefulInfoWithToken;
pub use with_token::WithToken;
