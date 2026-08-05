use super::DictEntryInfo;
use crate::mixin::StatefulInfo;
use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct DictEntry {
    #[model(identifier)]
    pub id: Option<i64>,
    pub dict: StatefulInfo,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub comment: Option<String>,
    #[model(opaque)]
    pub parent: Option<Box<DictEntryInfo>>,
    #[model(time(precision=second,normalization=utc))]
    pub create_time: DateTime<Utc>,
    #[model(time(precision=second,normalization=utc))]
    pub modify_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
