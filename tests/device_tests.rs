use qubit_model::{
    device::{DataNetworkType, DeviceType, SimCardStatus},
    system::Platform,
};

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
