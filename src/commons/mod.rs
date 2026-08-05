// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Shared domain values migrated from the Java commons package.

mod app;
mod app_resource;
mod code;
mod code_map;
mod credential_info_codec;
mod credential_info_codec_error;
mod dict_entry;
mod dict_entry_info;
mod enums;
mod faq;
mod mq_failed_task;
mod records;
mod schedule;
mod state;
mod token;

pub use app::App;
pub use app_resource::AppResource;
pub use code::Code;
pub use code_map::CodeMap;
pub use credential_info_codec::CredentialInfoCodec;
pub use credential_info_codec_error::CredentialInfoCodecError;
pub use dict_entry::DictEntry;
pub use dict_entry_info::DictEntryInfo;
pub use enums::{CredentialType, Currency, DayType, Kinship, MqType, RequestStatus, VerifyState};
pub use faq::Faq;
pub use mq_failed_task::MqFailedTask;
pub use records::{
    AuthorizeRecord, Category, Credential, CredentialInfo, Owner, Owners, Payload, Source,
};
pub use schedule::Schedule;
pub use state::State;
pub use token::Token;

pub use crate::metadata::{Dict, FullDict};
