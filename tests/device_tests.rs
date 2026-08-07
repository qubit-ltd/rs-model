// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_model::device::DataNetworkType;
use qubit_model::device::DeviceType;
use qubit_model::device::SimCardStatus;
use qubit_model::system::Platform;

#[test]
fn test_data_network_type_reports_source_generation() {
    assert_eq!(DataNetworkType::Gprs.generation(), 2);
    assert_eq!(DataNetworkType::Hspa.generation(), 3);
    assert_eq!(DataNetworkType::Lte.generation(), 4);
    assert_eq!(DataNetworkType::Nr.generation(), 5);
    assert_eq!(DataNetworkType::Unknown.generation(), 0);
}

#[test]
fn test_device_enums_preserve_java_wire_values() {
    assert_eq!(
        serde_json::to_string(&DeviceType::IotBox).expect("device type serializes"),
        "\"IOT_BOX\""
    );
    assert_eq!(
        serde_json::to_string(&SimCardStatus::CardIoError).expect("SIM status serializes"),
        "\"CARD_IO_ERROR\""
    );
    assert_eq!(
        serde_json::to_string(&Platform::IpadOs).expect("platform serializes"),
        "\"IPAD_OS\""
    );
}
