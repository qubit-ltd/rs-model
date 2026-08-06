// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use chrono::Utc;
use qubit_mixin::{
    Emptyful,
    InfoWithEntity,
};
use qubit_model::metadata::AggregateRef;
use qubit_model::{
    commons::State,
    file::{
        Attachment,
        AttachmentType,
        FileInfo,
        MediaInfo,
        MediaType,
        Upload,
        UploadParams,
    },
};
use qubit_model_metadata::{
    UniqueComparison,
    metadata_of,
};
use qubit_redact::Redact;
use serde::Serialize;
use std::io;

fn assert_redact<T: Redact>() {}

/// A deterministic writer that fails on the selected write operation.
struct FailingWriter {
    failure_at: usize,
    writes: usize,
}

impl FailingWriter {
    /// Creates a writer that fails exactly at `failure_at`.
    const fn new(failure_at: usize) -> Self {
        Self {
            failure_at,
            writes: 0,
        }
    }
}

impl io::Write for FailingWriter {
    fn write(&mut self, buffer: &[u8]) -> io::Result<usize> {
        self.writes += 1;
        if self.writes == self.failure_at {
            Err(io::Error::other("intentional test writer failure"))
        } else {
            Ok(buffer.len())
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

/// Verifies that every serializer write boundary propagates its I/O error.
fn assert_serializer_propagates_each_write_error<T: Serialize>(value: &T) {
    for failure_at in 1..=2_048 {
        let mut serializer =
            serde_json::Serializer::new(FailingWriter::new(failure_at));
        if value.serialize(&mut serializer).is_ok() {
            return;
        }
    }
    panic!("serializer did not complete within the expected write boundary");
}

/// Serializes a value through the public JSON text representation.
fn json_value<T: Serialize>(value: &T) -> serde_json::Value {
    let text = serde_json::to_string(value)
        .expect("value should serialize to JSON text");
    serde_json::from_str(&text).expect("JSON text should parse into a value")
}

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
    assert_eq!(
        file.unique_constraints()
            .next()
            .unwrap()
            .comparison_of("path"),
        Some(UniqueComparison::IgnoreCase)
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

    let serialized = json_value(&attachment);
    assert_eq!(serialized["file_path"], "/private/original.jpg");
    assert_eq!(serialized["screenshot_path"], "/private/screenshot.jpg");
    assert_eq!(serialized["small_thumbnail_path"], "/private/small.jpg");
    assert_eq!(serialized["large_thumbnail_path"], "/private/large.jpg");

    let empty = Attachment::default();
    assert_eq!(empty.file_path(), Some(""));
    assert_eq!(json_value(&empty)["file_path"], "");
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
    let screenshot = upload.set_screenshot_info().path.clone();
    let small = upload.set_small_thumbnail_info().path.clone();
    let large = upload.set_large_thumbnail_info().path.clone();
    assert!(screenshot.contains("image_screenshot-"));
    assert!(small.contains("image_thumbnail_small-"));
    assert!(large.contains("image_thumbnail_large-"));
    assert!(screenshot.ends_with(".jpg"));
    assert!(small.ends_with(".jpg"));
    assert!(large.ends_with(".jpg"));
    assert_ne!(
        screenshot,
        upload.set_screenshot_info().path,
        "generated rendition paths must not collide"
    );
    assert_eq!(Upload::IMAGE_EXTENSION, ".jpg");
    assert_eq!(Upload::IMAGE_FORMAT, "jpeg");
}

#[test]
fn file_emptiness_observes_every_source_field() {
    assert!(FileInfo::default().is_empty());
    assert!(
        !FileInfo {
            format: "jpeg".into(),
            ..FileInfo::default()
        }
        .is_empty()
    );
    assert!(Upload::default().is_empty());
    assert!(
        !Upload {
            hash_algorithm: Some("SHA-256".into()),
            ..Upload::default()
        }
        .is_empty()
    );
}

#[test]
fn upload_create_populates_file_and_verification_metadata() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let params = UploadParams {
        filename: None,
        content_type: Some("application/toml".into()),
        delete_origin: false,
        algorithm: Some("SHA-256".into()),
        hash: Some("expected-hash".into()),
    };

    let upload = Upload::create(&path, &params).unwrap();

    assert_eq!(upload.original_filename, None);
    assert_eq!(upload.file.path, path.to_string_lossy());
    assert!(upload.file.size > 0);
    assert_eq!(upload.file.content_type, "application/toml");
    assert_eq!(upload.r#type, AttachmentType::Document);
    assert_eq!(upload.hash_algorithm.as_deref(), Some("SHA-256"));
    assert_eq!(upload.hash_value.as_deref(), Some("expected-hash"));

    let serialized = json_value(&upload);
    assert!(serialized.get("original_filename").is_none());
    assert!(serialized.get("originalFilename").is_none());
    assert!(serialized.get("id").is_none());
    assert_eq!(serialized["hash_algorithm"], "SHA-256");
}

#[test]
fn upload_create_rejects_the_source_invalid_missing_content_type() {
    let path =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let error = Upload::create(&path, &UploadParams::default()).unwrap_err();

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
}

#[test]
fn upload_file_info_supports_relative_paths_and_missing_files() {
    let mut upload = Upload::default();
    let file = upload
        .set_file_info(
            std::path::Path::new("missing-upload-file"),
            "text/plain",
        )
        .expect("relative paths are resolved against the working directory");
    assert!(file.path.ends_with("missing-upload-file"));
    assert_eq!(file.size, 0);
    assert_eq!(file.content_type, "text/plain");
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

#[test]
fn attachment_serialization_preserves_every_present_optional_property() {
    let now = Utc::now();
    let attachment = Attachment {
        id: Some(7),
        aggregate_ref: Some(AggregateRef::default()),
        category: Some(InfoWithEntity::default()),
        index: 2,
        title: Some("private title".into()),
        description: Some("description".into()),
        upload: Upload {
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
        },
        create_time: Some(now),
        modify_time: Some(now),
        delete_time: Some(now),
        ..Attachment::default()
    };

    let serialized = json_value(&attachment);
    for field in [
        "id",
        "aggregate_ref",
        "category",
        "title",
        "description",
        "create_time",
        "modify_time",
        "delete_time",
        "screenshot_path",
        "small_thumbnail_path",
        "large_thumbnail_path",
    ] {
        assert!(
            serialized.get(field).is_some(),
            "missing serialized {field}"
        );
    }
}

#[test]
fn attachment_serialization_propagates_every_writer_failure() {
    let attachment = Attachment {
        id: Some(7),
        aggregate_ref: Some(AggregateRef::default()),
        category: Some(InfoWithEntity::default()),
        title: Some("private title".into()),
        description: Some("description".into()),
        upload: Upload {
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
        },
        create_time: Some(Utc::now()),
        modify_time: Some(Utc::now()),
        delete_time: Some(Utc::now()),
        ..Attachment::default()
    };

    assert_serializer_propagates_each_write_error(&attachment);
}

#[test]
fn upload_rendition_uses_a_generated_basename_when_the_source_path_is_empty() {
    let mut upload = Upload::default();

    let screenshot = upload.set_screenshot_info();

    assert!(screenshot.path.contains(Upload::SCREENSHOT_SUFFIX));
    assert!(screenshot.path.ends_with(Upload::IMAGE_EXTENSION));
}

#[test]
fn file_emptyful_trait_and_attachment_deserialization_preserve_defaults() {
    let file = FileInfo::default();
    assert!(<FileInfo as Emptyful>::is_empty(&file));
    let upload = Upload::default();
    assert!(<Upload as Emptyful>::is_empty(&upload));

    let attachment: Attachment = serde_json::from_value(serde_json::json!({
        "type": "DOCUMENT",
        "upload": {},
        "state": "NORMAL"
    }))
    .expect("attachment defaults must deserialize from source JSON");
    assert!(attachment.visible);
}

#[test]
fn upload_file_info_resolves_relative_paths_and_preserves_missing_sizes() {
    let mut upload = Upload::default();
    let relative = std::path::Path::new("Cargo.toml");
    let file = upload
        .set_file_info(relative, "application/toml")
        .expect("relative project paths must resolve");
    assert!(std::path::Path::new(&file.path).is_absolute());
    assert!(file.size > 0);

    let missing = std::path::Path::new("missing-upload-coverage-file");
    let file = upload
        .set_file_info(missing, "application/octet-stream")
        .expect("missing source files retain the source zero-size behavior");
    assert_eq!(file.size, 0);
}
