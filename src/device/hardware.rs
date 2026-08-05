use crate::device::SimCard;
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct Hardware {
    #[model(identifier)]
    pub id: Option<i64>,
    pub name: Option<String>,
    pub model: Option<String>,
    pub brand: Option<String>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub firmware: Option<String>,
    pub board: Option<String>,
    pub hardware: Option<String>,
    pub supported_abis: Vec<String>,
    pub ethernet_mac_addresses: Vec<String>,
    pub wifi_mac_addresses: Vec<String>,
    pub sim_cards: Vec<SimCard>,
    #[redact(level = "secret")]
    pub serial: Option<String>,
    #[redact(level = "secret")]
    pub udid: Option<String>,
}
