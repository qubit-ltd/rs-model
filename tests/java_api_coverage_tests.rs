// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_mixin::MixinError;
use qubit_mixin::normalize;
use qubit_model::commons::Dict;
use qubit_model::commons::FullDict;
use qubit_model::contact::AddressBuilder;
use qubit_model::system::Setting;
use qubit_model::system::SettingJsonDeserializer;
use qubit_model::system::SettingJsonSerializer;
use qubit_model::system::SettingRandomizer;
use qubit_model::system::SettingXmlAdapted;
use qubit_model::system::SettingXmlAdapter;
use qubit_model::system::VerifyCode;
use qubit_model::system::VerifyScene;

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
