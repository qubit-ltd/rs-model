// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Standard field classifications used by model payloads and validation messages.

use serde::Deserialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Identifies a named property used across domain models and localized messages.
#[derive(Model, Redact, Clone, Copy, Deserialize, Eq, PartialEq)]
#[redact(debug, display, serde)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Field {
    /// The requested business action.
    Action,
    /// The value observed during validation.
    Actual,
    /// The actual credential number used by the domain model.
    ActualCredentialNumber,
    /// The actual credential type used by the domain model.
    ActualCredentialType,
    /// The address's detail.
    AddressDetail,
    /// The address's latitude.
    AddressLatitude,
    /// The address's longitude.
    AddressLongitude,
    /// The address's postalcode.
    AddressPostalcode,
    /// The address's street id.
    AddressStreetId,
    /// The address's verify.
    AddressVerify,
    /// The person's age.
    Age,
    /// The geographic altitude.
    Altitude,
    /// The associated tenant application.
    App,
    /// The identifier of the associated app.
    AppId,
    /// The avatar image reference.
    Avatar,
    /// The person's date of birth.
    Birthday,
    /// The person's blood type.
    Blood,
    /// The purchasing party.
    Buyer,
    /// The identifier of the associated buyer.
    BuyerId,
    /// The identifier of the associated category.
    CategoryId,
    /// The change password used by the domain model.
    ChangePassword,
    /// The originating business channel.
    Channel,
    /// The channel number used by the domain model.
    ChannelNumber,
    /// The associated city.
    City,
    /// The identifier of the associated city.
    CityId,
    /// The client associated with the record.
    Client,
    /// A business code.
    Code,
    /// Free-form explanatory text.
    Comment,
    /// The time at which complete occurred.
    CompleteTime,
    /// A cost amount.
    Cost,
    /// A quantity.
    Count,
    /// The associated country.
    Country,
    /// The identifier of the associated country.
    CountryId,
    /// The time at which create occurred.
    CreateTime,
    /// The identifier of the associated creator.
    CreatorId,
    /// An identity credential.
    Credential,
    /// The credential number used by the domain model.
    CredentialNumber,
    /// The credential type used by the domain model.
    CredentialType,
    /// The credential verify used by the domain model.
    CredentialVerify,
    /// The currency of a monetary amount.
    Currency,
    /// The identifier of the associated deleter.
    DeleterId,
    /// The time at which delete occurred.
    DeleteTime,
    /// A human-readable description.
    Description,
    /// The associated reference-data dictionary.
    Dict,
    /// A discount amount.
    Discount,
    /// The discount reason used by the domain model.
    DiscountReason,
    /// The associated district.
    District,
    /// The identifier of the associated district.
    DistrictId,
    /// The person's education level.
    Education,
    /// An email address.
    Email,
    /// The email verify used by the domain model.
    EmailVerify,
    /// Whether the value is encrypted.
    Encrypted,
    /// The referenced domain entity type.
    Entity,
    /// The error code used by the domain model.
    ErrorCode,
    /// The error message used by the domain model.
    ErrorMessage,
    /// The error params used by the domain model.
    ErrorParams,
    /// The error type used by the domain model.
    ErrorType,
    /// The person's ethnic group.
    Ethnic,
    /// The value expected by validation.
    Expected,
    /// The expected credential number used by the domain model.
    ExpectedCredentialNumber,
    /// The expected credential type used by the domain model.
    ExpectedCredentialType,
    /// The expired reason used by the domain model.
    ExpiredReason,
    /// The time at which expired occurred.
    ExpiredTime,
    /// The person's full name.
    Fullname,
    /// The person's gender.
    Gender,
    /// The person's guardian.
    Guardian,
    /// Whether the subject has child.
    HasChild,
    /// Whether the subject has medicare.
    HasMedicare,
    /// Whether the subject has medicare or social security.
    HasMedicareOrSocialSecurity,
    /// Whether the subject has social security.
    HasSocialSecurity,
    /// The person's height.
    Height,
    /// The host name or address.
    Host,
    /// An icon reference.
    Icon,
    /// The record identifier.
    Id,
    /// An image reference.
    Image,
    /// The person's income range.
    Incoming,
    /// The associated industry.
    Industry,
    /// The available inventory quantity.
    Inventory,
    /// The invoice status used by the domain model.
    InvoiceStatus,
    /// The network IP address.
    Ip,
    /// The identifier of the associated item.
    ItemId,
    /// The person's job description.
    Job,
    /// The job title used by the domain model.
    JobTitle,
    /// The time at which last active occurred.
    LastActiveTime,
    /// The last login failures used by the domain model.
    LastLoginFailures,
    /// The time at which last login occurred.
    LastLoginTime,
    /// The geographic latitude.
    Latitude,
    /// The hierarchy level.
    Level,
    /// The time at which login occurred.
    LoginTime,
    /// The geographic longitude.
    Longitude,
    /// The identifier of the associated manufacturer.
    ManufacturerId,
    /// The person's marital status.
    Marriage,
    /// The medicare city used by the domain model.
    MedicareCity,
    /// The identifier of the associated medicare city.
    MedicareCityId,
    /// A mobile telephone number.
    Mobile,
    /// The mobile number's city area.
    MobileCityArea,
    /// The mobile number's country area.
    MobileCountryArea,
    /// The mobile number's number.
    MobileNumber,
    /// The mobile number's verify.
    MobileVerify,
    /// The selected operating mode.
    Mode,
    /// The identifier of the associated modifier.
    ModifierId,
    /// The time at which modify occurred.
    ModifyTime,
    /// Whether multiple values are permitted.
    Multiple,
    /// The display name.
    Name,
    /// The need delivery used by the domain model.
    NeedDelivery,
    /// The proposed credential number.
    NewCredentialNumber,
    /// The proposed credential type.
    NewCredentialType,
    /// The proposed email.
    NewEmail,
    /// The proposed fullname.
    NewFullname,
    /// The proposed mobile.
    NewMobile,
    /// The person's nickname.
    Nickname,
    /// Whether a value may be absent.
    Nullable,
    /// A business number.
    Number,
    /// The previous credential number.
    OldCredentialNumber,
    /// The previous credential type.
    OldCredentialType,
    /// The previous email.
    OldEmail,
    /// The previous fullname.
    OldFullname,
    /// The previous mobile.
    OldMobile,
    /// The requested authorized operation.
    Operation,
    /// The identifier of the associated operator.
    OperatorId,
    /// The identifier of the associated order.
    OrderId,
    /// The associated order line.
    OrderItem,
    /// The identifier of the associated order item.
    OrderItemId,
    /// The order title used by the domain model.
    OrderTitle,
    /// The associated organization.
    Organization,
    /// The identifier of the associated organization.
    OrganizationId,
    /// The organization name used by the domain model.
    OrganizationName,
    /// The identifier of the associated origin.
    OriginId,
    /// The owning party.
    Owner,
    /// The owner code used by the domain model.
    OwnerCode,
    /// The identifier of the associated owner.
    OwnerId,
    /// The owner type used by the domain model.
    OwnerType,
    /// The amount already paid.
    Paid,
    /// The identifier of the associated parent.
    ParentId,
    /// The account password.
    Password,
    /// The amount due for payment.
    Payable,
    /// The payee's account id for the payment.
    PayeeAccountId,
    /// The payee's altitude for the payment.
    PayeeAltitude,
    /// The payee's credential number for the payment.
    PayeeCredentialNumber,
    /// The payee's credential type for the payment.
    PayeeCredentialType,
    /// The payee's email for the payment.
    PayeeEmail,
    /// The payee's id for the payment.
    PayeeId,
    /// The payee's ip for the payment.
    PayeeIp,
    /// The payee's latitude for the payment.
    PayeeLatitude,
    /// The payee's longitude for the payment.
    PayeeLongitude,
    /// The payee's name for the payment.
    PayeeName,
    /// The payee's phone for the payment.
    PayeePhone,
    /// The payee's platform for the payment.
    PayeePlatform,
    /// The payee's type for the payment.
    PayeeType,
    /// The payer's account id for the payment.
    PayerAccountId,
    /// The payer's altitude for the payment.
    PayerAltitude,
    /// The payer's credential number for the payment.
    PayerCredentialNumber,
    /// The payer's credential type for the payment.
    PayerCredentialType,
    /// The payer's email for the payment.
    PayerEmail,
    /// The payer's id for the payment.
    PayerId,
    /// The payer's ip for the payment.
    PayerIp,
    /// The payer's latitude for the payment.
    PayerLatitude,
    /// The payer's longitude for the payment.
    PayerLongitude,
    /// The payer's name for the payment.
    PayerName,
    /// The payer's phone for the payment.
    PayerPhone,
    /// The payer's platform for the payment.
    PayerPlatform,
    /// The payer's type for the payment.
    PayerType,
    /// The time at which pay occurred.
    PayTime,
    /// The identifier of the associated person.
    PersonId,
    /// A telephone number.
    Phone,
    /// The telephone number's area.
    PhoneArea,
    /// The telephone number's city area.
    PhoneCityArea,
    /// The telephone number's country area.
    PhoneCountryArea,
    /// The telephone number's number.
    PhoneNumber,
    /// The telephone number's verify.
    PhoneVerify,
    /// The client platform.
    Platform,
    /// The postal code.
    Postalcode,
    /// Whether the record is predefined by the platform.
    Predefined,
    /// The identifier of the associated prescription.
    PrescriptionId,
    /// A monetary price.
    Price,
    /// A private cryptographic key.
    PrivateKey,
    /// The collection of granted privileges.
    Privileges,
    /// The associated catalog product.
    Product,
    /// The calendar date of production.
    ProductionDate,
    /// The catalog product's code.
    ProductCode,
    /// The catalog product's count.
    ProductCount,
    /// The catalog product's currency.
    ProductCurrency,
    /// The catalog product's description.
    ProductDescription,
    /// The catalog product's id.
    ProductId,
    /// The catalog product's image.
    ProductImage,
    /// The catalog product's item.
    ProductItem,
    /// The catalog product's item id.
    ProductItemId,
    /// The catalog product's name.
    ProductName,
    /// The catalog product's price.
    ProductPrice,
    /// The catalog product's quality.
    ProductQuality,
    /// The catalog product's specification.
    ProductSpecification,
    /// The catalog product's unit.
    ProductUnit,
    /// The provider app used by the domain model.
    ProviderApp,
    /// The identifier of the associated provider app.
    ProviderAppId,
    /// The associated province or state.
    Province,
    /// The identifier of the associated province.
    ProvinceId,
    /// The push-notification delivery token.
    PushToken,
    /// Whether the value is read-only.
    ReadOnly,
    /// The time at which refund occurred.
    RefundTime,
    /// The person's religion.
    Religion,
    /// The assigned authorization role.
    Role,
    /// An RSA public key.
    RsaPublicKey,
    /// The calendar date of sale end.
    SaleEndDate,
    /// The calendar date of sale start.
    SaleStartDate,
    /// The business scenario in which the operation occurs.
    Scene,
    /// A security key used to protect the operation.
    SecurityKey,
    /// The selling party.
    Seller,
    /// The identifier of the associated seller.
    SellerId,
    /// The person's sexual orientation.
    SexOrientation,
    /// The shipping cost used by the domain model.
    ShippingCost,
    /// The shipping demand used by the domain model.
    ShippingDemand,
    /// The shipping mode used by the domain model.
    ShippingMode,
    /// The shipping number used by the domain model.
    ShippingNumber,
    /// The time at which ship occurred.
    ShipTime,
    /// The social security city used by the domain model.
    SocialSecurityCity,
    /// The identifier of the associated social security city.
    SocialSecurityCityId,
    /// The originating source channel.
    Source,
    /// The identifier of the associated source.
    SourceId,
    /// The applicable specification.
    Specification,
    /// The lifecycle state.
    State,
    /// The current business status.
    Status,
    /// Whether the operation succeeded.
    Success,
    /// The identifier of the associated target.
    TargetId,
    /// The target type used by the domain model.
    TargetType,
    /// A timestamp supplied by the caller or system.
    Timestamp,
    /// A human-readable title.
    Title,
    /// An authentication or authorization token.
    Token,
    /// The time at which token create occurred.
    TokenCreateTime,
    /// The time at which token expired occurred.
    TokenExpiredTime,
    /// The total discount used by the domain model.
    TotalDiscount,
    /// The total price used by the domain model.
    TotalPrice,
    /// The identifier of the associated transaction.
    TransactionId,
    /// The applicable business type.
    Type,
    /// The device's unique identifier.
    Udid,
    /// The unit of measure.
    Unit,
    /// The identifier of the associated upload.
    UploadId,
    /// A web resource URL.
    Url,
    /// The associated user account.
    User,
    /// The account login name.
    Username,
    /// The identifier of the associated user.
    UserId,
    /// The time at which valid occurred.
    ValidTime,
    /// The associated value.
    Value,
    /// Whether verification succeeded.
    Verify,
    /// A one-time verification code.
    VerifyCode,
    /// The person's weight.
    Weight,
    /// The openid type used by the domain model.
    OpenidType,
    /// The third-party OpenID.
    Openid,
    /// A security or questionnaire prompt.
    Question,
    /// The identifier of the associated thirdparty insurance.
    ThirdpartyInsuranceId,
    /// The accident reason used by the domain model.
    AccidentReason,
    /// The policy number used by the domain model.
    PolicyNumber,
    /// The kinship type used by the domain model.
    KinshipType,
    /// The party submitting the insurance claim.
    Claimant,
    /// The calendar date of accident.
    AccidentDate,
    /// The accident place used by the domain model.
    AccidentPlace,
    /// The accident description used by the domain model.
    AccidentDescription,
    /// The associated payment account.
    Account,
    /// The account type used by the domain model.
    AccountType,
    /// The insured person.
    Insured,
    /// The identifier of the associated claim.
    ClaimId,
    /// The identifier of the associated claim medical.
    ClaimMedicalId,
    /// The identifier of the associated claim invoice.
    ClaimInvoiceId,
    /// The invoice number used by the domain model.
    InvoiceNumber,
    /// The calendar date of treatment.
    TreatmentDate,
    /// A monetary amount.
    Amount,
    /// The invoice list used by the domain model.
    InvoiceList,
    /// The fund paid amount used by the domain model.
    FundPaidAmount,
    /// The self paid amount used by the domain model.
    SelfPaidAmount,
    /// The self care amount used by the domain model.
    SelfCareAmount,
    /// The medicare amount used by the domain model.
    MedicareAmount,
    /// The serious illness paid used by the domain model.
    SeriousIllnessPaid,
    /// The serious illness insurance paid used by the domain model.
    SeriousIllnessInsurancePaid,
    /// The civil affair subsidy paid used by the domain model.
    CivilAffairSubsidyPaid,
    /// The message key used by the domain model.
    MessageKey,
    /// The primary medical diagnosis.
    PrimaryDiagnosis,
    /// The medical charge name used by the domain model.
    MedicalChargeName,
}

