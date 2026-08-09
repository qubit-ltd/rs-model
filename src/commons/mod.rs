// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Cross-domain application, credential, scheduling, and lifecycle value models.

mod app;
mod app_resource;
mod authorize_record;
mod category;
mod code;
mod code_map;
mod credential;
mod credential_info;
mod credential_info_codec;
mod credential_info_codec_error;
mod credential_type;
mod currency;
mod day_type;
mod dict_entry;
mod dict_entry_info;
mod faq;
mod kinship;
mod mq_failed_task;
mod mq_type;
mod owner;
mod owners;
mod payload;
mod request_status;
mod schedule;
mod source;
mod state;
mod token;
mod verify_state;

pub use app::App;
pub use app_resource::AppResource;
pub use authorize_record::AuthorizeRecord;
pub use category::Category;
pub use code::Code;
pub use code_map::CodeMap;
pub use credential::Credential;
pub use credential_info::CredentialInfo;
pub use credential_info_codec::CredentialInfoCodec;
pub use credential_info_codec_error::CredentialInfoCodecError;
pub use credential_type::CredentialType;
pub use currency::Currency;
pub use day_type::DayType;
pub use dict_entry::DictEntry;
pub use dict_entry_info::DictEntryInfo;
pub use faq::Faq;
pub use kinship::Kinship;
pub use mq_failed_task::MqFailedTask;
pub use mq_type::MqType;
pub use owner::Owner;
pub use owners::Owners;
pub use payload::Payload;
pub use request_status::RequestStatus;
pub use schedule::Schedule;
pub use source::Source;
pub use state::State;
pub use token::Token;
pub use verify_state::VerifyState;

pub use crate::metadata::Dict;
pub use crate::metadata::FullDict;
