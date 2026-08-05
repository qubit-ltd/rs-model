//! Products participating in marketing activities.

use chrono::{DateTime, Utc};
use qubit_model_derive::Model;
use qubit_redact_derive::Redact;
use serde::{Deserialize, Serialize};

use crate::product::ProductInfo;

/// One indexed product entry within an activity.
#[derive(Clone, Debug, Deserialize, Model, PartialEq, Redact, Serialize)]
pub struct ActivityProductItem {
    /// Identifier of the owning activity.
    #[model(identifier)]
    pub activity_id: i64,
    /// Zero-based position within the activity's product list.
    pub index: i32,
    /// Product snapshot.
    pub product: ProductInfo,
    /// UTC creation timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub create_time: DateTime<Utc>,
    /// Optional UTC soft-deletion timestamp.
    #[model(time(precision = second, normalization = utc))]
    pub delete_time: Option<DateTime<Utc>>,
}
