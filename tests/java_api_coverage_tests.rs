// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

#[allow(unused_imports)]
use qubit_mixin::{
    Auditable,
    Creatable,
    DataWithMaxAge,
    Deletable,
    Desensitizable,
    Emptyful,
    HasClock,
    HasInfo,
    HasInfoWithEntity,
    HasLogger,
    HasSpecificInfo,
    Identifiable,
    Info,
    InfoWithEntity,
    MixinError,
    Modifiable,
    NameBuilder,
    Normalizable,
    Predefinable,
    Validatable,
    WithBirthday,
    WithCode,
    WithComment,
    WithEmail,
    WithEntity,
    WithIndex,
    WithKey,
    WithName,
    WithPassword,
    WithSecurityKey,
    WithStatus,
    WithUdid,
    WithUsername,
    WithUuid,
    WithVisibility,
    normalize,
};
use qubit_model::{
    commons::{
        Dict,
        FullDict,
    },
    contact::AddressBuilder,
    system::{
        Setting,
        SettingJsonDeserializer,
        SettingJsonSerializer,
        SettingRandomizer,
        SettingXmlAdapted,
        SettingXmlAdapter,
        VerifyCode,
        VerifyScene,
    },
};

#[test]
fn test_package_compatibility_reexports_resolve() {
    let _ = core::mem::size_of::<AddressBuilder>();
    let _ = core::mem::size_of::<Dict>();
    let _ = core::mem::size_of::<FullDict>();
    let _ = core::mem::size_of::<Setting>();
    let _ = core::mem::size_of::<SettingJsonDeserializer>();
    let _ = core::mem::size_of::<SettingJsonSerializer>();
    let _ = core::mem::size_of::<SettingRandomizer>();
    let _ = core::mem::size_of::<SettingXmlAdapted>();
    let _ = core::mem::size_of::<SettingXmlAdapter>();
    let _ = core::mem::size_of::<VerifyCode>();
    let _ = core::mem::size_of::<VerifyScene>();
}

#[test]
fn test_common_mixin_semantic_type_mappings_resolve() {
    let _ = core::mem::size_of::<MixinError>();
    assert_eq!(
        normalize(Some(" value ".to_owned())),
        Some("value".to_owned())
    );
}
