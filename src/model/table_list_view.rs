use serde::{Serialize, Deserialize};

/// TableListView : Additional details for a view.

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableListView {
    /// True if view is defined in legacy SQL dialect, false if in standard SQL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_legacy_sql: Option<bool>,
}