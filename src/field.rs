// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Domain field identifiers.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;
use qubit_redact_derive::Redact;

/// Identifies a field that can appear in a model or validation message.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Redact, Serialize,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Field {
    /// Source field `ACTION`.
    Action,
    /// Source field `ACTUAL`.
    Actual,
    /// Source field `ACTUAL_CREDENTIAL_NUMBER`.
    ActualCredentialNumber,
    /// Source field `ACTUAL_CREDENTIAL_TYPE`.
    ActualCredentialType,
    /// Source field `ADDRESS_DETAIL`.
    AddressDetail,
    /// Source field `ADDRESS_LATITUDE`.
    AddressLatitude,
    /// Source field `ADDRESS_LONGITUDE`.
    AddressLongitude,
    /// Source field `ADDRESS_POSTALCODE`.
    AddressPostalcode,
    /// Source field `ADDRESS_STREET_ID`.
    AddressStreetId,
    /// Source field `ADDRESS_VERIFY`.
    AddressVerify,
    /// Source field `AGE`.
    Age,
    /// Source field `ALTITUDE`.
    Altitude,
    /// Source field `APP`.
    App,
    /// Source field `APP_ID`.
    AppId,
    /// Source field `AVATAR`.
    Avatar,
    /// Source field `BIRTHDAY`.
    Birthday,
    /// Source field `BLOOD`.
    Blood,
    /// Source field `BUYER`.
    Buyer,
    /// Source field `BUYER_ID`.
    BuyerId,
    /// Source field `CATEGORY_ID`.
    CategoryId,
    /// Source field `CHANGE_PASSWORD`.
    ChangePassword,
    /// Source field `CHANNEL`.
    Channel,
    /// Source field `CHANNEL_NUMBER`.
    ChannelNumber,
    /// Source field `CITY`.
    City,
    /// Source field `CITY_ID`.
    CityId,
    /// Source field `CLIENT`.
    Client,
    /// Source field `CODE`.
    Code,
    /// Source field `COMMENT`.
    Comment,
    /// Source field `COMPLETE_TIME`.
    CompleteTime,
    /// Source field `COST`.
    Cost,
    /// Source field `COUNT`.
    Count,
    /// Source field `COUNTRY`.
    Country,
    /// Source field `COUNTRY_ID`.
    CountryId,
    /// Source field `CREATE_TIME`.
    CreateTime,
    /// Source field `CREATOR_ID`.
    CreatorId,
    /// Source field `CREDENTIAL`.
    Credential,
    /// Source field `CREDENTIAL_NUMBER`.
    CredentialNumber,
    /// Source field `CREDENTIAL_TYPE`.
    CredentialType,
    /// Source field `CREDENTIAL_VERIFY`.
    CredentialVerify,
    /// Source field `CURRENCY`.
    Currency,
    /// Source field `DELETER_ID`.
    DeleterId,
    /// Source field `DELETE_TIME`.
    DeleteTime,
    /// Source field `DESCRIPTION`.
    Description,
    /// Source field `DICT`.
    Dict,
    /// Source field `DISCOUNT`.
    Discount,
    /// Source field `DISCOUNT_REASON`.
    DiscountReason,
    /// Source field `DISTRICT`.
    District,
    /// Source field `DISTRICT_ID`.
    DistrictId,
    /// Source field `EDUCATION`.
    Education,
    /// Source field `EMAIL`.
    Email,
    /// Source field `EMAIL_VERIFY`.
    EmailVerify,
    /// Source field `ENCRYPTED`.
    Encrypted,
    /// Source field `ENTITY`.
    Entity,
    /// Source field `ERROR_CODE`.
    ErrorCode,
    /// Source field `ERROR_MESSAGE`.
    ErrorMessage,
    /// Source field `ERROR_PARAMS`.
    ErrorParams,
    /// Source field `ERROR_TYPE`.
    ErrorType,
    /// Source field `ETHNIC`.
    Ethnic,
    /// Source field `EXPECTED`.
    Expected,
    /// Source field `EXPECTED_CREDENTIAL_NUMBER`.
    ExpectedCredentialNumber,
    /// Source field `EXPECTED_CREDENTIAL_TYPE`.
    ExpectedCredentialType,
    /// Source field `EXPIRED_REASON`.
    ExpiredReason,
    /// Source field `EXPIRED_TIME`.
    ExpiredTime,
    /// Source field `FULLNAME`.
    Fullname,
    /// Source field `GENDER`.
    Gender,
    /// Source field `GUARDIAN`.
    Guardian,
    /// Source field `HAS_CHILD`.
    HasChild,
    /// Source field `HAS_MEDICARE`.
    HasMedicare,
    /// Source field `HAS_MEDICARE_OR_SOCIAL_SECURITY`.
    HasMedicareOrSocialSecurity,
    /// Source field `HAS_SOCIAL_SECURITY`.
    HasSocialSecurity,
    /// Source field `HEIGHT`.
    Height,
    /// Source field `HOST`.
    Host,
    /// Source field `ICON`.
    Icon,
    /// Source field `ID`.
    Id,
    /// Source field `IMAGE`.
    Image,
    /// Source field `INCOMING`.
    Incoming,
    /// Source field `INDUSTRY`.
    Industry,
    /// Source field `INVENTORY`.
    Inventory,
    /// Source field `INVOICE_STATUS`.
    InvoiceStatus,
    /// Source field `IP`.
    Ip,
    /// Source field `ITEM_ID`.
    ItemId,
    /// Source field `JOB`.
    Job,
    /// Source field `JOB_TITLE`.
    JobTitle,
    /// Source field `LAST_ACTIVE_TIME`.
    LastActiveTime,
    /// Source field `LAST_LOGIN_FAILURES`.
    LastLoginFailures,
    /// Source field `LAST_LOGIN_TIME`.
    LastLoginTime,
    /// Source field `LATITUDE`.
    Latitude,
    /// Source field `LEVEL`.
    Level,
    /// Source field `LOGIN_TIME`.
    LoginTime,
    /// Source field `LONGITUDE`.
    Longitude,
    /// Source field `MANUFACTURER_ID`.
    ManufacturerId,
    /// Source field `MARRIAGE`.
    Marriage,
    /// Source field `MEDICARE_CITY`.
    MedicareCity,
    /// Source field `MEDICARE_CITY_ID`.
    MedicareCityId,
    /// Source field `MOBILE`.
    Mobile,
    /// Source field `MOBILE_CITY_AREA`.
    MobileCityArea,
    /// Source field `MOBILE_COUNTRY_AREA`.
    MobileCountryArea,
    /// Source field `MOBILE_NUMBER`.
    MobileNumber,
    /// Source field `MOBILE_VERIFY`.
    MobileVerify,
    /// Source field `MODE`.
    Mode,
    /// Source field `MODIFIER_ID`.
    ModifierId,
    /// Source field `MODIFY_TIME`.
    ModifyTime,
    /// Source field `MULTIPLE`.
    Multiple,
    /// Source field `NAME`.
    Name,
    /// Source field `NEED_DELIVERY`.
    NeedDelivery,
    /// Source field `NEW_CREDENTIAL_NUMBER`.
    NewCredentialNumber,
    /// Source field `NEW_CREDENTIAL_TYPE`.
    NewCredentialType,
    /// Source field `NEW_EMAIL`.
    NewEmail,
    /// Source field `NEW_FULLNAME`.
    NewFullname,
    /// Source field `NEW_MOBILE`.
    NewMobile,
    /// Source field `NICKNAME`.
    Nickname,
    /// Source field `NULLABLE`.
    Nullable,
    /// Source field `NUMBER`.
    Number,
    /// Source field `OLD_CREDENTIAL_NUMBER`.
    OldCredentialNumber,
    /// Source field `OLD_CREDENTIAL_TYPE`.
    OldCredentialType,
    /// Source field `OLD_EMAIL`.
    OldEmail,
    /// Source field `OLD_FULLNAME`.
    OldFullname,
    /// Source field `OLD_MOBILE`.
    OldMobile,
    /// Source field `OPERATION`.
    Operation,
    /// Source field `OPERATOR_ID`.
    OperatorId,
    /// Source field `ORDER_ID`.
    OrderId,
    /// Source field `ORDER_ITEM`.
    OrderItem,
    /// Source field `ORDER_ITEM_ID`.
    OrderItemId,
    /// Source field `ORDER_TITLE`.
    OrderTitle,
    /// Source field `ORGANIZATION`.
    Organization,
    /// Source field `ORGANIZATION_ID`.
    OrganizationId,
    /// Source field `ORGANIZATION_NAME`.
    OrganizationName,
    /// Source field `ORIGIN_ID`.
    OriginId,
    /// Source field `OWNER`.
    Owner,
    /// Source field `OWNER_CODE`.
    OwnerCode,
    /// Source field `OWNER_ID`.
    OwnerId,
    /// Source field `OWNER_TYPE`.
    OwnerType,
    /// Source field `PAID`.
    Paid,
    /// Source field `PARENT_ID`.
    ParentId,
    /// Source field `PASSWORD`.
    Password,
    /// Source field `PAYABLE`.
    Payable,
    /// Source field `PAYEE_ACCOUNT_ID`.
    PayeeAccountId,
    /// Source field `PAYEE_ALTITUDE`.
    PayeeAltitude,
    /// Source field `PAYEE_CREDENTIAL_NUMBER`.
    PayeeCredentialNumber,
    /// Source field `PAYEE_CREDENTIAL_TYPE`.
    PayeeCredentialType,
    /// Source field `PAYEE_EMAIL`.
    PayeeEmail,
    /// Source field `PAYEE_ID`.
    PayeeId,
    /// Source field `PAYEE_IP`.
    PayeeIp,
    /// Source field `PAYEE_LATITUDE`.
    PayeeLatitude,
    /// Source field `PAYEE_LONGITUDE`.
    PayeeLongitude,
    /// Source field `PAYEE_NAME`.
    PayeeName,
    /// Source field `PAYEE_PHONE`.
    PayeePhone,
    /// Source field `PAYEE_PLATFORM`.
    PayeePlatform,
    /// Source field `PAYEE_TYPE`.
    PayeeType,
    /// Source field `PAYER_ACCOUNT_ID`.
    PayerAccountId,
    /// Source field `PAYER_ALTITUDE`.
    PayerAltitude,
    /// Source field `PAYER_CREDENTIAL_NUMBER`.
    PayerCredentialNumber,
    /// Source field `PAYER_CREDENTIAL_TYPE`.
    PayerCredentialType,
    /// Source field `PAYER_EMAIL`.
    PayerEmail,
    /// Source field `PAYER_ID`.
    PayerId,
    /// Source field `PAYER_IP`.
    PayerIp,
    /// Source field `PAYER_LATITUDE`.
    PayerLatitude,
    /// Source field `PAYER_LONGITUDE`.
    PayerLongitude,
    /// Source field `PAYER_NAME`.
    PayerName,
    /// Source field `PAYER_PHONE`.
    PayerPhone,
    /// Source field `PAYER_PLATFORM`.
    PayerPlatform,
    /// Source field `PAYER_TYPE`.
    PayerType,
    /// Source field `PAY_TIME`.
    PayTime,
    /// Source field `PERSON_ID`.
    PersonId,
    /// Source field `PHONE`.
    Phone,
    /// Source field `PHONE_AREA`.
    PhoneArea,
    /// Source field `PHONE_CITY_AREA`.
    PhoneCityArea,
    /// Source field `PHONE_COUNTRY_AREA`.
    PhoneCountryArea,
    /// Source field `PHONE_NUMBER`.
    PhoneNumber,
    /// Source field `PHONE_VERIFY`.
    PhoneVerify,
    /// Source field `PLATFORM`.
    Platform,
    /// Source field `POSTALCODE`.
    Postalcode,
    /// Source field `PREDEFINED`.
    Predefined,
    /// Source field `PRESCRIPTION_ID`.
    PrescriptionId,
    /// Source field `PRICE`.
    Price,
    /// Source field `PRIVATE_KEY`.
    PrivateKey,
    /// Source field `PRIVILEGES`.
    Privileges,
    /// Source field `PRODUCT`.
    Product,
    /// Source field `PRODUCTION_DATE`.
    ProductionDate,
    /// Source field `PRODUCT_CODE`.
    ProductCode,
    /// Source field `PRODUCT_COUNT`.
    ProductCount,
    /// Source field `PRODUCT_CURRENCY`.
    ProductCurrency,
    /// Source field `PRODUCT_DESCRIPTION`.
    ProductDescription,
    /// Source field `PRODUCT_ID`.
    ProductId,
    /// Source field `PRODUCT_IMAGE`.
    ProductImage,
    /// Source field `PRODUCT_ITEM`.
    ProductItem,
    /// Source field `PRODUCT_ITEM_ID`.
    ProductItemId,
    /// Source field `PRODUCT_NAME`.
    ProductName,
    /// Source field `PRODUCT_PRICE`.
    ProductPrice,
    /// Source field `PRODUCT_QUALITY`.
    ProductQuality,
    /// Source field `PRODUCT_SPECIFICATION`.
    ProductSpecification,
    /// Source field `PRODUCT_UNIT`.
    ProductUnit,
    /// Source field `PROVIDER_APP`.
    ProviderApp,
    /// Source field `PROVIDER_APP_ID`.
    ProviderAppId,
    /// Source field `PROVINCE`.
    Province,
    /// Source field `PROVINCE_ID`.
    ProvinceId,
    /// Source field `PUSH_TOKEN`.
    PushToken,
    /// Source field `READ_ONLY`.
    ReadOnly,
    /// Source field `REFUND_TIME`.
    RefundTime,
    /// Source field `RELIGION`.
    Religion,
    /// Source field `ROLE`.
    Role,
    /// Source field `RSA_PUBLIC_KEY`.
    RsaPublicKey,
    /// Source field `SALE_END_DATE`.
    SaleEndDate,
    /// Source field `SALE_START_DATE`.
    SaleStartDate,
    /// Source field `SCENE`.
    Scene,
    /// Source field `SECURITY_KEY`.
    SecurityKey,
    /// Source field `SELLER`.
    Seller,
    /// Source field `SELLER_ID`.
    SellerId,
    /// Source field `SEX_ORIENTATION`.
    SexOrientation,
    /// Source field `SHIPPING_COST`.
    ShippingCost,
    /// Source field `SHIPPING_DEMAND`.
    ShippingDemand,
    /// Source field `SHIPPING_MODE`.
    ShippingMode,
    /// Source field `SHIPPING_NUMBER`.
    ShippingNumber,
    /// Source field `SHIP_TIME`.
    ShipTime,
    /// Source field `SOCIAL_SECURITY_CITY`.
    SocialSecurityCity,
    /// Source field `SOCIAL_SECURITY_CITY_ID`.
    SocialSecurityCityId,
    /// Source field `SOURCE`.
    Source,
    /// Source field `SOURCE_ID`.
    SourceId,
    /// Source field `SPECIFICATION`.
    Specification,
    /// Source field `STATE`.
    State,
    /// Source field `STATUS`.
    Status,
    /// Source field `SUCCESS`.
    Success,
    /// Source field `TARGET_ID`.
    TargetId,
    /// Source field `TARGET_TYPE`.
    TargetType,
    /// Source field `TIMESTAMP`.
    Timestamp,
    /// Source field `TITLE`.
    Title,
    /// Source field `TOKEN`.
    Token,
    /// Source field `TOKEN_CREATE_TIME`.
    TokenCreateTime,
    /// Source field `TOKEN_EXPIRED_TIME`.
    TokenExpiredTime,
    /// Source field `TOTAL_DISCOUNT`.
    TotalDiscount,
    /// Source field `TOTAL_PRICE`.
    TotalPrice,
    /// Source field `TRANSACTION_ID`.
    TransactionId,
    /// Source field `TYPE`.
    Type,
    /// Source field `UDID`.
    Udid,
    /// Source field `UNIT`.
    Unit,
    /// Source field `UPLOAD_ID`.
    UploadId,
    /// Source field `URL`.
    Url,
    /// Source field `USER`.
    User,
    /// Source field `USERNAME`.
    Username,
    /// Source field `USER_ID`.
    UserId,
    /// Source field `VALID_TIME`.
    ValidTime,
    /// Source field `VALUE`.
    Value,
    /// Source field `VERIFY`.
    Verify,
    /// Source field `VERIFY_CODE`.
    VerifyCode,
    /// Source field `WEIGHT`.
    Weight,
    /// Source field `OPENID_TYPE`.
    OpenidType,
    /// Source field `OPENID`.
    Openid,
    /// Source field `QUESTION`.
    Question,
    /// Source field `THIRDPARTY_INSURANCE_ID`.
    ThirdpartyInsuranceId,
    /// Source field `ACCIDENT_REASON`.
    AccidentReason,
    /// Source field `POLICY_NUMBER`.
    PolicyNumber,
    /// Source field `KINSHIP_TYPE`.
    KinshipType,
    /// Source field `CLAIMANT`.
    Claimant,
    /// Source field `ACCIDENT_DATE`.
    AccidentDate,
    /// Source field `ACCIDENT_PLACE`.
    AccidentPlace,
    /// Source field `ACCIDENT_DESCRIPTION`.
    AccidentDescription,
    /// Source field `ACCOUNT`.
    Account,
    /// Source field `ACCOUNT_TYPE`.
    AccountType,
    /// Source field `INSURED`.
    Insured,
    /// Source field `CLAIM_ID`.
    ClaimId,
    /// Source field `CLAIM_MEDICAL_ID`.
    ClaimMedicalId,
    /// Source field `CLAIM_INVOICE_ID`.
    ClaimInvoiceId,
    /// Source field `INVOICE_NUMBER`.
    InvoiceNumber,
    /// Source field `TREATMENT_DATE`.
    TreatmentDate,
    /// Source field `AMOUNT`.
    Amount,
    /// Source field `INVOICE_LIST`.
    InvoiceList,
    /// Source field `FUND_PAID_AMOUNT`.
    FundPaidAmount,
    /// Source field `SELF_PAID_AMOUNT`.
    SelfPaidAmount,
    /// Source field `SELF_CARE_AMOUNT`.
    SelfCareAmount,
    /// Source field `MEDICARE_AMOUNT`.
    MedicareAmount,
    /// Source field `SERIOUS_ILLNESS_PAID`.
    SeriousIllnessPaid,
    /// Source field `SERIOUS_ILLNESS_INSURANCE_PAID`.
    SeriousIllnessInsurancePaid,
    /// Source field `CIVIL_AFFAIR_SUBSIDY_PAID`.
    CivilAffairSubsidyPaid,
    /// Source field `MESSAGE_KEY`.
    MessageKey,
    /// Source field `PRIMARY_DIAGNOSIS`.
    PrimaryDiagnosis,
    /// Source field `MEDICAL_CHARGE_NAME`.
    MedicalChargeName,
}

impl Field {
    /// Returns the stable lowercase source code for this field.
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
            Self::HasMedicareOrSocialSecurity => {
                "has_medicare_or_social_security"
            }
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
            Self::SeriousIllnessInsurancePaid => {
                "serious_illness_insurance_paid"
            }
            Self::CivilAffairSubsidyPaid => "civil_affair_subsidy_paid",
            Self::MessageKey => "message_key",
            Self::PrimaryDiagnosis => "primary_diagnosis",
            Self::MedicalChargeName => "medical_charge_name",
        }
    }
}
