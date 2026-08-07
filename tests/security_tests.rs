// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::commons::State;
use qubit_model::security::KeyFormat;
use qubit_model::security::KeyPair;
use qubit_model::security::KeyValuePair;
use qubit_model::security::Signature;
use qubit_model::security::SignatureAlgorithm;
use qubit_model::security::SignedData;
use qubit_model::security::SignedInfo;
use qubit_model_metadata::metadata_of;
use qubit_redact::Redact;

fn assert_redact<T: Redact>() {}

#[test]
fn security_public_models_preserve_source_shapes_and_traits() {
    assert_redact::<KeyPair>();
    assert_redact::<Signature>();
    assert_redact::<SignedInfo>();
    assert_redact::<SignedData>();
    assert_redact::<KeyValuePair>();

    assert_eq!(metadata_of::<KeyPair>().struct_fields().len(), 13);
    assert_eq!(metadata_of::<Signature>().struct_fields().len(), 8);
    assert_eq!(metadata_of::<SignedInfo>().struct_fields().len(), 9);
    assert_eq!(metadata_of::<SignedData>().struct_fields().len(), 2);
    assert_eq!(metadata_of::<KeyValuePair>().struct_fields().len(), 2);
    assert_eq!(
        metadata_of::<KeyPair>().primary_key().unwrap().fields()[0].name(),
        "id"
    );
}

#[test]
fn security_enums_preserve_wire_values_and_codes() {
    assert_eq!(
        serde_json::to_string(&SignatureAlgorithm::Sha256WithRsa).unwrap(),
        "\"SHA256_WITH_RSA\""
    );
    assert_eq!(SignatureAlgorithm::Sha256WithRsa.code(), "SHA256withRSA");
    assert_eq!(KeyFormat::Pkcs8.code(), "PKCS#8");
    assert_eq!(KeyFormat::for_name("x.509"), Some(KeyFormat::X509));
}

#[test]
fn signature_payload_and_key_matching_preserve_source_behavior() {
    let mut signature = Signature::default();
    signature.set_message("approve");
    signature.set_payload("tenant", "qubit");
    signature.signer_type = "USER".into();
    signature.signer_id = 7;
    signature.signed_info.algorithm = SignatureAlgorithm::Sha256WithRsa;
    signature.signed_info.key_version = "v1".into();
    signature.signed_info.public_key = "public-material".into();

    assert_eq!(signature.signed_info.message, "approve");
    assert_eq!(signature.signed_info.payload[0].key, "tenant");
    assert_eq!(
        signature.signed_info.payload[0].value.as_deref(),
        Some("qubit")
    );

    let key_pair = KeyPair {
        owner_type: "USER".into(),
        owner_id: 7,
        algorithm: SignatureAlgorithm::Sha256WithRsa,
        format: KeyFormat::X509,
        version: "v1".into(),
        public_key: "public-material".into(),
        state: State::Normal,
        ..KeyPair::default()
    };
    assert!(key_pair.matches_signature(&signature));
}

#[test]
fn security_redaction_hides_private_and_signed_material() {
    let key_pair = KeyPair {
        public_key: "public-material".into(),
        private_key: Some("private-material".into()),
        ..KeyPair::default()
    };
    let rendered = format!("{:?}", key_pair.redacted());
    assert!(!rendered.contains("private-material"));

    let signature = Signature {
        signed_value: "raw-signature".into(),
        signed_info: SignedInfo {
            credential_number: "private-credential".into(),
            ..SignedInfo::default()
        },
        ..Signature::default()
    };
    let rendered = format!("{:?}", signature.redacted());
    assert!(!rendered.contains("raw-signature"));
    assert!(!rendered.contains("private-credential"));
}
