use anyhow::Result;
use crate::request::ListLogsRequest;
use crate::time::TimeParser;

/// Builder for constructing GCP Logging filter strings
pub struct FilterBuilder<'a> {
    request: &'a ListLogsRequest,
    time_parser: TimeParser,
}

impl<'a> FilterBuilder<'a> {
    pub fn new(request: &'a ListLogsRequest) -> Self {
        Self {
            request,
            time_parser: TimeParser::new(),
        }
    }
    
    /// Build the complete filter string
    pub fn build(&self) -> Result<String> {
        let mut parts = Vec::new();

        // Add time range filters
        self.add_time_filters(&mut parts)?;
        
        // Add resource filters
        self.add_resource_filters(&mut parts);
        
        // Add severity filter
        self.add_severity_filter(&mut parts);
        
        // Add Kubernetes filters
        self.add_kubernetes_filters(&mut parts);

        Ok(parts.join(" AND "))
    }
    
    fn add_time_filters(&self, parts: &mut Vec<String>) -> Result<()> {
        // IMPORTANT: Always add time range filter to improve performance
        // If no time range is specified, default to last 24 hours
        if let Some(since) = &self.request.since {
            let timestamp = self.time_parser.parse(since)?;
            parts.push(format!("timestamp>=\"{}\"", timestamp));
        } else {
            // Default to last 24 hours if no since is provided
            let default_since = self.time_parser.parse("24h")?;
            eprintln!("[FILTER] No 'since' provided, defaulting to last 24h: {}", default_since);
            parts.push(format!("timestamp>=\"{}\"", default_since));
        }

        if let Some(until) = &self.request.until {
            let timestamp = self.time_parser.parse(until)?;
            parts.push(format!("timestamp<=\"{}\"", timestamp));
        }
        
        Ok(())
    }
    
    fn add_resource_filters(&self, parts: &mut Vec<String>) {
        // Resource type filter (indexed, fast)
        if let Some(rt) = &self.request.resource_type {
            parts.push(format!("resource.type=\"{}\"", rt));
        }

        // Log name filter (substring match)
        if let Some(ln) = &self.request.log_name {
            parts.push(format!("logName:\"{}\"", ln));
        }
    }
    
    fn add_severity_filter(&self, parts: &mut Vec<String>) {
        if let Some(sev) = &self.request.severity {
            parts.push(format!("severity>=\"{}\"", sev.to_uppercase()));
        }
    }
    
    fn add_kubernetes_filters(&self, parts: &mut Vec<String>) {
        // Kubernetes namespace filter
        if let Some(ns) = &self.request.namespace {
            parts.push(format!("jsonPayload.src.namespace=\"{}\"", ns));
        }

        // Pod name filter (substring match)
        if let Some(pn) = &self.request.pod_name {
            parts.push(format!("jsonPayload.src.pod_name:\"{}\"", pn));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_filter_with_all_fields() {
        let req = ListLogsRequest {
            project_id: "test-project".to_string(),
            resource_type: Some("k8s_pod".to_string()),
            log_name: Some("policy-action".to_string()),
            namespace: Some("default".to_string()),
            pod_name: Some("my-pod".to_string()),
            severity: Some("ERROR".to_string()),
            since: Some("1h".to_string()),
            until: None,
            limit: 50,
            order: "desc".to_string(),
        };
        
        let builder = FilterBuilder::new(&req);
        let filter = builder.build().unwrap();
        
        assert!(filter.contains("resource.type=\"k8s_pod\""));
        assert!(filter.contains("severity>=\"ERROR\""));
        assert!(filter.contains("logName:\"policy-action\""));
    }
}
