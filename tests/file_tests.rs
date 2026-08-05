// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
// =============================================================================

use qubit_model::{
    commons::State,
    file::{Attachment, AttachmentType, FileInfo, MediaInfo, MediaType, Upload, UploadParams},
};
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn file_public_types_preserve_source_fields_and_traits() {
    assert_redact::<Attachment>();
    assert_redact::<AttachmentType>();
    assert_redact::<FileInfo>();
    assert_redact::<MediaInfo>();
    assert_redact::<MediaType>();
    assert_redact::<Upload>();
    assert_redact::<UploadParams>();

    assert_eq!(metadata_of::<Attachment>().struct_fields().len(), 13);
    assert_eq!(metadata_of::<FileInfo>().struct_fields().len(), 8);
    assert_eq!(metadata_of::<MediaInfo>().struct_fields().len(), 4);
    assert_eq!(metadata_of::<Upload>().struct_fields().len(), 11);
    assert_eq!(metadata_of::<UploadParams>().struct_fields().len(), 5);
}

#[test]
fn file_model_constraints_preserve_source_annotations() {
    let attachment = metadata_of::<Attachment>();
    assert_eq!(attachment.primary_key().unwrap().fields()[0].name(), "id");
    assert!(
        attachment
            .indexes()
            .any(|index| index.contains("aggregate_ref"))
    );
    assert!(attachment.field("category").unwrap().reference().is_some());
    assert!(attachment.field("upload").unwrap().reference().is_some());

    let file = metadata_of::<FileInfo>();
    assert!(
        file.unique_constraints()
            .any(|unique| unique.contains("path"))
    );

    let upload = metadata_of::<Upload>();
    for field in ["original_filename", "type", "create_time", "delete_time"] {
        assert!(upload.indexes().any(|index| index.contains(field)));
    }
}

#[test]
fn attachment_types_preserve_wire_values_ids_and_content_type_rules() {
    assert_eq!(
        serde_json::to_string(&AttachmentType::ExternalVideo).unwrap(),
        "\"EXTERNAL_VIDEO\""
    );
    assert_eq!(AttachmentType::ExternalVideo.id(), "external_video");
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
    assert_eq!(
        serde_json::to_string(&MediaType::Record).unwrap(),
        "\"RECORD\""
    );
}

#[test]
fn attachment_create_copies_upload_defaults_and_proxies_paths() {
    let upload = Upload {
        original_filename: Some("portrait.jpg".into()),
        r#type: AttachmentType::Image,
        file: FileInfo {
            path: "/private/original.jpg".into(),
            ..FileInfo::default()
        },
        screenshot: Some(FileInfo {
            path: "/private/screenshot.jpg".into(),
            ..FileInfo::default()
        }),
        small_thumbnail: Some(FileInfo {
            path: "/private/small.jpg".into(),
            ..FileInfo::default()
        }),
        large_thumbnail: Some(FileInfo {
            path: "/private/large.jpg".into(),
            ..FileInfo::default()
        }),
        ..Upload::default()
    };

    let attachment = Attachment::create(upload);

    assert_eq!(attachment.title.as_deref(), Some("portrait.jpg"));
    assert_eq!(attachment.r#type, AttachmentType::Image);
    assert_eq!(attachment.index, 0);
    assert_eq!(attachment.state, State::Normal);
    assert!(attachment.visible);
    assert_eq!(attachment.file_path(), Some("/private/original.jpg"));
    assert_eq!(
        attachment.screenshot_path(),
        Some("/private/screenshot.jpg")
    );
    assert_eq!(
        attachment.small_thumbnail_path(),
        Some("/private/small.jpg")
    );
    assert_eq!(
        attachment.large_thumbnail_path(),
        Some("/private/large.jpg")
    );
}

#[test]
fn file_info_and_upload_preserve_source_helpers() {
    let mut info = FileInfo {
        path: "/tmp/image.png".into(),
        ..FileInfo::default()
    };
    assert_eq!(
        info.to_local_path(),
        std::path::PathBuf::from("/tmp/image.png")
    );
    info.set_image_size(Some((640, 480)));
    assert_eq!((info.width, info.height), (Some(640), Some(480)));
    info.set_image_size(None);
    assert_eq!((info.width, info.height), (None, None));

    let mut upload = Upload {
        file: info,
        ..Upload::default()
    };
    assert_eq!(upload.set_content_type("image/webp"), AttachmentType::Image);
    assert_eq!(upload.file.content_type, "image/webp");
    assert!(
        upload
            .set_screenshot_info()
            .path
            .ends_with("image_screenshot.jpg")
    );
    assert!(
        upload
            .set_small_thumbnail_info()
            .path
            .ends_with("image_small.jpg")
    );
    assert!(
        upload
            .set_large_thumbnail_info()
            .path
            .ends_with("image_large.jpg")
    );
}

#[test]
fn upload_create_populates_file_and_verification_metadata() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let params = UploadParams {
        filename: None,
        content_type: Some("application/toml".into()),
        delete_origin: false,
        algorithm: Some("SHA-256".into()),
        hash: Some("expected-hash".into()),
    };

    let upload = Upload::create(&path, &params).unwrap();

    assert_eq!(upload.original_filename.as_deref(), Some("Cargo.toml"));
    assert_eq!(upload.file.path, path.to_string_lossy());
    assert!(upload.file.size > 0);
    assert_eq!(upload.file.content_type, "application/toml");
    assert_eq!(upload.r#type, AttachmentType::Document);
    assert_eq!(upload.hash_algorithm.as_deref(), Some("SHA-256"));
    assert_eq!(upload.hash_value.as_deref(), Some("expected-hash"));
}

#[test]
fn file_redaction_hides_paths_filenames_and_hashes_recursively() {
    let upload = Upload {
        original_filename: Some("private-name.pdf".into()),
        file: FileInfo {
            path: "/secret/storage/file.pdf".into(),
            ..FileInfo::default()
        },
        hash_value: Some("raw-upload-hash".into()),
        ..Upload::default()
    };
    let attachment = Attachment::create(upload);
    let rendered = format!("{:?}", attachment.redacted());
    assert!(!rendered.contains("private-name.pdf"));
    assert!(!rendered.contains("/secret/storage/file.pdf"));
    assert!(!rendered.contains("raw-upload-hash"));

    let params = UploadParams {
        filename: Some("private-source.pdf".into()),
        hash: Some("raw-expected-hash".into()),
        ..UploadParams::default()
    };
    let rendered = format!("{:?}", params.redacted());
    assert!(!rendered.contains("private-source.pdf"));
    assert!(!rendered.contains("raw-expected-hash"));
}
