// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for polymorphic hospital-information-system records.

use chrono::Utc;

use qubit_mixin::Info;
use qubit_model::medical::ClinicInfo;
use qubit_model::medical::EmergentClinicInfo;
use qubit_model::medical::ExaminationInfo;
use qubit_model::medical::HisInfo;
use qubit_model::medical::HospitalizationInfo;
use qubit_model::medical::RegistrationInfo;
use qubit_model::medical::SpecificClinicInfo;
use qubit_model_metadata::metadata_of;

/// Verifies concrete HIS records retain their shared and specialized fields.
#[test]
fn test_his_structs_expose_all_payload_fields() {
    assert_eq!(metadata_of::<ClinicInfo>().struct_fields().len(), 5);
    assert_eq!(metadata_of::<SpecificClinicInfo>().struct_fields().len(), 6);
    assert_eq!(metadata_of::<EmergentClinicInfo>().struct_fields().len(), 5);
    assert_eq!(metadata_of::<ExaminationInfo>().struct_fields().len(), 4);
    assert_eq!(
        metadata_of::<HospitalizationInfo>().struct_fields().len(),
        11
    );
    assert_eq!(metadata_of::<RegistrationInfo>().struct_fields().len(), 4);
}

/// Verifies the Rust enum emits the Java `type` discriminator.
#[test]
fn test_his_info_serializes_existing_property_discriminator() {
    let info = HisInfo::Clinic(ClinicInfo {
        number: "visit-1".to_owned(),
        remark: None,
        department: Info::new(
            Some(1),
            "internal".to_owned(),
            "Internal Medicine".to_owned(),
            None,
        ),
        record_number: Some("record-1".to_owned()),
        visit_time: Utc::now(),
    });

    let value =
        serde_json::to_value(info).expect("HIS information should serialize");

    assert_eq!(value["type"], "CLINIC");
    assert_eq!(value["number"], "visit-1");
    assert_eq!(value["record_number"], "record-1");

    let decoded: HisInfo = serde_json::from_value(value)
        .expect("tagged HIS information should deserialize");
    assert!(matches!(decoded, HisInfo::Clinic(_)));
}
