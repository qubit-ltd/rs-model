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
    assert_eq!(
        InsuranceClaimStatus::ClaimApplicationWaitAudit.status_group(),
        InsuranceClaimStatusGroup::PendingCase
    );
    assert_eq!(
        InsuranceClaimStatus::InsuranceCompanyCompleted.status_group(),
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
        EnterpriseClaimStatus::ClaimApplicationWaitAudit.status_group(),
        EnterpriseClaimStatusGroup::Register
    );
    assert_eq!(
        EnterpriseClaimStatus::InsuranceCompanyCompleted.status_group(),
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
    assert_eq!(EnterpriseInsuredType::OverSeventy.code(), "13");
    assert_eq!(
        EnterpriseInsuredType::OnlyChild.description(),
        "独生子女<=16"
    );
    assert_eq!(EnterpriseOwnership::Yangtze.code(), "1");
    assert_eq!(EnterpriseOwnership::Test.description(), "测试");
}
