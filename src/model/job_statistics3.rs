use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatistics3 {
    /// [Output-only] The number of bad records encountered. Note that if the job has failed because of more bad records encountered than the maximum allowed in the load job configuration, then this number can be less than the total number of bad records present in the input data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bad_records: Option<String>,
    /// [Output-only] Number of bytes of source data in a load job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_file_bytes: Option<String>,
    /// [Output-only] Number of source files in a load job.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_files: Option<String>,
    /// [Output-only] Size of the loaded data in bytes. Note that while a load job is in the running state, this value may change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_bytes: Option<String>,
    /// [Output-only] Number of rows imported in a load job. Note that while an import job is in the running state, this value may change.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_rows: Option<String>,
}
