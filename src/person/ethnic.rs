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

/// Ethnic group recorded for a person, including China's recognized groups.
#[derive(Model, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Ethnic {
    /// Han Chinese.
    Han,
    /// Mongol.
    Mongol,
    /// Hui.
    Hui,
    /// Tibetan.
    Tibetan,
    /// Uyghur.
    Uyghur,
    /// Miao.
    Miao,
    /// Yi.
    Yi,
    /// Zhuang.
    Zhuang,
    /// Buyei.
    Buyei,
    /// Korean (Chaoxian).
    Chosen,
    /// Manchu.
    Man,
    /// Dong.
    Dong,
    /// Yao.
    Yao,
    /// Bai.
    Bai,
    /// Tujia.
    Tujia,
    /// Hani.
    Hani,
    /// Kazak.
    Kazak,
    /// Dai.
    Dai,
    /// Li.
    Li,
    /// Lisu.
    Lisu,
    /// Va.
    Va,
    /// She.
    She,
    /// Lahu.
    Lahu,
    /// Sui.
    Sui,
    /// Dongxiang.
    Dongxiang,
    /// Naxi.
    Naxi,
    /// Jingpo.
    Jingpo,
    /// Kirgiz.
    Kirgiz,
    /// Tu.
    Tu,
    /// Daur.
    Daur,
    /// Mulao.
    Mulao,
    /// Qiang.
    Qiang,
    /// Blang.
    Blang,
    /// Salar.
    Salar,
    /// Maonan.
    Maonan,
    /// Gelao.
    Gelao,
    /// Xibe.
    Xibe,
    /// Achang.
    Achang,
    /// Pumi.
    Pumi,
    /// Tajik.
    Tajik,
    /// Nu.
    Nu,
    /// Uzbek.
    Uzbek,
    /// Russian.
    Russ,
    /// Ewenki.
    Ewenki,
    /// De'ang.
    Deang,
    /// Bonan.
    Bonan,
    /// Yugur.
    Yugur,
    /// Gin.
    Gin,
    /// Tatar.
    Tatar,
    /// Derung.
    Derung,
    /// Oroqen.
    Oroqen,
    /// Hezhen.
    Hezhen,
    /// Monba.
    Monba,
    /// Lhoba.
    Lhoba,
    /// Jino.
    Jino,
    /// Gaoshan.
    Gaoshan,
    /// Person of a foreign nationality or ethnicity.
    Foreigner,
    /// Ethnic group not otherwise represented here.
    Other,
}
