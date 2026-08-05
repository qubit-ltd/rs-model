// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Runtime environment, sessions, hosts, and operation-log models.

mod action;
mod environment;
mod error_info;
mod expired;
mod expired_reason;
mod host;
mod log;
mod logic_relation;
mod operation_log;
mod operation_log_info;
mod platform;
mod session;

pub use action::Action;
pub use environment::Environment;
pub use error_info::ErrorInfo;
pub use expired::Expired;
pub use expired_reason::ExpiredReason;
pub use host::Host;
pub use log::Log;
pub use logic_relation::LogicRelation;
pub use operation_log::OperationLog;
pub use operation_log_info::OperationLogInfo;
pub use platform::Platform;
pub use session::Session;

pub use crate::{
    notification::{VerifyCode, VerifyScene},
    setting::Setting,
};
