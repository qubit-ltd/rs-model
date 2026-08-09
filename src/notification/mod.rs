// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Verification-code records and errors raised by notification delivery.

mod notification_error_code;
mod send_sms_exception;
mod verify_code;
mod verify_scene;

pub use notification_error_code::NotificationErrorCode;
pub use send_sms_exception::SendSmsException;
pub use verify_code::VerifyCode;
pub use verify_scene::VerifyScene;