impl Field {
    /// Returns the stable lowercase code used in payloads and message keys.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::Action => "action",
            Self::Actual => "actual",
            Self::ActualCredentialNumber => "actual_credential_number",
            Self::ActualCredentialType => "actual_credential_type",
            Self::AddressDetail => "address_detail",
            Self::AddressLatitude => "address_latitude",
            Self::AddressLongitude => "address_longitude",
            Self::AddressPostalcode => "address_postalcode",
            Self::AddressStreetId => "address_street_id",
            Self::AddressVerify => "address_verify",
            Self::Age => "age",
            Self::Altitude => "altitude",
            Self::App => "app",
            Self::AppId => "app_id",
            Self::Avatar => "avatar",
            Self::Birthday => "birthday",
            Self::Blood => "blood",
            Self::Buyer => "buyer",
            Self::BuyerId => "buyer_id",
            Self::CategoryId => "category_id",
            Self::ChangePassword => "change_password",
            Self::Channel => "channel",
            Self::ChannelNumber => "channel_number",
            Self::City => "city",
            Self::CityId => "city_id",
            Self::Client => "client",
            Self::Code => "code",
            Self::Comment => "comment",
            Self::CompleteTime => "complete_time",
            Self::Cost => "cost",
            Self::Count => "count",
            Self::Country => "country",
            Self::CountryId => "country_id",
            Self::CreateTime => "create_time",
            Self::CreatorId => "creator_id",
            Self::Credential => "credential",
            Self::CredentialNumber => "credential_number",
            Self::CredentialType => "credential_type",
            Self::CredentialVerify => "credential_verify",
            Self::Currency => "currency",
            Self::DeleterId => "deleter_id",
            Self::DeleteTime => "delete_time",
            Self::Description => "description",
            Self::Dict => "dict",
            Self::Discount => "discount",
            Self::DiscountReason => "discount_reason",
            Self::District => "district",
            Self::DistrictId => "district_id",
            Self::Education => "education",
            Self::Email => "email",
            Self::EmailVerify => "email_verify",
            Self::Encrypted => "encrypted",
            Self::Entity => "entity",
            Self::ErrorCode => "error_code",
            Self::ErrorMessage => "error_message",
            Self::ErrorParams => "error_params",
            Self::ErrorType => "error_type",
            Self::Ethnic => "ethnic",
            Self::Expected => "expected",
            Self::ExpectedCredentialNumber => "expected_credential_number",
            Self::ExpectedCredentialType => "expected_credential_type",
            Self::ExpiredReason => "expired_reason",
            Self::ExpiredTime => "expired_time",
            Self::Fullname => "fullname",
            Self::Gender => "gender",
            Self::Guardian => "guardian",
            Self::HasChild => "has_child",
            Self::HasMedicare => "has_medicare",
            Self::HasMedicareOrSocialSecurity => "has_medicare_or_social_security",
            Self::HasSocialSecurity => "has_social_security",
            Self::Height => "height",
            Self::Host => "host",
            Self::Icon => "icon",
            Self::Id => "id",
            Self::Image => "image",
            Self::Incoming => "incoming",
            Self::Industry => "industry",
            Self::Inventory => "inventory",
            Self::InvoiceStatus => "invoice_status",
            Self::Ip => "ip",
            Self::ItemId => "item_id",
            Self::Job => "job",
            Self::JobTitle => "job_title",
            Self::LastActiveTime => "last_active_time",
            Self::LastLoginFailures => "last_login_failures",
            Self::LastLoginTime => "last_login_time",
            Self::Latitude => "latitude",
            Self::Level => "level",
            Self::LoginTime => "login_time",
            Self::Longitude => "longitude",
            Self::ManufacturerId => "manufacturer_id",
            Self::Marriage => "marriage",
            Self::MedicareCity => "medicare_city",
            Self::MedicareCityId => "medicare_city_id",
            Self::Mobile => "mobile",
            Self::MobileCityArea => "mobile_city_area",
            Self::MobileCountryArea => "mobile_country_area",
            Self::MobileNumber => "mobile_number",
            Self::MobileVerify => "mobile_verify",
            Self::Mode => "mode",
            Self::ModifierId => "modifier_id",
            Self::ModifyTime => "modify_time",
            Self::Multiple => "multiple",
            Self::Name => "name",
            Self::NeedDelivery => "need_delivery",
            Self::NewCredentialNumber => "new_credential_number",
            Self::NewCredentialType => "new_credential_type",
            Self::NewEmail => "new_email",
            Self::NewFullname => "new_fullname",
            Self::NewMobile => "new_mobile",
            Self::Nickname => "nickname",
            Self::Nullable => "nullable",
            Self::Number => "number",
            Self::OldCredentialNumber => "old_credential_number",
            Self::OldCredentialType => "old_credential_type",
            Self::OldEmail => "old_email",
            Self::OldFullname => "old_fullname",
            Self::OldMobile => "old_mobile",
            Self::Operation => "operation",
            Self::OperatorId => "operator_id",
            Self::OrderId => "order_id",
            Self::OrderItem => "order_item",
            Self::OrderItemId => "order_item_id",
            Self::OrderTitle => "order_title",
            Self::Organization => "organization",
            Self::OrganizationId => "organization_id",
            Self::OrganizationName => "organization_name",
            Self::OriginId => "origin_id",
            Self::Owner => "owner",
            Self::OwnerCode => "owner_code",
            Self::OwnerId => "owner_id",
            Self::OwnerType => "owner_type",
            Self::Paid => "paid",
            Self::ParentId => "parent_id",
            Self::Password => "password",
            Self::Payable => "payable",
            Self::PayeeAccountId => "payee_account_id",
            Self::PayeeAltitude => "payee_altitude",
            Self::PayeeCredentialNumber => "payee_credential_number",
            Self::PayeeCredentialType => "payee_credential_type",
            Self::PayeeEmail => "payee_email",
            Self::PayeeId => "payee_id",
            Self::PayeeIp => "payee_ip",
            Self::PayeeLatitude => "payee_latitude",
            Self::PayeeLongitude => "payee_longitude",
            Self::PayeeName => "payee_name",
            Self::PayeePhone => "payee_phone",
            Self::PayeePlatform => "payee_platform",
            Self::PayeeType => "payee_type",
            Self::PayerAccountId => "payer_account_id",
            Self::PayerAltitude => "payer_altitude",
            Self::PayerCredentialNumber => "payer_credential_number",
            Self::PayerCredentialType => "payer_credential_type",
            Self::PayerEmail => "payer_email",
            Self::PayerId => "payer_id",
            Self::PayerIp => "payer_ip",
            Self::PayerLatitude => "payer_latitude",
            Self::PayerLongitude => "payer_longitude",
            Self::PayerName => "payer_name",
            Self::PayerPhone => "payer_phone",
            Self::PayerPlatform => "payer_platform",
            Self::PayerType => "payer_type",
            Self::PayTime => "pay_time",
            Self::PersonId => "person_id",
            Self::Phone => "phone",
            Self::PhoneArea => "phone_area",
            Self::PhoneCityArea => "phone_city_area",
            Self::PhoneCountryArea => "phone_country_area",
            Self::PhoneNumber => "phone_number",
            Self::PhoneVerify => "phone_verify",
            Self::Platform => "platform",
            Self::Postalcode => "postalcode",
            Self::Predefined => "predefined",
            Self::PrescriptionId => "prescription_id",
            Self::Price => "price",
            Self::PrivateKey => "private_key",
            Self::Privileges => "privileges",
            Self::Product => "product",
            Self::ProductionDate => "production_date",
            Self::ProductCode => "product_code",
            Self::ProductCount => "product_count",
            Self::ProductCurrency => "product_currency",
            Self::ProductDescription => "product_description",
            Self::ProductId => "product_id",
            Self::ProductImage => "product_image",
            Self::ProductItem => "product_item",
            Self::ProductItemId => "product_item_id",
            Self::ProductName => "product_name",
            Self::ProductPrice => "product_price",
            Self::ProductQuality => "product_quality",
            Self::ProductSpecification => "product_specification",
            Self::ProductUnit => "product_unit",
            Self::ProviderApp => "provider_app",
            Self::ProviderAppId => "provider_app_id",
            Self::Province => "province",
            Self::ProvinceId => "province_id",
            Self::PushToken => "push_token",
            Self::ReadOnly => "read_only",
            Self::RefundTime => "refund_time",
            Self::Religion => "religion",
            Self::Role => "role",
            Self::RsaPublicKey => "rsa_public_key",
            Self::SaleEndDate => "sale_end_date",
            Self::SaleStartDate => "sale_start_date",
            Self::Scene => "scene",
            Self::SecurityKey => "security_key",
            Self::Seller => "seller",
            Self::SellerId => "seller_id",
            Self::SexOrientation => "sex_orientation",
            Self::ShippingCost => "shipping_cost",
            Self::ShippingDemand => "shipping_demand",
            Self::ShippingMode => "shipping_mode",
            Self::ShippingNumber => "shipping_number",
            Self::ShipTime => "ship_time",
            Self::SocialSecurityCity => "social_security_city",
            Self::SocialSecurityCityId => "social_security_city_id",
            Self::Source => "source",
            Self::SourceId => "source_id",
            Self::Specification => "specification",
            Self::State => "state",
            Self::Status => "status",
            Self::Success => "success",
            Self::TargetId => "target_id",
            Self::TargetType => "target_type",
            Self::Timestamp => "timestamp",
            Self::Title => "title",
            Self::Token => "token",
            Self::TokenCreateTime => "token_create_time",
            Self::TokenExpiredTime => "token_expired_time",
            Self::TotalDiscount => "total_discount",
            Self::TotalPrice => "total_price",
            Self::TransactionId => "transaction_id",
            Self::Type => "type",
            Self::Udid => "udid",
            Self::Unit => "unit",
            Self::UploadId => "upload_id",
            Self::Url => "url",
            Self::User => "user",
            Self::Username => "username",
            Self::UserId => "user_id",
            Self::ValidTime => "valid_time",
            Self::Value => "value",
            Self::Verify => "verify",
            Self::VerifyCode => "verify_code",
            Self::Weight => "weight",
            Self::OpenidType => "openid_type",
            Self::Openid => "openid",
            Self::Question => "question",
            Self::ThirdpartyInsuranceId => "thirdparty_insurance_id",
            Self::AccidentReason => "accident_reason",
            Self::PolicyNumber => "policy_number",
            Self::KinshipType => "kinship_type",
            Self::Claimant => "claimant",
            Self::AccidentDate => "accident_date",
            Self::AccidentPlace => "accident_place",
            Self::AccidentDescription => "accident_description",
            Self::Account => "account",
            Self::AccountType => "account_type",
            Self::Insured => "insured",
            Self::ClaimId => "claim_id",
            Self::ClaimMedicalId => "claim_medical_id",
            Self::ClaimInvoiceId => "claim_invoice_id",
            Self::InvoiceNumber => "invoice_number",
            Self::TreatmentDate => "treatment_date",
            Self::Amount => "amount",
            Self::InvoiceList => "invoice_list",
            Self::FundPaidAmount => "fund_paid_amount",
            Self::SelfPaidAmount => "self_paid_amount",
            Self::SelfCareAmount => "self_care_amount",
            Self::MedicareAmount => "medicare_amount",
            Self::SeriousIllnessPaid => "serious_illness_paid",
            Self::SeriousIllnessInsurancePaid => "serious_illness_insurance_paid",
            Self::CivilAffairSubsidyPaid => "civil_affair_subsidy_paid",
            Self::MessageKey => "message_key",
            Self::PrimaryDiagnosis => "primary_diagnosis",
            Self::MedicalChargeName => "medical_charge_name",
        }
    }
}
