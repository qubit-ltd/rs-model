use qubit_model::{
    claim::InsuranceClaimInvoiceStatus, invoice::InvoiceStatus, medical::MedicalType,
};
#[test]
fn test_leaf_enums_preserve_java_wire_values() {
    assert_eq!(
        serde_json::to_string(&MedicalType::SpecificClinic).expect("medical type"),
        "\"SPECIFIC_CLINIC\""
    );
    assert_eq!(
        serde_json::to_string(&InvoiceStatus::NoInvoice).expect("invoice status"),
        "\"NO_INVOICE\""
    );
    assert_eq!(
        serde_json::to_string(&InsuranceClaimInvoiceStatus::IgnoredMedicareProhibited)
            .expect("claim status"),
        "\"IGNORED_MEDICARE_PROHIBITED\""
    );
}
