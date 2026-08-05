// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::upload::AttachmentType;

#[test]
fn test_attachment_type_for_content_type_matches_source_rules() {
    assert_eq!(
        AttachmentType::for_content_type("image/jpeg"),
        AttachmentType::Image
    );
    assert_eq!(
        AttachmentType::for_content_type("audio/mpeg"),
        AttachmentType::Audio
    );
    assert_eq!(
        AttachmentType::for_content_type("video/mp4"),
        AttachmentType::Video
    );
    assert_eq!(
        AttachmentType::for_content_type("text/x-vcard"),
        AttachmentType::Vcard
    );
    assert_eq!(
        AttachmentType::for_content_type("application/pdf"),
        AttachmentType::Document
    );
}
