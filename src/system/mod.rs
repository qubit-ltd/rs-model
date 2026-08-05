//! System-level shared values.

use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};

/// Client operating-system platform.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Platform {
    Ios,
    IpadOs,
    Android,
    WindowsPhone,
    Windows,
    Linux,
    Mac,
    Web,
    Unknown,
}
