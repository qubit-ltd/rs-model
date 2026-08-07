// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

//! Demographic and social classification values.

use serde::Deserialize;
use serde::Serialize;

use qubit_model_derive::Model;


/// Source-domain Ethnic classification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Ethnic {
    /// Source variant `HAN`.
    Han,
    /// Source variant `MONGOL`.
    Mongol,
    /// Source variant `HUI`.
    Hui,
    /// Source variant `TIBETAN`.
    Tibetan,
    /// Source variant `UYGHUR`.
    Uyghur,
    /// Source variant `MIAO`.
    Miao,
    /// Source variant `YI`.
    Yi,
    /// Source variant `ZHUANG`.
    Zhuang,
    /// Source variant `BUYEI`.
    Buyei,
    /// Source variant `CHOSEN`.
    Chosen,
    /// Source variant `MAN`.
    Man,
    /// Source variant `DONG`.
    Dong,
    /// Source variant `YAO`.
    Yao,
    /// Source variant `BAI`.
    Bai,
    /// Source variant `TUJIA`.
    Tujia,
    /// Source variant `HANI`.
    Hani,
    /// Source variant `KAZAK`.
    Kazak,
    /// Source variant `DAI`.
    Dai,
    /// Source variant `LI`.
    Li,
    /// Source variant `LISU`.
    Lisu,
    /// Source variant `VA`.
    Va,
    /// Source variant `SHE`.
    She,
    /// Source variant `LAHU`.
    Lahu,
    /// Source variant `SUI`.
    Sui,
    /// Source variant `DONGXIANG`.
    Dongxiang,
    /// Source variant `NAXI`.
    Naxi,
    /// Source variant `JINGPO`.
    Jingpo,
    /// Source variant `KIRGIZ`.
    Kirgiz,
    /// Source variant `TU`.
    Tu,
    /// Source variant `DAUR`.
    Daur,
    /// Source variant `MULAO`.
    Mulao,
    /// Source variant `QIANG`.
    Qiang,
    /// Source variant `BLANG`.
    Blang,
    /// Source variant `SALAR`.
    Salar,
    /// Source variant `MAONAN`.
    Maonan,
    /// Source variant `GELAO`.
    Gelao,
    /// Source variant `XIBE`.
    Xibe,
    /// Source variant `ACHANG`.
    Achang,
    /// Source variant `PUMI`.
    Pumi,
    /// Source variant `TAJIK`.
    Tajik,
    /// Source variant `NU`.
    Nu,
    /// Source variant `UZBEK`.
    Uzbek,
    /// Source variant `RUSS`.
    Russ,
    /// Source variant `EWENKI`.
    Ewenki,
    /// Source variant `DEANG`.
    Deang,
    /// Source variant `BONAN`.
    Bonan,
    /// Source variant `YUGUR`.
    Yugur,
    /// Source variant `GIN`.
    Gin,
    /// Source variant `TATAR`.
    Tatar,
    /// Source variant `DERUNG`.
    Derung,
    /// Source variant `OROQEN`.
    Oroqen,
    /// Source variant `HEZHEN`.
    Hezhen,
    /// Source variant `MONBA`.
    Monba,
    /// Source variant `LHOBA`.
    Lhoba,
    /// Source variant `JINO`.
    Jino,
    /// Source variant `GAOSHAN`.
    Gaoshan,
    /// Source variant `FOREIGNER`.
    Foreigner,
    /// Source variant `OTHER`.
    Other,
}
