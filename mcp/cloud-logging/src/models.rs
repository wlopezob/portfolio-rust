use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub log_name: Option<String>,
    pub resource: Option<Resource>,
    pub timestamp: Option<String>,
    pub receive_timestamp: Option<String>,
    pub severity: Option<String>,
    pub insert_id: Option<String>,
    pub http_request: Option<serde_json::Value>,
    pub labels: Option<HashMap<String, String>>,
    pub metadata: Option<serde_json::Value>,
    pub operation: Option<serde_json::Value>,
    pub trace: Option<String>,
    pub span_id: Option<String>,
    pub trace_sampled: Option<bool>,
    pub source_location: Option<serde_json::Value>,
    pub split: Option<serde_json::Value>,

    // Payload fields (one of these will be set)
    pub text_payload: Option<String>,
    pub json_payload: Option<serde_json::Value>,
    pub proto_payload: Option<serde_json::Value>,
}

/// Resource information for log entries
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    #[serde(rename = "type")]
    pub resource_type: Option<String>,
    pub labels: Option<HashMap<String, String>>,
}
