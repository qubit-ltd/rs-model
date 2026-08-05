//! Invoice leaf classifications.
use qubit_model_derive::Model;
use serde::{Deserialize, Serialize};
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceApplyStatus {
    Submitted,
    Approved,
    Rejected,
    Cancelled,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStatus {
    NoInvoice,
    NotRequired,
    NotPrinted,
    Printed,
    Reprinted,
    Invalid,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceStockStatus {
    Unstocked,
    Stocked,
    Cancelled,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceTitleType {
    Person,
    Organization,
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Model, PartialEq, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvoiceType {
    Normal,
    ValueAdded,
}
