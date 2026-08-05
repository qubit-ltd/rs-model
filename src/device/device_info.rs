use crate::{
    commons::State, contact::Location, device::DeviceType, mixin::StatefulInfo, person::PersonInfo,
};
use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Serialize)]
pub struct DeviceInfo {
    #[model(identifier)]
    pub id: Option<i64>,
    pub code: String,
    pub name: String,
    pub app: StatefulInfo,
    pub owner: Option<PersonInfo>,
    pub state: State,
    pub device_type: DeviceType,
    pub location: Option<Location>,
    pub test: bool,
    #[model(time(precision=second,normalization=utc))]
    pub binding_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    pub register_time: Option<DateTime<Utc>>,
    #[model(time(precision=second,normalization=utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
