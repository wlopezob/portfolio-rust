use crate::models::LogEntry;

/// Trait for formatting log entries
pub trait LogFormatter {
    fn format(&self, logs: &[LogEntry]) -> String;
}

/// Text formatter for log entries (human-readable)
pub struct TextLogFormatter;

impl TextLogFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TextLogFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl LogFormatter for TextLogFormatter {
    fn format(&self, logs: &[LogEntry]) -> String {
        if logs.is_empty() {
            return "No logs found matching the criteria.".to_string();
        }

        let mut output = format!("Found {} log entries:\n\n", logs.len());

        for (i, log) in logs.iter().enumerate() {
            output.push_str(&format!("=== Log Entry {} ===\n", i + 1));

            if let Some(timestamp) = &log.timestamp {
                output.push_str(&format!("Timestamp: {}\n", timestamp));
            }

            if let Some(severity) = &log.severity {
                output.push_str(&format!("Severity: {}\n", severity));
            }

            if let Some(log_name) = &log.log_name {
                output.push_str(&format!("Log Name: {}\n", log_name));
            }

            if let Some(resource) = &log.resource {
                if let Some(resource_type) = &resource.resource_type {
                    output.push_str(&format!("Resource Type: {}\n", resource_type));
                }
                if let Some(labels) = &resource.labels {
                    output.push_str("Resource Labels:\n");
                    for (key, value) in labels {
                        output.push_str(&format!("  {}: {}\n", key, value));
                    }
                }
            }

            if let Some(text_payload) = &log.text_payload {
                output.push_str(&format!("Text Payload: {}\n", text_payload));
            }

            if let Some(json_payload) = &log.json_payload {
                output.push_str(&format!(
                    "JSON Payload:\n{}\n",
                    serde_json::to_string_pretty(json_payload)
                        .unwrap_or_else(|_| json_payload.to_string())
                ));
            }

            output.push_str("\n");
        }

        output
    }
}

/// JSON formatter for log entries (machine-readable)
pub struct JsonLogFormatter;

impl JsonLogFormatter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for JsonLogFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl LogFormatter for JsonLogFormatter {
    fn format(&self, logs: &[LogEntry]) -> String {
        serde_json::to_string_pretty(logs).unwrap_or_else(|_| "[]".to_string())
    }
}
