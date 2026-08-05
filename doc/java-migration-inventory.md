# Java migration inventory

## Scope and conventions

This inventory covers every public top-level declaration under
`common-mixin/src/main/java` and `common-model/src/main/java`, plus the one
public nested declaration (`SettingXmlAdapter.Adapted`). The Rust paths are
the intended public API paths; an entry marked **planned** does not yet create
that Rust type.

The dependency column records direct imports from the two inventoried Java
source trees. Dependencies on other Java Common modules or third-party
libraries are intentionally excluded because they are outside this migration
scope. A dash means that no such direct import exists.

Statuses:

- **available in qubit-mixin**: a matching public type is already exported by
  `qubit-mixin`; its semantic parity still belongs to the migration task that
  adopts it.
- **planned**: the type has not yet been migrated to `qubit-model`.

## Base dependency boundary

`qubit-model` owns domain types and its `ModelError` boundary. It may depend
on `qubit-mixin` for shared traits, `qubit-model-metadata` and
`qubit-model-derive` for model metadata, `serde`/`serde_json` for wire
formats, `chrono` for Java time equivalents, `bigdecimal` for monetary and
decimal values, `uuid` for identifiers, and `thiserror` for typed errors. It
does not depend on application, persistence, transport, or Java compatibility
crates.

The currently exported foundation is `qubit_model::ModelError`.

## Inventory

