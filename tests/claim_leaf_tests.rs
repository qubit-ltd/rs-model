// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Integration coverage for medical, service, invoice, and claim
//! classifications.

use qubit_model::{
    claim::{
        AccidentReason,
        InsuranceClaimInvoiceStatus,
        InsuranceClaimInvoiceType,
        InsuranceClaimStatus,
        InsuranceClaimStatusGroup,
        InsuredStatus,
        QuickCompensationState,
        enterprise::{
            EnterpriseClaimItemStatus,
            EnterpriseClaimStatus,
            EnterpriseClaimStatusGroup,
            EnterpriseInsuredType,
            EnterpriseOwnership,
            SaveStatus,
        },
    },
    invoice::InvoiceStatus,
    medical::{
        MedicalInvoiceType,
        MedicalType,
    },
    service::UserServiceState,
};

/// Verifies classifications preserve every Java wire variant added to the
/// graph.
#[test]
fn test_leaf_enums_preserve_java_wire_values() {
    let values = [
        serde_json::to_string(&MedicalType::SpecificClinic)
            .expect("medical type should serialize"),
        serde_json::to_string(&MedicalInvoiceType::Other)
            .expect("medical invoice type should serialize"),
        serde_json::to_string(&InvoiceStatus::NoInvoice)
            .expect("invoice status should serialize"),
        serde_json::to_string(&InsuranceClaimInvoiceStatus::IgnoredRepeat)
            .expect("claim invoice status should serialize"),
        serde_json::to_string(&InsuranceClaimInvoiceType::Hospitalization)
            .expect("claim invoice type should serialize"),
        serde_json::to_string(&AccidentReason::Other)
            .expect("accident reason should serialize"),
        serde_json::to_string(&InsuredStatus::Other)
            .expect("insured status should serialize"),
        serde_json::to_string(&QuickCompensationState::Failed)
            .expect("quick compensation state should serialize"),
        serde_json::to_string(&UserServiceState::AppointmentSuccess)
            .expect("user service state should serialize"),
        serde_json::to_string(&EnterpriseClaimItemStatus::Completed)
            .expect("enterprise item status should serialize"),
        serde_json::to_string(&SaveStatus::NotSaved)
            .expect("save status should serialize"),
    ];

    assert_eq!(
        values,
        [
            "\"SPECIFIC_CLINIC\"",
            "\"OTHER\"",
            "\"NO_INVOICE\"",
            "\"IGNORED_REPEAT\"",
            "\"HOSPITALIZATION\"",
            "\"OTHER\"",
            "\"OTHER\"",
            "\"FAILED\"",
            "\"APPOINTMENT_SUCCESS\"",
            "\"COMPLETED\"",
            "\"NOT_SAVED\"",
        ]
    );
}

/// Verifies claim states preserve their Java status-group relationships.
#[test]
fn test_claim_statuses_preserve_source_groups() {
    let individual_status_group: Box<
        dyn Fn(InsuranceClaimStatus) -> InsuranceClaimStatusGroup,
    > = Box::new(InsuranceClaimStatus::status_group);
    let enterprise_status_group: Box<
        dyn Fn(EnterpriseClaimStatus) -> EnterpriseClaimStatusGroup,
    > = Box::new(EnterpriseClaimStatus::status_group);
    assert_eq!(
        individual_status_group(
            InsuranceClaimStatus::ClaimApplicationWaitAudit
        ),
        InsuranceClaimStatusGroup::PendingCase
    );
    assert_eq!(
        individual_status_group(
            InsuranceClaimStatus::InsuranceCompanyCompleted
        ),
        InsuranceClaimStatusGroup::Completed
    );
    assert!(
        InsuranceClaimStatus::list_not_finished_status()
            .contains(&InsuranceClaimStatus::SystemRejected)
    );
    assert!(
        !InsuranceClaimStatus::list_not_finished_status()
            .contains(&InsuranceClaimStatus::InsuranceCompanyCompleted)
    );

    assert_eq!(
        enterprise_status_group(
            EnterpriseClaimStatus::ClaimApplicationWaitAudit
        ),
        EnterpriseClaimStatusGroup::Register
    );
    assert_eq!(
        enterprise_status_group(
            EnterpriseClaimStatus::InsuranceCompanyCompleted
        ),
        EnterpriseClaimStatusGroup::Complete
    );
    assert!(
        EnterpriseClaimStatus::list_not_finished_status()
            .contains(&EnterpriseClaimStatus::TemporarySaved)
    );
}

/// Verifies enterprise code-bearing classifications retain source codes.
#[test]
fn test_enterprise_classifications_preserve_source_codes() {
    let insured_type_code: Box<dyn Fn(EnterpriseInsuredType) -> &'static str> =
        Box::new(EnterpriseInsuredType::code);
    let insured_type_description: Box<
        dyn Fn(EnterpriseInsuredType) -> &'static str,
    > = Box::new(EnterpriseInsuredType::description);
    let insured_types = [
        (EnterpriseInsuredType::InService, "10", "在职"),
        (EnterpriseInsuredType::Retired, "11", "退休"),
        (EnterpriseInsuredType::Resigned, "12", "退职"),
        (EnterpriseInsuredType::OverSeventy, "13", "70岁以上"),
        (EnterpriseInsuredType::OnlyChild, "31", "独生子女<=16"),
        (EnterpriseInsuredType::ChildDonorGenus, "32", "子女供属"),
        (EnterpriseInsuredType::DonorGenus, "41", "供属"),
    ];
    for (insured_type, code, description) in insured_types {
        assert_eq!(insured_type_code(insured_type), code);
        assert_eq!(insured_type_description(insured_type), description);
    }
    let code = std::hint::black_box(
        EnterpriseOwnership::code as fn(EnterpriseOwnership) -> &'static str,
    );
    let description = std::hint::black_box(
        EnterpriseOwnership::description
            as fn(EnterpriseOwnership) -> &'static str,
    );
    let ownerships = [
        (EnterpriseOwnership::Yangtze, "1", "扬子"),
        (EnterpriseOwnership::Reform, "0", "改制"),
        (EnterpriseOwnership::CoSolution, "2", "协解"),
        (EnterpriseOwnership::Test, "z", "测试"),
    ];
    for (ownership, expected_code, expected_description) in ownerships {
        assert_eq!(code(ownership), expected_code);
        assert_eq!(description(ownership), expected_description);
    }
}
