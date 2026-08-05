use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Model, PartialEq, Eq, Serialize)]
pub struct DictEntryInfo {
    #[model(identifier)]
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    #[model(identifier)]
    pub dict_id: Option<i64>,
    pub params: Vec<String>,
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
