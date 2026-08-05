use crate::contact::{Location, Phone};
use crate::device::{DataNetworkType, SimCardStatus};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct SimCard {
    #[model(identifier)]
    pub id: Option<i64>,
    #[redact(level = "secret")]
    pub iccid: String,
    #[redact(level = "secret")]
    pub imei: Option<String>,
    #[redact(level = "secret")]
    pub meid: Option<String>,
    pub phone: Option<Phone>,
    pub operator: Option<String>,
    pub country: Option<String>,
    pub location: Option<Location>,
    pub network_type: Option<DataNetworkType>,
    pub status: Option<SimCardStatus>,
}
