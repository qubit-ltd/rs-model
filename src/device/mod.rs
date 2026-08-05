//! Device inventory and telemetry models.

#[allow(clippy::module_inception)]
mod device;
mod device_current_data;
mod device_info;
mod hardware;
mod sim_card;
mod software;

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

pub use device::Device;
pub use device_current_data::DeviceCurrentData;
pub use device_info::DeviceInfo;
pub use hardware::Hardware;
pub use sim_card::SimCard;
pub use software::Software;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeviceType {
    IotBox,
    RadarBedMonitor,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SimCardStatus {
    Unknown,
    Absent,
    PinRequired,
    PukRequired,
    NetworkLocked,
    Ready,
    NotReady,
    PermDisabled,
    CardIoError,
    CardRestricted,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataNetworkType {
    Gprs,
    Cdma,
    Edge,
    OneXRtt,
    Iden,
    Gsm,
    TdScdma,
    Cdma2000,
    EvdoA,
    Umts,
    Evdo0,
    Hsdpa,
    Hsupa,
    Hspa,
    EvdoB,
    Ehrpd,
    Hspap,
    Iwlan,
    Lte,
    Nr,
    Unknown,
}
impl DataNetworkType {
    #[must_use]
    pub const fn generation(self) -> i32 {
        match self {
            Self::Lte => 4,
            Self::Nr => 5,
            Self::Unknown => 0,
            Self::Gprs | Self::Cdma | Self::Edge | Self::OneXRtt | Self::Iden | Self::Gsm => 2,
            _ => 3,
        }
    }
}
