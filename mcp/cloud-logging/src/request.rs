use rmcp::schemars;

/// Parameters for the list_logs tool
#[derive(serde::Deserialize, schemars::JsonSchema, Clone)]
pub struct ListLogsRequest {
    /// Google Cloud project ID
    pub project_id: String,

    /// Filter by resource type (e.g., k8s_node, k8s_pod, gce_instance)
    #[serde(default)]
    pub resource_type: Option<String>,

    /// Filter by log name (uses substring match)
    #[serde(default)]
    pub log_name: Option<String>,

    /// Filter by Kubernetes namespace (in jsonPayload.src.namespace)
    #[serde(default)]
    pub namespace: Option<String>,

    /// Filter by pod name (uses substring match in jsonPayload.src.pod_name)
    #[serde(default)]
    pub pod_name: Option<String>,

    /// Filter by severity (e.g., ERROR, WARNING, INFO, DEBUG)
    #[serde(default)]
    pub severity: Option<String>,

    /// Start time in RFC3339 format (e.g., "2026-01-15T00:00:00Z") or relative (e.g., "1h", "2d")
    #[serde(default)]
    pub since: Option<String>,

    /// End time in RFC3339 format (e.g., "2026-01-15T23:59:59Z")
    #[serde(default)]
    pub until: Option<String>,

    /// Maximum number of logs to return (default: 20, max: 1000)
    #[serde(default = "default_limit")]
    pub limit: u32,

    /// Sort order: "asc" for ascending (oldest first), "desc" for descending (newest first, default)
    #[serde(default = "default_order")]
    pub order: String,
}

fn default_limit() -> u32 {
    20
}

fn default_order() -> String {
    "desc".to_string()
}

impl ListLogsRequest {
    /// Get the normalized sort order (either "asc" or "desc")
    pub fn normalized_order(&self) -> &str {
        match self.order.to_lowercase().as_str() {
            "asc" | "ascending" => "asc",
            _ => "desc",
        }
    }
    
    /// Get the effective limit (capped at reasonable maximum)
    pub fn effective_limit(&self) -> u32 {
        self.limit.min(100)
    }
}
