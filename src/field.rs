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
    /// The credential number presented for comparison.
    ActualCredentialNumber,
    /// The type of credential presented for comparison.
    ActualCredentialType,
    /// The address's detail.
    AddressDetail,
    /// The address's latitude.
    AddressLatitude,
    /// The address's longitude.
    AddressLongitude,
    /// The postal code within the address.
    AddressPostalcode,
    /// The identifier of the address street.
    AddressStreetId,
    /// Whether the address has been verified.
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
    /// Whether the account password must be changed.
    ChangePassword,
    /// The originating business channel.
    Channel,
    /// The business number assigned by the originating channel.
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
    /// The time at which the workflow completed.
    CompleteTime,
    /// A cost amount.
    Cost,
    /// A quantity.
    Count,
    /// The associated country.
    Country,
    /// The identifier of the associated country.
    CountryId,
    /// The time at which the record was created.
    CreateTime,
    /// The identifier of the associated creator.
    CreatorId,
    /// An identity credential.
    Credential,
    /// The credential number.
    CredentialNumber,
    /// The kind of identity credential.
    CredentialType,
    /// Whether the credential has been verified.
    CredentialVerify,
    /// The currency of a monetary amount.
    Currency,
    /// The identifier of the associated deleter.
    DeleterId,
    /// The time at which the record was deleted.
    DeleteTime,
    /// A human-readable description.
    Description,
    /// The associated reference-data dictionary.
    Dict,
    /// A discount amount.
    Discount,
    /// The reason a discount was applied.
    DiscountReason,
    /// The associated district.
    District,
    /// The identifier of the associated district.
    DistrictId,
    /// The person's education level.
    Education,
    /// An email address.
    Email,
    /// Whether the email address has been verified.
    EmailVerify,
    /// Whether the value is encrypted.
    Encrypted,
    /// The referenced domain entity type.
    Entity,
    /// The machine-readable error code.
    ErrorCode,
    /// The human-readable error message.
    ErrorMessage,
    /// Parameters used to render the error message.
    ErrorParams,
    /// The broad category assigned to the error.
    ErrorType,
    /// The person's ethnic group.
    Ethnic,
    /// The value expected by validation.
    Expected,
    /// The credential number expected for comparison.
    ExpectedCredentialNumber,
    /// The type of credential expected for comparison.
    ExpectedCredentialType,
    /// The reason the record or credential expired.
    ExpiredReason,
    /// The time at which the record or credential expires.
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
    /// The current invoicing status.
    InvoiceStatus,
    /// The network IP address.
    Ip,
    /// The identifier of the associated item.
    ItemId,
    /// The person's job description.
    Job,
    /// The person’s occupational title.
    JobTitle,
    /// The most recent time the account was active.
    LastActiveTime,
    /// The number of consecutive failed login attempts.
    LastLoginFailures,
    /// The most recent successful login time.
    LastLoginTime,
    /// The geographic latitude.
    Latitude,
    /// The hierarchy level.
    Level,
    /// The time at which authentication succeeded.
    LoginTime,
    /// The geographic longitude.
    Longitude,
    /// The identifier of the associated manufacturer.
    ManufacturerId,
    /// The person's marital status.
    Marriage,
    /// The city administering the person’s Medicare coverage.
    MedicareCity,
    /// The identifier of the associated medicare city.
    MedicareCityId,
    /// A mobile telephone number.
    Mobile,
    /// The mobile number's city area.
    MobileCityArea,
    /// The mobile number's country area.
    MobileCountryArea,
    /// The subscriber number portion of a mobile telephone number.
    MobileNumber,
    /// Whether the mobile telephone number has been verified.
    MobileVerify,
    /// The selected operating mode.
    Mode,
    /// The identifier of the associated modifier.
    ModifierId,
    /// The time at which the record was last modified.
    ModifyTime,
    /// Whether multiple values are permitted.
    Multiple,
    /// The display name.
    Name,
    /// Whether the order requires delivery.
    NeedDelivery,
    /// The proposed credential number.
    NewCredentialNumber,
    /// The proposed credential type.
    NewCredentialType,
    /// The proposed email.
    NewEmail,
    /// The proposed replacement full name.
    NewFullname,
    /// The proposed replacement mobile number.
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
    /// The previous full name.
    OldFullname,
    /// The previous mobile number.
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
    /// The customer-facing order title.
    OrderTitle,
    /// The associated organization.
    Organization,
    /// The identifier of the associated organization.
    OrganizationId,
    /// The organization’s display name.
    OrganizationName,
    /// The identifier of the associated origin.
    OriginId,
    /// The owning party.
    Owner,
    /// The business code assigned to the owner.
    OwnerCode,
    /// The identifier of the associated owner.
    OwnerId,
    /// The kind of party that owns the record.
    OwnerType,
    /// The amount already paid.
    Paid,
    /// The identifier of the associated parent.
    ParentId,
    /// The account password.
    Password,
    /// The amount due for payment.
    Payable,
    /// The identifier of the receiving account held by the payment recipient.
    PayeeAccountId,
    /// The recipient's geographic altitude at the payment location.
    PayeeAltitude,
    /// The identity-credential number supplied for the payment recipient.
    PayeeCredentialNumber,
    /// The type of identity credential supplied for the payment recipient.
    PayeeCredentialType,
    /// The email address used to contact the payment recipient.
    PayeeEmail,
    /// The identifier of the party receiving the payment.
    PayeeId,
    /// The network IP address reported for the payment recipient.
    PayeeIp,
    /// The recipient's geographic latitude at the payment location.
    PayeeLatitude,
    /// The recipient's geographic longitude at the payment location.
    PayeeLongitude,
    /// The display name of the party receiving the payment.
    PayeeName,
    /// The telephone number used to contact the payment recipient.
    PayeePhone,
    /// The client platform reported by the payment recipient.
    PayeePlatform,
    /// The business classification of the payment recipient.
    PayeeType,
    /// The identifier of the funding account held by the payment sender.
    PayerAccountId,
    /// The sender's geographic altitude at the payment location.
    PayerAltitude,
    /// The identity-credential number supplied for the payment sender.
    PayerCredentialNumber,
    /// The type of identity credential supplied for the payment sender.
    PayerCredentialType,
    /// The email address used to contact the payment sender.
    PayerEmail,
    /// The identifier of the party making the payment.
    PayerId,
    /// The network IP address reported for the payment sender.
    PayerIp,
    /// The sender's geographic latitude at the payment location.
    PayerLatitude,
    /// The sender's geographic longitude at the payment location.
    PayerLongitude,
    /// The display name of the party making the payment.
    PayerName,
    /// The telephone number used to contact the payment sender.
    PayerPhone,
    /// The client platform reported by the payment sender.
    PayerPlatform,
    /// The business classification of the payment sender.
    PayerType,
    /// The time at which payment completed.
    PayTime,
    /// The identifier of the associated person.
    PersonId,
    /// A telephone number.
    Phone,
    /// The telephone number’s area or extension component.
    PhoneArea,
    /// The city-area component of the telephone number.
    PhoneCityArea,
    /// The country calling code of the telephone number.
    PhoneCountryArea,
    /// The local subscriber number.
    PhoneNumber,
    /// Whether the telephone number has been verified.
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
    /// The identifier of the catalog product.
    ProductId,
    /// The catalog product's image.
    ProductImage,
    /// The catalog product's item.
    ProductItem,
    /// The identifier of the catalog product item.
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
    /// The tenant application providing the service.
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
    /// The time at which the payment was refunded.
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
    /// The charge for shipping the order.
    ShippingCost,
    /// The customer’s delivery requirements.
    ShippingDemand,
    /// The selected shipping method.
    ShippingMode,
    /// The carrier tracking number.
    ShippingNumber,
    /// The time at which the order was dispatched.
    ShipTime,
    /// The city administering the person’s social-security coverage.
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
    /// The kind of resource targeted by the operation.
    TargetType,
    /// A timestamp supplied by the caller or system.
    Timestamp,
    /// A human-readable title.
    Title,
    /// An authentication or authorization token.
    Token,
    /// The time at which the token was issued.
    TokenCreateTime,
    /// The time at which the token expires.
    TokenExpiredTime,
    /// The sum of all discounts applied.
    TotalDiscount,
    /// The total price before or after discounts, as defined by the payload.
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
    /// The time from which the record or credential is valid.
    ValidTime,
    /// The associated value.
    Value,
    /// Whether verification succeeded.
    Verify,
    /// A one-time verification code.
    VerifyCode,
    /// The person's weight.
    Weight,
    /// The third-party platform that issued the OpenID.
    OpenidType,
    /// The third-party OpenID.
    Openid,
    /// A security or questionnaire prompt.
    Question,
    /// The identifier of the associated thirdparty insurance.
    ThirdpartyInsuranceId,
    /// The reported cause of the accident.
    AccidentReason,
    /// The insurance policy number.
    PolicyNumber,
    /// The claimant’s relationship to the insured person.
    KinshipType,
    /// The party submitting the insurance claim.
    Claimant,
    /// The calendar date of accident.
    AccidentDate,
    /// The location where the accident occurred.
    AccidentPlace,
    /// A narrative description of the accident.
    AccidentDescription,
    /// The associated payment account.
    Account,
    /// The category of payment account.
    AccountType,
    /// The insured person.
    Insured,
    /// The identifier of the associated claim.
    ClaimId,
    /// The identifier of the associated claim medical.
    ClaimMedicalId,
    /// The identifier of the associated claim invoice.
    ClaimInvoiceId,
    /// The issuer’s invoice number.
    InvoiceNumber,
    /// The calendar date of treatment.
    TreatmentDate,
    /// A monetary amount.
    Amount,
    /// The collection of invoices attached to the claim.
    InvoiceList,
    /// The amount paid by the relevant fund.
    FundPaidAmount,
    /// The amount paid directly by the insured person.
    SelfPaidAmount,
    /// The portion payable outside insurance coverage.
    SelfCareAmount,
    /// The amount covered by Medicare.
    MedicareAmount,
    /// The amount paid under serious-illness coverage.
    SeriousIllnessPaid,
    /// The amount paid by serious-illness insurance.
    SeriousIllnessInsurancePaid,
    /// The amount paid through a civil-affairs subsidy.
    CivilAffairSubsidyPaid,
    /// The localization key used to resolve a message.
    MessageKey,
    /// The primary medical diagnosis.
    PrimaryDiagnosis,
    /// The name of the billed medical charge.
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
