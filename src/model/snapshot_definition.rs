use serde::{Serialize, Deserialize};
use crate::model::table_reference::TableReference;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_table_reference: Option<TableReference>,
    /// [Required] The time at which the base table was snapshot.
    pub snapshot_time: String,
}
