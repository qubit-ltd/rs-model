// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! File uploads, media information, and persisted attachments.

mod attachment;
mod attachment_type;
mod file_info;
mod media_info;
mod media_type;
#[allow(clippy::module_inception)]
mod upload;
mod upload_params;

pub use attachment::Attachment;
pub use attachment_type::AttachmentType;
pub use file_info::FileInfo;
pub use media_info::MediaInfo;
pub use media_type::MediaType;
pub use upload::Upload;
pub use upload_params::UploadParams;
