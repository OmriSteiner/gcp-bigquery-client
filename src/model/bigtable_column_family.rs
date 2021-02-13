use serde::{Serialize, Deserialize};
use crate::model::bigtable_column::BigtableColumn;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BigtableColumnFamily {
    /// [Optional] Lists of columns that should be exposed as individual fields as opposed to a list of (column name, value) pairs. All columns whose qualifier matches a qualifier in this list can be accessed as .. Other columns can be accessed as a list through .Column field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<BigtableColumn>>,
    /// [Optional] The encoding of the values when the type is not STRING. Acceptable encoding values are: TEXT - indicates values are alphanumeric text strings. BINARY - indicates values are encoded using HBase Bytes.toBytes family of functions. This can be overridden for a specific column by listing that column in 'columns' and specifying an encoding for it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// Identifier of the column family.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_id: Option<String>,
    /// [Optional] If this is set only the latest version of value are exposed for all columns in this column family. This can be overridden for a specific column by listing that column in 'columns' and specifying a different setting for that column.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_read_latest: Option<bool>,
    /// [Optional] The type to convert the value in cells of this column family. The values are expected to be encoded using HBase Bytes.toBytes function when using the BINARY encoding value. Following BigQuery types are allowed (case-sensitive) - BYTES STRING INTEGER FLOAT BOOLEAN Default type is BYTES. This can be overridden for a specific column by listing that column in 'columns' and specifying a type for it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}