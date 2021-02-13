use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionProperty {
    /// [Required] Name of the connection property to set.
    pub key: String,
    /// [Required] Value of the connection property.
    pub value: String,
}