| Java FQCN | Target Rust path | Type category | Direct migration dependencies | Migration status |
| --- | --- | --- | --- | --- |
| `ltd.qubit.commons.error.BeanValidationFailedException` | `qubit_model::error::BeanValidationFailedException` | struct (Java class) | - | planned |
| `ltd.qubit.commons.mixin.Auditable` | `qubit_mixin::Auditable` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.Creatable` | `qubit_mixin::Creatable` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.DataWithMaxAge` | `qubit_mixin::DataWithMaxAge` | struct (Java class) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.Deletable` | `qubit_mixin::Deletable` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.Desensitizable` | `qubit_mixin::Desensitizable` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.Emptyful` | `qubit_mixin::Emptyful` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.HasClock` | `qubit_mixin::HasClock` | trait (Java interface) | - | planned |
| `ltd.qubit.commons.mixin.HasInfo` | `qubit_mixin::HasInfo` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.HasInfoWithEntity` | `qubit_mixin::HasInfoWithEntity` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.HasLogger` | `qubit_mixin::HasLogger` | trait (Java interface) | - | planned |
| `ltd.qubit.commons.mixin.HasSpecificInfo` | `qubit_mixin::HasSpecificInfo` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.Identifiable` | `qubit_mixin::Identifiable` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.impl.NormalizeImpl` | `qubit_mixin::normalize_impl::NormalizeImpl` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Normalizable | planned |
| `ltd.qubit.commons.mixin.Info` | `qubit_mixin::Info` | struct (Java class) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.InfoWithEntity` | `qubit_mixin::InfoWithEntity` | struct (Java class) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.Modifiable` | `qubit_mixin::Modifiable` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.Normalizable` | `qubit_mixin::Normalizable` | trait (Java interface) | ltd.qubit.commons.mixin.impl.NormalizeImpl | available in qubit-mixin |
| `ltd.qubit.commons.mixin.Predefinable` | `qubit_mixin::Predefinable` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.Validatable` | `qubit_mixin::Validatable` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithBirthday` | `qubit_mixin::WithBirthday` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithCode` | `qubit_mixin::WithCode` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithComment` | `qubit_mixin::WithComment` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithEmail` | `qubit_mixin::WithEmail` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithEntity` | `qubit_mixin::WithEntity` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithIndex` | `qubit_mixin::WithIndex` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithKey` | `qubit_mixin::WithKey` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithName` | `qubit_mixin::WithName` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithPassword` | `qubit_mixin::WithPassword` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithSecurityKey` | `qubit_mixin::WithSecurityKey` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithStatus` | `qubit_mixin::WithStatus` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithUdid` | `qubit_mixin::WithUdid` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithUsername` | `qubit_mixin::WithUsername` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithUuid` | `qubit_mixin::WithUuid` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.mixin.WithVisibility` | `qubit_mixin::WithVisibility` | trait (Java interface) | - | available in qubit-mixin |
| `ltd.qubit.commons.util.NameBuilder` | `qubit_model::util::NameBuilder` | struct (Java class) | - | planned |
| `ltd.qubit.model.activity.ActivityCoupon` | `qubit_model::activity::ActivityCoupon` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.model.order.OrderInfo, ltd.qubit.model.person.Person | planned |
| `ltd.qubit.model.activity.Activity` | `qubit_model::activity::Activity` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Info, ltd.qubit.model.commons.State, ltd.qubit.model.mixin.HasStatefulInfo | planned |
| `ltd.qubit.model.activity.ActivityProductItem` | `qubit_model::activity::ActivityProductItem` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Deletable, ltd.qubit.model.product.Product, ltd.qubit.model.product.ProductInfo | planned |
| `ltd.qubit.model.ai.AiResult` | `qubit_model::ai::AiResult` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.ai.AiResultType` | `qubit_model::ai::AiResultType` | enum | - | planned |
| `ltd.qubit.model.appointment.Appointment` | `qubit_model::appointment::Appointment` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.audit.AuditStatus, ltd.qubit.model.commons.App, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.person.PersonInfo | planned |
| `ltd.qubit.model.audit.Audit` | `qubit_model::audit::Audit` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.organization.Employee, ltd.qubit.model.organization.EmployeeInfo | planned |
| `ltd.qubit.model.audit.AuditStatus` | `qubit_model::audit::AuditStatus` | enum | - | planned |
| `ltd.qubit.model.china.ChinaCities` | `qubit_model::china::ChinaCities` | struct (Java class) | - | planned |
| `ltd.qubit.model.china.ChinaDistricts` | `qubit_model::china::ChinaDistricts` | struct (Java class) | - | planned |
| `ltd.qubit.model.china.ChinaProvinces` | `qubit_model::china::ChinaProvinces` | struct (Java class) | - | planned |
| `ltd.qubit.model.china.IdentityCardUtils` | `qubit_model::china::IdentityCardUtils` | struct (Java class) | ltd.qubit.model.contact.District, ltd.qubit.model.person.Gender | planned |
| `ltd.qubit.model.claim.AccidentReason` | `qubit_model::claim::AccidentReason` | enum | - | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseClaimEvent` | `qubit_model::claim::enterprise::EnterpriseClaimEvent` | final struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseClaimInvoice` | `qubit_model::claim::enterprise::EnterpriseClaimInvoice` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.model.mixin.WithSource | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseClaimItem` | `qubit_model::claim::enterprise::EnterpriseClaimItem` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.DictEntryInfo | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseClaimItemMedical` | `qubit_model::claim::enterprise::EnterpriseClaimItemMedical` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseClaimItemStatus` | `qubit_model::claim::enterprise::EnterpriseClaimItemStatus` | enum | - | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseClaim` | `qubit_model::claim::enterprise::EnterpriseClaim` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Modifiable, ltd.qubit.model.claim.AccidentReason, ltd.qubit.model.claim.InsuredStatus, ltd.qubit.model.claim.QuickCompensationState, ltd.qubit.model.commons.Kinship, ltd.qubit.model.order.Client, ltd.qubit.model.product.Product, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseClaimMedical` | `qubit_model::claim::enterprise::EnterpriseClaimMedical` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.DictEntryInfo | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseClaimSelfCareItem` | `qubit_model::claim::enterprise::EnterpriseClaimSelfCareItem` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Deletable, ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseClaimStatusGroup` | `qubit_model::claim::enterprise::EnterpriseClaimStatusGroup` | enum | - | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseClaimStatus` | `qubit_model::claim::enterprise::EnterpriseClaimStatus` | enum | - | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseHistoryClaimAmount` | `qubit_model::claim::enterprise::EnterpriseHistoryClaimAmount` | final struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Modifiable, ltd.qubit.model.commons.DictEntryInfo | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseInsuredInfo` | `qubit_model::claim::enterprise::EnterpriseInsuredInfo` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.Modifiable | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseInsuredType` | `qubit_model::claim::enterprise::EnterpriseInsuredType` | enum | - | planned |
| `ltd.qubit.model.claim.enterprise.EnterpriseOwnership` | `qubit_model::claim::enterprise::EnterpriseOwnership` | enum | - | planned |
| `ltd.qubit.model.claim.enterprise.HistoryClaimAmount` | `qubit_model::claim::enterprise::HistoryClaimAmount` | final struct (Java class) | - | planned |
| `ltd.qubit.model.claim.enterprise.SaveStatus` | `qubit_model::claim::enterprise::SaveStatus` | enum | - | planned |
| `ltd.qubit.model.claim.InsuranceClaimAmount` | `qubit_model::claim::InsuranceClaimAmount` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Modifiable | planned |
| `ltd.qubit.model.claim.InsuranceClaimEvent` | `qubit_model::claim::InsuranceClaimEvent` | final struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.claim.InsuranceClaimInvoiceCost` | `qubit_model::claim::InsuranceClaimInvoiceCost` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Modifiable | planned |
| `ltd.qubit.model.claim.InsuranceClaimInvoice` | `qubit_model::claim::InsuranceClaimInvoice` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.claim.InsuranceClaimInvoiceStatus` | `qubit_model::claim::InsuranceClaimInvoiceStatus` | enum | - | planned |
| `ltd.qubit.model.claim.InsuranceClaimInvoiceType` | `qubit_model::claim::InsuranceClaimInvoiceType` | enum | - | planned |
| `ltd.qubit.model.claim.InsuranceClaim` | `qubit_model::claim::InsuranceClaim` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.model.commons.Currency, ltd.qubit.model.commons.Kinship, ltd.qubit.model.contact.Address, ltd.qubit.model.order.Client, ltd.qubit.model.payment.Account, ltd.qubit.model.product.Product, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.claim.InsuranceClaimMedical` | `qubit_model::claim::InsuranceClaimMedical` | final struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.model.medical.MedicalType | planned |
| `ltd.qubit.model.claim.InsuranceClaimStatusGroup` | `qubit_model::claim::InsuranceClaimStatusGroup` | enum | - | planned |
| `ltd.qubit.model.claim.InsuranceClaimStatus` | `qubit_model::claim::InsuranceClaimStatus` | enum | - | planned |
| `ltd.qubit.model.claim.InsuranceProductRule` | `qubit_model::claim::InsuranceProductRule` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info | planned |
| `ltd.qubit.model.claim.InsuredStatus` | `qubit_model::claim::InsuredStatus` | enum | - | planned |
| `ltd.qubit.model.claim.QuickCompensationState` | `qubit_model::claim::QuickCompensationState` | enum | - | planned |
| `ltd.qubit.model.commons.App` | `qubit_model::commons::App` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Desensitizable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable, ltd.qubit.commons.mixin.WithComment, ltd.qubit.commons.mixin.WithSecurityKey, ltd.qubit.model.mixin.HasStatefulInfo, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithCategory, ltd.qubit.model.mixin.WithOrganization, ltd.qubit.model.mixin.WithToken, ltd.qubit.model.organization.Organization, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.commons.AppResource` | `qubit_model::commons::AppResource` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.commons.AuthorizeRecord` | `qubit_model::commons::AuthorizeRecord` | struct (Java class) | - | planned |
| `ltd.qubit.model.commons.Category` | `qubit_model::commons::Category` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasInfoWithEntity, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable | planned |
| `ltd.qubit.model.commons.Code` | `qubit_model::commons::Code` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp | planned |
| `ltd.qubit.model.commons.CodeMap` | `qubit_model::commons::CodeMap` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithEntity | planned |
| `ltd.qubit.model.commons.CredentialInfoCodec` | `qubit_model::commons::CredentialInfoCodec` | struct (Java class) | - | planned |
| `ltd.qubit.model.commons.CredentialInfo` | `qubit_model::commons::CredentialInfo` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.person.Gender | planned |
| `ltd.qubit.model.commons.Credential` | `qubit_model::commons::Credential` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasSpecificInfo, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithIndex, ltd.qubit.model.mixin.WithAttachments, ltd.qubit.model.mixin.WithOwner, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.commons.CredentialType` | `qubit_model::commons::CredentialType` | enum | - | planned |
| `ltd.qubit.model.commons.Currency` | `qubit_model::commons::Currency` | enum | - | planned |
| `ltd.qubit.model.commons.DayType` | `qubit_model::commons::DayType` | enum | - | planned |
| `ltd.qubit.model.commons.DictEntryInfo` | `qubit_model::commons::DictEntryInfo` | struct (Java class) | ltd.qubit.commons.mixin.Deletable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Normalizable | planned |
| `ltd.qubit.model.commons.DictEntry` | `qubit_model::commons::DictEntry` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasSpecificInfo, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithCode, ltd.qubit.commons.mixin.WithComment, ltd.qubit.commons.mixin.WithName, ltd.qubit.model.mixin.StatefulInfo | planned |
| `ltd.qubit.model.commons.Dict` | `qubit_model::commons::Dict` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable, ltd.qubit.commons.mixin.WithComment, ltd.qubit.model.mixin.HasStatefulInfo, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.mixin.WithCategory | planned |
| `ltd.qubit.model.commons.Faq` | `qubit_model::commons::Faq` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.mixin.Stateful, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithCategory | planned |
| `ltd.qubit.model.commons.FullDict` | `qubit_model::commons::FullDict` | struct (Java class) | - | planned |
| `ltd.qubit.model.commons.Kinship` | `qubit_model::commons::Kinship` | enum | - | planned |
| `ltd.qubit.model.commons.MqFailedTask` | `qubit_model::commons::MqFailedTask` | final struct (Java class) | ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.commons.MqType` | `qubit_model::commons::MqType` | enum | - | planned |
| `ltd.qubit.model.commons.Owner` | `qubit_model::commons::Owner` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable | planned |
| `ltd.qubit.model.commons.Owners` | `qubit_model::commons::Owners` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable | planned |
| `ltd.qubit.model.commons.Payload` | `qubit_model::commons::Payload` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithKey, ltd.qubit.model.mixin.WithOwner | planned |
| `ltd.qubit.model.commons.RequestStatus` | `qubit_model::commons::RequestStatus` | enum | - | planned |
| `ltd.qubit.model.commons.Schedule` | `qubit_model::commons::Schedule` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Normalizable | planned |
| `ltd.qubit.model.commons.Source` | `qubit_model::commons::Source` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasInfoWithEntity, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable, ltd.qubit.commons.mixin.WithEntity, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.organization.Organization | planned |
| `ltd.qubit.model.commons.State` | `qubit_model::commons::State` | enum | - | planned |
| `ltd.qubit.model.commons.Token` | `qubit_model::commons::Token` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Normalizable | planned |
| `ltd.qubit.model.commons.VerifyState` | `qubit_model::commons::VerifyState` | enum | - | planned |
| `ltd.qubit.model.contact.AddressBuilder` | `qubit_model::contact::AddressBuilder` | struct (Java class) | ltd.qubit.commons.mixin.Info | planned |
| `ltd.qubit.model.contact.Address` | `qubit_model::contact::Address` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.mixin.WithLocation | planned |
| `ltd.qubit.model.contact.City` | `qubit_model::contact::City` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable | planned |
| `ltd.qubit.model.contact.Contact` | `qubit_model::contact::Contact` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.commons.VerifyState, ltd.qubit.model.mixin.WithAddress | planned |
| `ltd.qubit.model.contact.CoordinateSystem` | `qubit_model::contact::CoordinateSystem` | enum | - | planned |
| `ltd.qubit.model.contact.Country` | `qubit_model::contact::Country` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable | planned |
| `ltd.qubit.model.contact.District` | `qubit_model::contact::District` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable | planned |
| `ltd.qubit.model.contact.LocationCodec` | `qubit_model::contact::LocationCodec` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.LocationCoordinateCodec` | `qubit_model::contact::LocationCoordinateCodec` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.LocationCoordinateDeserializer` | `qubit_model::contact::LocationCoordinateDeserializer` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.LocationCoordinateSerializer` | `qubit_model::contact::LocationCoordinateSerializer` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.LocationCoordinateXmlAdapter` | `qubit_model::contact::LocationCoordinateXmlAdapter` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.Location` | `qubit_model::contact::Location` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Normalizable | planned |
| `ltd.qubit.model.contact.PhoneCodec` | `qubit_model::contact::PhoneCodec` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.Phone` | `qubit_model::contact::Phone` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Normalizable | planned |
| `ltd.qubit.model.contact.PhoneJsonDeserializer` | `qubit_model::contact::PhoneJsonDeserializer` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.PhoneJsonKeyDeserializer` | `qubit_model::contact::PhoneJsonKeyDeserializer` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.PhoneJsonSerializer` | `qubit_model::contact::PhoneJsonSerializer` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.PhoneTypeRegister` | `qubit_model::contact::PhoneTypeRegister` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.PhoneXmlAdapter` | `qubit_model::contact::PhoneXmlAdapter` | struct (Java class) | - | planned |
| `ltd.qubit.model.contact.Province` | `qubit_model::contact::Province` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable | planned |
| `ltd.qubit.model.contact.Region` | `qubit_model::contact::Region` | enum | - | planned |
| `ltd.qubit.model.contact.Street` | `qubit_model::contact::Street` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable | planned |
| `ltd.qubit.model.controller.AppAuthenticateParams` | `qubit_model::controller::AppAuthenticateParams` | struct (Java class) | ltd.qubit.model.contact.Location, ltd.qubit.model.system.Platform | planned |
| `ltd.qubit.model.controller.AuditableQueryParams` | `qubit_model::controller::AuditableQueryParams` | abstract struct (Java class) | ltd.qubit.commons.mixin.Auditable | planned |
| `ltd.qubit.model.controller.BindDeviceParams` | `qubit_model::controller::BindDeviceParams` | struct (Java class) | - | planned |
| `ltd.qubit.model.controller.BindEmployeeParams` | `qubit_model::controller::BindEmployeeParams` | struct (Java class) | ltd.qubit.model.contact.Phone, ltd.qubit.model.mixin.StatefulInfo | planned |
| `ltd.qubit.model.controller.BindPersonParams` | `qubit_model::controller::BindPersonParams` | struct (Java class) | ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.contact.Phone | planned |
| `ltd.qubit.model.controller.LoginParams` | `qubit_model::controller::LoginParams` | struct (Java class) | ltd.qubit.commons.mixin.Desensitizable, ltd.qubit.model.contact.Phone, ltd.qubit.model.person.SocialNetwork, ltd.qubit.model.system.Environment | planned |
| `ltd.qubit.model.controller.LoginResponse` | `qubit_model::controller::LoginResponse` | struct (Java class) | ltd.qubit.model.commons.Token, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.person.UserInfo, ltd.qubit.model.system.Session | planned |
| `ltd.qubit.model.controller.RegisterUserParams` | `qubit_model::controller::RegisterUserParams` | struct (Java class) | ltd.qubit.commons.mixin.Desensitizable, ltd.qubit.model.contact.Phone, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.organization.Organization, ltd.qubit.model.person.Gender, ltd.qubit.model.person.SocialNetwork, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo, ltd.qubit.model.system.Environment | planned |
| `ltd.qubit.model.controller.UnupdatableQueryParams` | `qubit_model::controller::UnupdatableQueryParams` | abstract struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Deletable | planned |
| `ltd.qubit.model.controller.UpdatePasswordParams` | `qubit_model::controller::UpdatePasswordParams` | struct (Java class) | - | planned |
| `ltd.qubit.model.device.DataNetworkType` | `qubit_model::device::DataNetworkType` | enum | - | planned |
| `ltd.qubit.model.device.DeviceCurrentData` | `qubit_model::device::DeviceCurrentData` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable | planned |
| `ltd.qubit.model.device.DeviceInfo` | `qubit_model::device::DeviceInfo` | struct (Java class) | ltd.qubit.commons.mixin.Deletable, ltd.qubit.model.commons.State, ltd.qubit.model.contact.Location, ltd.qubit.model.mixin.HasStatefulInfo, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.person.PersonInfo | planned |
| `ltd.qubit.model.device.Device` | `qubit_model::device::Device` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasSpecificInfo, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithCode, ltd.qubit.commons.mixin.WithComment, ltd.qubit.commons.mixin.WithName, ltd.qubit.model.commons.App, ltd.qubit.model.commons.Payload, ltd.qubit.model.commons.State, ltd.qubit.model.contact.Address, ltd.qubit.model.contact.Location, ltd.qubit.model.mixin.Stateful, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.mixin.WithLocation, ltd.qubit.model.mixin.WithPayloads, ltd.qubit.model.person.Person, ltd.qubit.model.person.PersonInfo, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.device.DeviceType` | `qubit_model::device::DeviceType` | enum | - | planned |
| `ltd.qubit.model.device.Hardware` | `qubit_model::device::Hardware` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.device.SimCard` | `qubit_model::device::SimCard` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.contact.Location, ltd.qubit.model.contact.Phone | planned |
| `ltd.qubit.model.device.SimCardStatus` | `qubit_model::device::SimCardStatus` | enum | - | planned |
| `ltd.qubit.model.device.Software` | `qubit_model::device::Software` | struct (Java class) | ltd.qubit.commons.mixin.HasInfo, ltd.qubit.model.system.Platform | planned |
| `ltd.qubit.model.Entity` | `qubit_model::Entity` | enum | ltd.qubit.model.activity.Activity, ltd.qubit.model.activity.ActivityCoupon, ltd.qubit.model.activity.ActivityProductItem, ltd.qubit.model.appointment.Appointment, ltd.qubit.model.claim.InsuranceClaim, ltd.qubit.model.claim.InsuranceClaimAmount, ltd.qubit.model.claim.InsuranceClaimEvent, ltd.qubit.model.claim.InsuranceClaimInvoice, ltd.qubit.model.claim.InsuranceClaimInvoiceCost, ltd.qubit.model.claim.InsuranceClaimMedical, ltd.qubit.model.claim.InsuranceProductRule, ltd.qubit.model.claim.enterprise.EnterpriseClaim, ltd.qubit.model.claim.enterprise.EnterpriseClaimInvoice, ltd.qubit.model.claim.enterprise.EnterpriseClaimItem, ltd.qubit.model.claim.enterprise.EnterpriseClaimMedical, ltd.qubit.model.commons.App, ltd.qubit.model.commons.AppResource, ltd.qubit.model.commons.Category, ltd.qubit.model.commons.CodeMap, ltd.qubit.model.commons.Credential, ltd.qubit.model.commons.Dict, ltd.qubit.model.commons.DictEntry, ltd.qubit.model.commons.Faq, ltd.qubit.model.commons.MqFailedTask, ltd.qubit.model.commons.Payload, ltd.qubit.model.commons.Source, ltd.qubit.model.contact.Address, ltd.qubit.model.contact.City, ltd.qubit.model.contact.Country, ltd.qubit.model.contact.District, ltd.qubit.model.contact.Province, ltd.qubit.model.contact.Street, ltd.qubit.model.device.Device, ltd.qubit.model.device.Hardware, ltd.qubit.model.device.SimCard, ltd.qubit.model.device.Software, ltd.qubit.model.feedback.Feedback, ltd.qubit.model.feedback.FeedbackTrack, ltd.qubit.model.medical.Diagnosis, ltd.qubit.model.medical.Disease, ltd.qubit.model.medical.Drug, ltd.qubit.model.medical.DrugProduct, ltd.qubit.model.medical.HospitalDrugstore, ltd.qubit.model.medical.Patient, ltd.qubit.model.medical.Prescription, ltd.qubit.model.medical.PrescriptionContent, ltd.qubit.model.medical.PrescriptionItem, ltd.qubit.model.order.Client, ltd.qubit.model.order.ClientOrder, ltd.qubit.model.order.Consignee, ltd.qubit.model.order.Order, ltd.qubit.model.order.OrderItem, ltd.qubit.model.order.RefererInfo, ltd.qubit.model.order.RefererOrderRecord, ltd.qubit.model.order.Return, ltd.qubit.model.organization.Department, ltd.qubit.model.organization.Employee, ltd.qubit.model.organization.Organization, ltd.qubit.model.payment.Account, ltd.qubit.model.payment.Payment, ltd.qubit.model.person.Person, ltd.qubit.model.person.User, ltd.qubit.model.privilege.Role, ltd.qubit.model.privilege.UserRole, ltd.qubit.model.product.Product, ltd.qubit.model.product.ProductItem, ltd.qubit.model.product.ProductPrice, ltd.qubit.model.security.KeyPair, ltd.qubit.model.security.Signature, ltd.qubit.model.service.MedicalItem, ltd.qubit.model.service.MedicalItemUseRecord, ltd.qubit.model.service.MedicalPackage, ltd.qubit.model.service.MedicalPackageItem, ltd.qubit.model.service.UserMedicalItem, ltd.qubit.model.settlement.Transaction, ltd.qubit.model.system.Log, ltd.qubit.model.system.Session, ltd.qubit.model.system.Setting, ltd.qubit.model.system.VerifyCode, ltd.qubit.model.upload.Attachment, ltd.qubit.model.upload.Upload | planned |
| `ltd.qubit.model.feedback.FeedbackAction` | `qubit_model::feedback::FeedbackAction` | enum | - | planned |
| `ltd.qubit.model.feedback.Feedback` | `qubit_model::feedback::Feedback` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithComment, ltd.qubit.commons.mixin.WithStatus, ltd.qubit.model.commons.App, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithAttachments, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.feedback.FeedbackProcessingRule` | `qubit_model::feedback::FeedbackProcessingRule` | struct (Java class) | - | planned |
| `ltd.qubit.model.feedback.FeedbackRating` | `qubit_model::feedback::FeedbackRating` | enum | - | planned |
| `ltd.qubit.model.feedback.FeedbackStatus` | `qubit_model::feedback::FeedbackStatus` | enum | - | planned |
| `ltd.qubit.model.feedback.FeedbackTrack` | `qubit_model::feedback::FeedbackTrack` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithComment, ltd.qubit.model.mixin.WithAttachments, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.feedback.FeedbackType` | `qubit_model::feedback::FeedbackType` | enum | - | planned |
| `ltd.qubit.model.Field` | `qubit_model::Field` | enum | - | planned |
| `ltd.qubit.model.invoice.InvoiceApply` | `qubit_model::invoice::InvoiceApply` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.App, ltd.qubit.model.commons.DictEntry, ltd.qubit.model.commons.DictEntryInfo, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.mixin.WithOrganization, ltd.qubit.model.organization.Organization, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.invoice.InvoiceApplyStatus` | `qubit_model::invoice::InvoiceApplyStatus` | enum | - | planned |
| `ltd.qubit.model.invoice.InvoiceHospitalRegiste` | `qubit_model::invoice::InvoiceHospitalRegiste` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.invoice.InvoiceInfo` | `qubit_model::invoice::InvoiceInfo` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.Currency | planned |
| `ltd.qubit.model.invoice.InvoiceItem` | `qubit_model::invoice::InvoiceItem` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.invoice.Invoice` | `qubit_model::invoice::Invoice` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.WithCode, ltd.qubit.model.commons.App, ltd.qubit.model.commons.Currency, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.mixin.WithOrganization, ltd.qubit.model.organization.Organization, ltd.qubit.model.payment.Participant, ltd.qubit.model.payment.PaymentChannel, ltd.qubit.model.payment.PaymentMode, ltd.qubit.model.settlement.Settlement | planned |
| `ltd.qubit.model.invoice.InvoiceNumberSegment` | `qubit_model::invoice::InvoiceNumberSegment` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.DictEntryInfo, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.mixin.WithOrganization | planned |
| `ltd.qubit.model.invoice.InvoicePlace` | `qubit_model::invoice::InvoicePlace` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.model.commons.State, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.mixin.WithOrganization | planned |
| `ltd.qubit.model.invoice.InvoiceStatus` | `qubit_model::invoice::InvoiceStatus` | enum | - | planned |
| `ltd.qubit.model.invoice.InvoiceStockStatus` | `qubit_model::invoice::InvoiceStockStatus` | enum | - | planned |
| `ltd.qubit.model.invoice.InvoiceTitleType` | `qubit_model::invoice::InvoiceTitleType` | enum | - | planned |
| `ltd.qubit.model.invoice.InvoiceType` | `qubit_model::invoice::InvoiceType` | enum | - | planned |
| `ltd.qubit.model.medical.ClinicInfo` | `qubit_model::medical::ClinicInfo` | struct (Java class) | ltd.qubit.commons.mixin.Info | planned |
| `ltd.qubit.model.medical.Diagnosis` | `qubit_model::medical::Diagnosis` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.model.Entity | planned |
| `ltd.qubit.model.medical.Disease` | `qubit_model::medical::Disease` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Predefinable, ltd.qubit.model.commons.Category | planned |
| `ltd.qubit.model.medical.Dosage` | `qubit_model::medical::Dosage` | struct (Java class) | ltd.qubit.model.commons.DictEntryInfo | planned |
| `ltd.qubit.model.medical.DrugInfo` | `qubit_model::medical::DrugInfo` | struct (Java class) | ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Info, ltd.qubit.model.commons.DictEntryInfo | planned |
| `ltd.qubit.model.medical.Drug` | `qubit_model::medical::Drug` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.Predefinable, ltd.qubit.model.commons.DictEntryInfo | planned |
| `ltd.qubit.model.medical.DrugProduct` | `qubit_model::medical::DrugProduct` | struct (Java class) | ltd.qubit.model.product.ProductInfo | planned |
| `ltd.qubit.model.medical.EmergentClinicInfo` | `qubit_model::medical::EmergentClinicInfo` | struct (Java class) | ltd.qubit.commons.mixin.Info | planned |
| `ltd.qubit.model.medical.ExaminationInfo` | `qubit_model::medical::ExaminationInfo` | struct (Java class) | ltd.qubit.commons.mixin.Info | planned |
| `ltd.qubit.model.medical.HisInfo` | `qubit_model::medical::HisInfo` | abstract struct (Java class) | - | planned |
| `ltd.qubit.model.medical.HospitalDrugstore` | `qubit_model::medical::HospitalDrugstore` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info | planned |
| `ltd.qubit.model.medical.HospitalizationInfo` | `qubit_model::medical::HospitalizationInfo` | struct (Java class) | ltd.qubit.commons.mixin.Info | planned |
| `ltd.qubit.model.medical.MedicalInvoiceType` | `qubit_model::medical::MedicalInvoiceType` | enum | - | planned |
| `ltd.qubit.model.medical.MedicalPayment` | `qubit_model::medical::MedicalPayment` | struct (Java class) | - | planned |
| `ltd.qubit.model.medical.MedicalSettlementItem` | `qubit_model::medical::MedicalSettlementItem` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.DictEntryInfo | planned |
| `ltd.qubit.model.medical.MedicalSettlement` | `qubit_model::medical::MedicalSettlement` | struct (Java class) | ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.settlement.Settlement | planned |
| `ltd.qubit.model.medical.MedicalType` | `qubit_model::medical::MedicalType` | enum | - | planned |
| `ltd.qubit.model.medical.MedicareItemType` | `qubit_model::medical::MedicareItemType` | enum | - | planned |
| `ltd.qubit.model.medical.MedicareType` | `qubit_model::medical::MedicareType` | enum | - | planned |
| `ltd.qubit.model.medical.PatientInfo` | `qubit_model::medical::PatientInfo` | struct (Java class) | ltd.qubit.commons.mixin.HasInfo, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.contact.Phone, ltd.qubit.model.person.Gender | planned |
| `ltd.qubit.model.medical.Patient` | `qubit_model::medical::Patient` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Info, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.commons.State, ltd.qubit.model.contact.Phone, ltd.qubit.model.mixin.Stateful, ltd.qubit.model.person.Gender, ltd.qubit.model.person.Person, ltd.qubit.model.person.PersonInfo, ltd.qubit.model.person.User | planned |
| `ltd.qubit.model.medical.PrescriptionAction` | `qubit_model::medical::PrescriptionAction` | enum | - | planned |
| `ltd.qubit.model.medical.PrescriptionActionParams` | `qubit_model::medical::PrescriptionActionParams` | struct (Java class) | ltd.qubit.model.Entity | planned |
| `ltd.qubit.model.medical.PrescriptionContent` | `qubit_model::medical::PrescriptionContent` | struct (Java class) | ltd.qubit.commons.mixin.Info, ltd.qubit.model.commons.DictEntryInfo, ltd.qubit.model.organization.EmployeeInfo | planned |
| `ltd.qubit.model.medical.PrescriptionItem` | `qubit_model::medical::PrescriptionItem` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.medical.Prescription` | `qubit_model::medical::Prescription` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.organization.EmployeeInfo, ltd.qubit.model.security.Signature | planned |
| `ltd.qubit.model.medical.PrescriptionOrderRequest` | `qubit_model::medical::PrescriptionOrderRequest` | struct (Java class) | ltd.qubit.model.order.Order | planned |
| `ltd.qubit.model.medical.PrescriptionStatus` | `qubit_model::medical::PrescriptionStatus` | enum | - | planned |
| `ltd.qubit.model.medical.RegistrationInfo` | `qubit_model::medical::RegistrationInfo` | struct (Java class) | ltd.qubit.commons.mixin.Info | planned |
| `ltd.qubit.model.medical.SpecificClinicInfo` | `qubit_model::medical::SpecificClinicInfo` | struct (Java class) | ltd.qubit.commons.mixin.Info | planned |
| `ltd.qubit.model.mixin.Expirable` | `qubit_model::mixin::Expirable` | trait (Java interface) | ltd.qubit.model.system.Expired | planned |
| `ltd.qubit.model.mixin.HasStatefulInfo` | `qubit_model::mixin::HasStatefulInfo` | trait (Java interface) | ltd.qubit.commons.mixin.Deletable, ltd.qubit.commons.mixin.HasSpecificInfo, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.WithCode, ltd.qubit.commons.mixin.WithName | planned |
| `ltd.qubit.model.mixin.InfoWithAppEntity` | `qubit_model::mixin::InfoWithAppEntity` | struct (Java class) | ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.InfoWithEntity | planned |
| `ltd.qubit.model.mixin.InfoWithToken` | `qubit_model::mixin::InfoWithToken` | struct (Java class) | ltd.qubit.commons.mixin.Info, ltd.qubit.model.commons.Token | planned |
| `ltd.qubit.model.mixin.StatefulInfo` | `qubit_model::mixin::StatefulInfo` | struct (Java class) | ltd.qubit.commons.mixin.Deletable, ltd.qubit.commons.mixin.Info, ltd.qubit.model.commons.State | planned |
| `ltd.qubit.model.mixin.StatefulInfoWithToken` | `qubit_model::mixin::StatefulInfoWithToken` | struct (Java class) | ltd.qubit.commons.mixin.Info, ltd.qubit.model.commons.State, ltd.qubit.model.commons.Token | planned |
| `ltd.qubit.model.mixin.Stateful` | `qubit_model::mixin::Stateful` | trait (Java interface) | ltd.qubit.model.commons.State | planned |
| `ltd.qubit.model.mixin.WithAddress` | `qubit_model::mixin::WithAddress` | trait (Java interface) | ltd.qubit.model.contact.Address | planned |
| `ltd.qubit.model.mixin.WithApp` | `qubit_model::mixin::WithApp` | trait (Java interface) | - | planned |
| `ltd.qubit.model.mixin.WithAttachment` | `qubit_model::mixin::WithAttachment` | trait (Java interface) | ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.mixin.WithAttachments` | `qubit_model::mixin::WithAttachments` | trait (Java interface) | ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.mixin.WithCategory` | `qubit_model::mixin::WithCategory` | trait (Java interface) | ltd.qubit.commons.mixin.InfoWithEntity | planned |
| `ltd.qubit.model.mixin.WithContact` | `qubit_model::mixin::WithContact` | trait (Java interface) | ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.contact.Contact | planned |
| `ltd.qubit.model.mixin.WithCreator` | `qubit_model::mixin::WithCreator` | trait (Java interface) | ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.mixin.WithCredential` | `qubit_model::mixin::WithCredential` | trait (Java interface) | ltd.qubit.model.commons.CredentialInfo | planned |
| `ltd.qubit.model.mixin.WithDeleter` | `qubit_model::mixin::WithDeleter` | trait (Java interface) | ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.mixin.WithLocation` | `qubit_model::mixin::WithLocation` | trait (Java interface) | ltd.qubit.model.contact.Location | planned |
| `ltd.qubit.model.mixin.WithMobile` | `qubit_model::mixin::WithMobile` | trait (Java interface) | ltd.qubit.model.contact.Phone | planned |
| `ltd.qubit.model.mixin.WithModifier` | `qubit_model::mixin::WithModifier` | trait (Java interface) | ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.mixin.WithOrganization` | `qubit_model::mixin::WithOrganization` | trait (Java interface) | - | planned |
| `ltd.qubit.model.mixin.WithOwner` | `qubit_model::mixin::WithOwner` | trait (Java interface) | ltd.qubit.model.commons.Owner | planned |
| `ltd.qubit.model.mixin.WithPayloads` | `qubit_model::mixin::WithPayloads` | trait (Java interface) | ltd.qubit.model.commons.Payload | planned |
| `ltd.qubit.model.mixin.WithSource` | `qubit_model::mixin::WithSource` | trait (Java interface) | ltd.qubit.commons.mixin.InfoWithEntity | planned |
| `ltd.qubit.model.mixin.WithStatefulInfoWithToken` | `qubit_model::mixin::WithStatefulInfoWithToken` | trait (Java interface) | - | planned |
| `ltd.qubit.model.mixin.WithToken` | `qubit_model::mixin::WithToken` | trait (Java interface) | ltd.qubit.model.commons.Token | planned |
| `ltd.qubit.model.Module` | `qubit_model::Module` | enum | - | planned |
| `ltd.qubit.model.Operation` | `qubit_model::Operation` | enum | - | planned |
| `ltd.qubit.model.order.Buyer` | `qubit_model::order::Buyer` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.WithBirthday, ltd.qubit.commons.mixin.WithName, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.contact.Contact, ltd.qubit.model.contact.Phone, ltd.qubit.model.mixin.WithCredential, ltd.qubit.model.person.Gender, ltd.qubit.model.person.Person, ltd.qubit.model.person.User | planned |
| `ltd.qubit.model.order.Client` | `qubit_model::order::Client` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.WithBirthday, ltd.qubit.commons.mixin.WithName, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.commons.Kinship, ltd.qubit.model.contact.Contact, ltd.qubit.model.contact.Phone, ltd.qubit.model.medical.MedicareType, ltd.qubit.model.mixin.WithCredential, ltd.qubit.model.person.Gender, ltd.qubit.model.person.Person, ltd.qubit.model.person.PersonInfo | planned |
| `ltd.qubit.model.order.ClientOrder` | `qubit_model::order::ClientOrder` | struct (Java class) | ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.model.invoice.InvoiceStatus, ltd.qubit.model.mixin.WithSource, ltd.qubit.model.organization.Organization, ltd.qubit.model.payment.Account, ltd.qubit.model.payment.PaymentChannel, ltd.qubit.model.payment.PaymentMode, ltd.qubit.model.product.ProductInfo | planned |
| `ltd.qubit.model.order.ClientRefundSubmitRequest` | `qubit_model::order::ClientRefundSubmitRequest` | struct (Java class) | - | planned |
| `ltd.qubit.model.order.ConfirmStatus` | `qubit_model::order::ConfirmStatus` | enum | - | planned |
| `ltd.qubit.model.order.Consignee` | `qubit_model::order::Consignee` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.contact.Address, ltd.qubit.model.contact.Contact, ltd.qubit.model.contact.Phone, ltd.qubit.model.person.Person, ltd.qubit.model.person.User | planned |
| `ltd.qubit.model.order.OpenidType` | `qubit_model::order::OpenidType` | enum | - | planned |
| `ltd.qubit.model.order.OrderDetail` | `qubit_model::order::OrderDetail` | struct (Java class) | ltd.qubit.model.settlement.Transaction | planned |
| `ltd.qubit.model.order.OrderInfo` | `qubit_model::order::OrderInfo` | struct (Java class) | ltd.qubit.commons.mixin.Deletable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.model.commons.Currency, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithCategory, ltd.qubit.model.mixin.WithSource, ltd.qubit.model.product.Seller | planned |
| `ltd.qubit.model.order.OrderItem` | `qubit_model::order::OrderItem` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.commons.Currency, ltd.qubit.model.product.Product, ltd.qubit.model.product.ProductInfo | planned |
| `ltd.qubit.model.order.Order` | `qubit_model::order::Order` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.model.commons.App, ltd.qubit.model.commons.Currency, ltd.qubit.model.invoice.InvoiceStatus, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.mixin.WithCategory, ltd.qubit.model.mixin.WithSource, ltd.qubit.model.person.User, ltd.qubit.model.product.Seller, ltd.qubit.model.shipping.ShippingDemand, ltd.qubit.model.shipping.ShippingMode, ltd.qubit.model.system.Environment | planned |
| `ltd.qubit.model.order.OrderStatus` | `qubit_model::order::OrderStatus` | enum | - | planned |
| `ltd.qubit.model.order.OrderSubmitRequest` | `qubit_model::order::OrderSubmitRequest` | struct (Java class) | - | planned |
| `ltd.qubit.model.order.OrderSubmitResponse` | `qubit_model::order::OrderSubmitResponse` | struct (Java class) | - | planned |
| `ltd.qubit.model.order.PayType` | `qubit_model::order::PayType` | enum | - | planned |
| `ltd.qubit.model.order.RefererInfo` | `qubit_model::order::RefererInfo` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.order.RefererOrderRecord` | `qubit_model::order::RefererOrderRecord` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.product.Product, ltd.qubit.model.product.ProductItem | planned |
| `ltd.qubit.model.order.RefererOrderRecordStatus` | `qubit_model::order::RefererOrderRecordStatus` | enum | - | planned |
| `ltd.qubit.model.order.Replacement` | `qubit_model::order::Replacement` | struct (Java class) | - | planned |
| `ltd.qubit.model.order.ReturnIssuer` | `qubit_model::order::ReturnIssuer` | enum | - | planned |
| `ltd.qubit.model.order.Return` | `qubit_model::order::Return` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.Currency, ltd.qubit.model.invoice.InvoiceStatus, ltd.qubit.model.product.Product, ltd.qubit.model.product.ProductInfo, ltd.qubit.model.settlement.Transaction, ltd.qubit.model.system.Environment | planned |
| `ltd.qubit.model.order.ReturnReason` | `qubit_model::order::ReturnReason` | enum | - | planned |
| `ltd.qubit.model.order.ReturnStatus` | `qubit_model::order::ReturnStatus` | enum | - | planned |
| `ltd.qubit.model.organization.Department` | `qubit_model::organization::Department` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable, ltd.qubit.model.commons.Category, ltd.qubit.model.commons.Payload, ltd.qubit.model.commons.State, ltd.qubit.model.contact.Contact, ltd.qubit.model.mixin.HasStatefulInfo, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithCategory, ltd.qubit.model.mixin.WithContact, ltd.qubit.model.mixin.WithPayloads | planned |
| `ltd.qubit.model.organization.EmployeeInfo` | `qubit_model::organization::EmployeeInfo` | struct (Java class) | ltd.qubit.commons.mixin.Deletable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithBirthday, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.commons.State, ltd.qubit.model.contact.Phone, ltd.qubit.model.mixin.Stateful, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.person.Gender, ltd.qubit.model.person.User, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.organization.Employee` | `qubit_model::organization::Employee` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasSpecificInfo, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithBirthday, ltd.qubit.commons.mixin.WithCode, ltd.qubit.commons.mixin.WithComment, ltd.qubit.commons.mixin.WithEmail, ltd.qubit.commons.mixin.WithName, ltd.qubit.commons.mixin.WithUsername, ltd.qubit.model.commons.Category, ltd.qubit.model.commons.Credential, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.commons.State, ltd.qubit.model.contact.Phone, ltd.qubit.model.mixin.Stateful, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithCategory, ltd.qubit.model.mixin.WithCredential, ltd.qubit.model.mixin.WithMobile, ltd.qubit.model.mixin.WithOrganization, ltd.qubit.model.person.Gender, ltd.qubit.model.person.Person, ltd.qubit.model.person.User, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.organization.Organization` | `qubit_model::organization::Organization` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable, ltd.qubit.commons.mixin.WithComment, ltd.qubit.model.commons.Category, ltd.qubit.model.commons.Credential, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.commons.Payload, ltd.qubit.model.commons.State, ltd.qubit.model.contact.Contact, ltd.qubit.model.mixin.HasStatefulInfo, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithCategory, ltd.qubit.model.mixin.WithContact, ltd.qubit.model.mixin.WithPayloads, ltd.qubit.model.person.PersonInfo, ltd.qubit.model.product.Seller | planned |
| `ltd.qubit.model.organization.TaxPayerType` | `qubit_model::organization::TaxPayerType` | enum | - | planned |
| `ltd.qubit.model.payment.Account` | `qubit_model::payment::Account` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp | planned |
| `ltd.qubit.model.payment.AccountType` | `qubit_model::payment::AccountType` | enum | - | planned |
| `ltd.qubit.model.payment.Participant` | `qubit_model::payment::Participant` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.contact.Contact, ltd.qubit.model.contact.Phone, ltd.qubit.model.mixin.WithCredential, ltd.qubit.model.order.Buyer, ltd.qubit.model.order.Client, ltd.qubit.model.organization.Organization, ltd.qubit.model.person.Person, ltd.qubit.model.product.Seller | planned |
| `ltd.qubit.model.payment.ParticipantType` | `qubit_model::payment::ParticipantType` | enum | - | planned |
| `ltd.qubit.model.payment.PaymentChannel` | `qubit_model::payment::PaymentChannel` | enum | - | planned |
| `ltd.qubit.model.payment.Payment` | `qubit_model::payment::Payment` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.commons.Currency, ltd.qubit.model.order.Order, ltd.qubit.model.settlement.Transaction, ltd.qubit.model.system.Environment | planned |
| `ltd.qubit.model.payment.PaymentMode` | `qubit_model::payment::PaymentMode` | enum | - | planned |
| `ltd.qubit.model.payment.PaymentOption` | `qubit_model::payment::PaymentOption` | enum | - | planned |
| `ltd.qubit.model.payment.PaymentRequest` | `qubit_model::payment::PaymentRequest` | struct (Java class) | ltd.qubit.model.settlement.Transaction | planned |
| `ltd.qubit.model.payment.PaymentRequestTransformer` | `qubit_model::payment::PaymentRequestTransformer` | struct (Java class) | ltd.qubit.model.settlement.Transaction | planned |
| `ltd.qubit.model.payment.PaymentResponseBase64` | `qubit_model::payment::PaymentResponseBase64` | struct (Java class) | - | planned |
| `ltd.qubit.model.payment.PaymentResponse` | `qubit_model::payment::PaymentResponse` | struct (Java class) | - | planned |
| `ltd.qubit.model.payment.PaymentType` | `qubit_model::payment::PaymentType` | enum | - | planned |
| `ltd.qubit.model.person.Blood` | `qubit_model::person::Blood` | enum | - | planned |
| `ltd.qubit.model.person.Education` | `qubit_model::person::Education` | enum | - | planned |
| `ltd.qubit.model.person.Ethnic` | `qubit_model::person::Ethnic` | enum | - | planned |
| `ltd.qubit.model.person.Gender` | `qubit_model::person::Gender` | enum | - | planned |
| `ltd.qubit.model.person.Incoming` | `qubit_model::person::Incoming` | enum | - | planned |
| `ltd.qubit.model.person.Industry` | `qubit_model::person::Industry` | enum | - | planned |
| `ltd.qubit.model.person.JobTitle` | `qubit_model::person::JobTitle` | enum | - | planned |
| `ltd.qubit.model.person.Marriage` | `qubit_model::person::Marriage` | enum | - | planned |
| `ltd.qubit.model.person.PersonInfo` | `qubit_model::person::PersonInfo` | struct (Java class) | ltd.qubit.commons.mixin.Deletable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithBirthday, ltd.qubit.commons.mixin.WithName, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.contact.Contact, ltd.qubit.model.contact.Phone, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.person.Person` | `qubit_model::person::Person` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasSpecificInfo, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithBirthday, ltd.qubit.commons.mixin.WithComment, ltd.qubit.commons.mixin.WithName, ltd.qubit.commons.mixin.WithUsername, ltd.qubit.model.commons.Category, ltd.qubit.model.commons.Credential, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.commons.Source, ltd.qubit.model.contact.City, ltd.qubit.model.contact.Contact, ltd.qubit.model.contact.Country, ltd.qubit.model.contact.Province, ltd.qubit.model.medical.MedicareType, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithCategory, ltd.qubit.model.mixin.WithContact, ltd.qubit.model.mixin.WithCredential, ltd.qubit.model.order.Buyer, ltd.qubit.model.order.Client, ltd.qubit.model.order.Consignee, ltd.qubit.model.organization.Organization, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.person.Politics` | `qubit_model::person::Politics` | enum | - | planned |
| `ltd.qubit.model.person.Religion` | `qubit_model::person::Religion` | enum | - | planned |
| `ltd.qubit.model.person.SexOrientation` | `qubit_model::person::SexOrientation` | enum | - | planned |
| `ltd.qubit.model.person.SocialNetworkAccount` | `qubit_model::person::SocialNetworkAccount` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.commons.Payload, ltd.qubit.model.controller.RegisterUserParams, ltd.qubit.model.mixin.WithPayloads | planned |
| `ltd.qubit.model.person.SocialNetwork` | `qubit_model::person::SocialNetwork` | enum | - | planned |
| `ltd.qubit.model.person.UserInfo` | `qubit_model::person::UserInfo` | struct (Java class) | ltd.qubit.commons.mixin.Deletable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.WithName, ltd.qubit.commons.mixin.WithUsername, ltd.qubit.model.commons.State, ltd.qubit.model.contact.Phone, ltd.qubit.model.mixin.Stateful | planned |
| `ltd.qubit.model.person.User` | `qubit_model::person::User` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Desensitizable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.HasSpecificInfo, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.commons.mixin.Predefinable, ltd.qubit.commons.mixin.WithComment, ltd.qubit.commons.mixin.WithEmail, ltd.qubit.commons.mixin.WithName, ltd.qubit.commons.mixin.WithPassword, ltd.qubit.commons.mixin.WithUsername, ltd.qubit.model.commons.AuthorizeRecord, ltd.qubit.model.commons.State, ltd.qubit.model.commons.VerifyState, ltd.qubit.model.contact.Phone, ltd.qubit.model.controller.RegisterUserParams, ltd.qubit.model.mixin.Stateful, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithMobile, ltd.qubit.model.mixin.WithOrganization, ltd.qubit.model.organization.Organization | planned |
| `ltd.qubit.model.privilege.PrivilegesCodec` | `qubit_model::privilege::PrivilegesCodec` | struct (Java class) | - | planned |
| `ltd.qubit.model.privilege.Privileges` | `qubit_model::privilege::Privileges` | struct (Java class) | - | planned |
| `ltd.qubit.model.privilege.Role` | `qubit_model::privilege::Role` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.model.commons.App, ltd.qubit.model.commons.State, ltd.qubit.model.mixin.HasStatefulInfo, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp | planned |
| `ltd.qubit.model.privilege.UserRole` | `qubit_model::privilege::UserRole` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.App, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.product.Coupon` | `qubit_model::product::Coupon` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.model.commons.App, ltd.qubit.model.commons.State, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp | planned |
| `ltd.qubit.model.product.CouponRule` | `qubit_model::product::CouponRule` | struct (Java class) | ltd.qubit.model.commons.App, ltd.qubit.model.mixin.StatefulInfo | planned |
| `ltd.qubit.model.product.CouponType` | `qubit_model::product::CouponType` | enum | - | planned |
| `ltd.qubit.model.product.PersonConstraint` | `qubit_model::product::PersonConstraint` | struct (Java class) | ltd.qubit.model.person.Gender | planned |
| `ltd.qubit.model.product.ProductConstraint` | `qubit_model::product::ProductConstraint` | struct (Java class) | - | planned |
| `ltd.qubit.model.product.ProductInfo` | `qubit_model::product::ProductInfo` | struct (Java class) | ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Info, ltd.qubit.model.commons.Currency, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.product.ProductItem` | `qubit_model::product::ProductItem` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.Entity, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.product.Product` | `qubit_model::product::Product` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.commons.mixin.Info, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.model.commons.App, ltd.qubit.model.commons.Category, ltd.qubit.model.commons.Currency, ltd.qubit.model.commons.State, ltd.qubit.model.mixin.Stateful, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.mixin.WithCategory, ltd.qubit.model.upload.Attachment | planned |
| `ltd.qubit.model.product.ProductPrice` | `qubit_model::product::ProductPrice` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info | planned |
| `ltd.qubit.model.product.Quality` | `qubit_model::product::Quality` | enum | - | planned |
| `ltd.qubit.model.product.Seller` | `qubit_model::product::Seller` | struct (Java class) | ltd.qubit.commons.mixin.HasInfo, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.contact.Address, ltd.qubit.model.contact.Contact, ltd.qubit.model.contact.Phone, ltd.qubit.model.organization.Organization | planned |
| `ltd.qubit.model.security.KeyPair` | `qubit_model::security::KeyPair` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.State, ltd.qubit.model.mixin.Stateful | planned |
| `ltd.qubit.model.security.Signature` | `qubit_model::security::Signature` | struct (Java class) | - | planned |
| `ltd.qubit.model.security.SignedData` | `qubit_model::security::SignedData` | final struct (Java class) | - | planned |
| `ltd.qubit.model.security.SignedInfo` | `qubit_model::security::SignedInfo` | struct (Java class) | - | planned |
| `ltd.qubit.model.service.EmployeeMedicalItem` | `qubit_model::service::EmployeeMedicalItem` | struct (Java class) | ltd.qubit.model.organization.Employee, ltd.qubit.model.organization.Organization | planned |
| `ltd.qubit.model.service.MedicalItem` | `qubit_model::service::MedicalItem` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.HasInfo | planned |
| `ltd.qubit.model.service.MedicalItemUseRecord` | `qubit_model::service::MedicalItemUseRecord` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.organization.EmployeeInfo, ltd.qubit.model.person.PersonInfo | planned |
| `ltd.qubit.model.service.MedicalPackageItem` | `qubit_model::service::MedicalPackageItem` | struct (Java class) | - | planned |
| `ltd.qubit.model.service.MedicalPackage` | `qubit_model::service::MedicalPackage` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.HasInfo, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.organization.Organization | planned |
| `ltd.qubit.model.service.UserMedicalItem` | `qubit_model::service::UserMedicalItem` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.service.UserMedicalPackage` | `qubit_model::service::UserMedicalPackage` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.person.User | planned |
| `ltd.qubit.model.service.UserServiceState` | `qubit_model::service::UserServiceState` | enum | - | planned |
| `ltd.qubit.model.settlement.Settlement` | `qubit_model::settlement::Settlement` | abstract struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.App, ltd.qubit.model.medical.MedicalSettlement, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.organization.Organization | planned |
| `ltd.qubit.model.settlement.Transaction` | `qubit_model::settlement::Transaction` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.model.commons.App, ltd.qubit.model.commons.Category, ltd.qubit.model.commons.Currency, ltd.qubit.model.commons.Source, ltd.qubit.model.invoice.InvoiceStatus, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.mixin.WithCategory, ltd.qubit.model.mixin.WithSource, ltd.qubit.model.order.Order, ltd.qubit.model.order.OrderInfo, ltd.qubit.model.order.Return, ltd.qubit.model.order.ReturnIssuer, ltd.qubit.model.payment.Participant, ltd.qubit.model.payment.Payment, ltd.qubit.model.system.Environment | planned |
| `ltd.qubit.model.settlement.TransactionStatus` | `qubit_model::settlement::TransactionStatus` | enum | - | planned |
| `ltd.qubit.model.settlement.TransactionType` | `qubit_model::settlement::TransactionType` | enum | - | planned |
| `ltd.qubit.model.shipping.ConsignInfo` | `qubit_model::shipping::ConsignInfo` | struct (Java class) | ltd.qubit.commons.mixin.WithName, ltd.qubit.model.commons.CredentialInfo, ltd.qubit.model.contact.Address, ltd.qubit.model.contact.Phone, ltd.qubit.model.mixin.WithCredential | planned |
| `ltd.qubit.model.shipping.Packing` | `qubit_model::shipping::Packing` | enum | - | planned |
| `ltd.qubit.model.shipping.ShippingDemand` | `qubit_model::shipping::ShippingDemand` | struct (Java class) | ltd.qubit.model.commons.DayType | planned |
| `ltd.qubit.model.shipping.Shipping` | `qubit_model::shipping::Shipping` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Info, ltd.qubit.model.organization.Organization | planned |
| `ltd.qubit.model.shipping.ShippingMode` | `qubit_model::shipping::ShippingMode` | enum | - | planned |
| `ltd.qubit.model.statistics.CategoryValue` | `qubit_model::statistics::CategoryValue` | struct (Java class) | - | planned |
| `ltd.qubit.model.statistics.StatsDataset` | `qubit_model::statistics::StatsDataset` | struct (Java class) | - | planned |
| `ltd.qubit.model.statistics.StatsItem` | `qubit_model::statistics::StatsItem` | struct (Java class) | - | planned |
| `ltd.qubit.model.statistics.TimeDimension` | `qubit_model::statistics::TimeDimension` | enum | - | planned |
| `ltd.qubit.model.system.Environment` | `qubit_model::system::Environment` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.contact.Location | planned |
| `ltd.qubit.model.system.Expired` | `qubit_model::system::Expired` | struct (Java class) | - | planned |
| `ltd.qubit.model.system.ExpiredReason` | `qubit_model::system::ExpiredReason` | enum | - | planned |
| `ltd.qubit.model.system.Host` | `qubit_model::system::Host` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.WithUdid | planned |
| `ltd.qubit.model.system.Log` | `qubit_model::system::Log` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.system.OperationLogInfo` | `qubit_model::system::OperationLogInfo` | struct (Java class) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.system.OperationLog` | `qubit_model::system::OperationLog` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.HasSpecificInfo, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Modifiable, ltd.qubit.model.commons.App, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.system.Platform` | `qubit_model::system::Platform` | enum | - | planned |
| `ltd.qubit.model.system.Session` | `qubit_model::system::Session` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.Normalizable, ltd.qubit.model.commons.App, ltd.qubit.model.commons.Token, ltd.qubit.model.mixin.Expirable, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp, ltd.qubit.model.mixin.WithToken, ltd.qubit.model.organization.Organization, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo, ltd.qubit.model.privilege.Role | planned |
| `ltd.qubit.model.system.Setting` | `qubit_model::system::Setting` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Modifiable, ltd.qubit.commons.mixin.WithName | planned |
| `ltd.qubit.model.system.SettingJsonDeserializer` | `qubit_model::system::SettingJsonDeserializer` | struct (Java class) | - | planned |
| `ltd.qubit.model.system.SettingJsonSerializer` | `qubit_model::system::SettingJsonSerializer` | struct (Java class) | - | planned |
| `ltd.qubit.model.system.SettingRandomizer` | `qubit_model::system::SettingRandomizer` | struct (Java class) | - | planned |
| `ltd.qubit.model.system.SettingXmlAdapter` | `qubit_model::system::SettingXmlAdapter` | struct (Java class) | - | planned |
| `ltd.qubit.model.system.VerifyCode` | `qubit_model::system::VerifyCode` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Desensitizable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.commons.App, ltd.qubit.model.contact.Phone, ltd.qubit.model.mixin.StatefulInfo, ltd.qubit.model.mixin.WithApp | planned |
| `ltd.qubit.model.system.VerifyScene` | `qubit_model::system::VerifyScene` | enum | - | planned |
| `ltd.qubit.model.task.TaskAction` | `qubit_model::task::TaskAction` | enum | - | planned |
| `ltd.qubit.model.task.TaskInfo` | `qubit_model::task::TaskInfo` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.Modifiable, ltd.qubit.commons.mixin.WithStatus, ltd.qubit.model.commons.Category, ltd.qubit.model.mixin.WithCreator, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo | planned |
| `ltd.qubit.model.task.Task` | `qubit_model::task::Task` | trait (Java interface) | ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.InfoWithEntity | planned |
| `ltd.qubit.model.task.TaskPipeline` | `qubit_model::task::TaskPipeline` | trait (Java interface) | ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.task.TaskPipelineStatus` | `qubit_model::task::TaskPipelineStatus` | enum | - | planned |
| `ltd.qubit.model.task.TaskStatistics` | `qubit_model::task::TaskStatistics` | struct (Java class) | - | planned |
| `ltd.qubit.model.task.TaskStatus` | `qubit_model::task::TaskStatus` | enum | - | planned |
| `ltd.qubit.model.task.TaskStatusTransitionRule` | `qubit_model::task::TaskStatusTransitionRule` | struct (Java class) | - | planned |
| `ltd.qubit.model.thirdpart.WechatJsConfig` | `qubit_model::thirdpart::WechatJsConfig` | struct (Java class) | - | planned |
| `ltd.qubit.model.upload.Attachment` | `qubit_model::upload::Attachment` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.commons.mixin.InfoWithEntity, ltd.qubit.commons.mixin.WithIndex, ltd.qubit.commons.mixin.WithVisibility, ltd.qubit.model.commons.Category, ltd.qubit.model.commons.Owner, ltd.qubit.model.commons.State, ltd.qubit.model.mixin.Stateful, ltd.qubit.model.mixin.WithCategory, ltd.qubit.model.mixin.WithCreator, ltd.qubit.model.mixin.WithOwner, ltd.qubit.model.person.User, ltd.qubit.model.person.UserInfo, ltd.qubit.model.system.Session | planned |
| `ltd.qubit.model.upload.AttachmentType` | `qubit_model::upload::AttachmentType` | enum | - | planned |
| `ltd.qubit.model.upload.FileInfo` | `qubit_model::upload::FileInfo` | struct (Java class) | ltd.qubit.commons.mixin.Emptyful | planned |
| `ltd.qubit.model.upload.MediaInfo` | `qubit_model::upload::MediaInfo` | struct (Java class) | - | planned |
| `ltd.qubit.model.upload.MediaType` | `qubit_model::upload::MediaType` | enum | - | planned |
| `ltd.qubit.model.upload.Upload` | `qubit_model::upload::Upload` | struct (Java class) | ltd.qubit.commons.mixin.Creatable, ltd.qubit.commons.mixin.Deletable, ltd.qubit.commons.mixin.Emptyful, ltd.qubit.commons.mixin.Identifiable | planned |
| `ltd.qubit.model.upload.UploadParams` | `qubit_model::upload::UploadParams` | struct (Java class) | - | planned |
| `ltd.qubit.model.util.MessageFormatter` | `qubit_model::util::MessageFormatter` | struct (Java class) | ltd.qubit.model.Entity, ltd.qubit.model.commons.CredentialType, ltd.qubit.model.commons.Currency, ltd.qubit.model.person.Gender | planned |
| `ltd.qubit.model.util.Result` | `qubit_model::util::Result` | struct (Java class) | - | planned |
| `ltd.qubit.model.work.WorkSchedule` | `qubit_model::work::WorkSchedule` | struct (Java class) | ltd.qubit.commons.mixin.Auditable, ltd.qubit.commons.mixin.Identifiable, ltd.qubit.model.organization.Employee | planned |
| `ltd.qubit.model.system.SettingXmlAdapter.Adapted` | `qubit_model::system::setting_xml_adapter::Adapted` | struct (nested Java class) | - | planned |
<!-- inventory rows -->
