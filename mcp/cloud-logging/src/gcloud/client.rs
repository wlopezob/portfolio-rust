//! Google Cloud Logging client using gcloud CLI

use anyhow::Result;
use tokio::process::Command;
use crate::auth::AuthProvider;
use crate::models::LogEntry;

/// Trait for logging clients
#[async_trait::async_trait]
pub trait LoggingClient {
    /// Fetch logs from Google Cloud Logging
    async fn fetch_logs(
        &self,
        project_id: &str,
        filter: &str,
        limit: u32,
        order: &str,
    ) -> Result<Vec<LogEntry>>;
}

/// Google Cloud Logging client using gcloud CLI
pub struct GcloudClient<A: AuthProvider> {
    auth_provider: A,
}

impl<A: AuthProvider> GcloudClient<A> {
    pub fn new(auth_provider: A) -> Self {
        Self { auth_provider }
    }
    
    /// Build gcloud command arguments
    fn build_args(&self, project_id: &str, filter: &str, limit: u32, order: &str) -> Vec<String> {
        let mut args = vec![
            "logging".to_string(),
            "read".to_string(),
        ];
        
        // Only add filter if non-empty
        if !filter.is_empty() {
            args.push(filter.to_string());
        }
        
        args.extend(vec![
            "--project".to_string(),
            project_id.to_string(),
            "--limit".to_string(),
            limit.to_string(),
            "--order".to_string(),
            order.to_string(),
            "--format".to_string(),
            "json".to_string(),
        ]);
        
        args
    }
}

#[async_trait::async_trait]
impl<A: AuthProvider + Send + Sync> LoggingClient for GcloudClient<A> {
    async fn fetch_logs(
        &self,
        project_id: &str,
        filter: &str,
        limit: u32,
        order: &str,
    ) -> Result<Vec<LogEntry>> {
        // Verify authentication first
        self.auth_provider.verify().await?;

        let args = self.build_args(project_id, filter, limit, order);

        eprintln!("[GCLOUD] Executing: gcloud {}", args.join(" "));
        eprintln!("[GCLOUD] This may take a while depending on the time range and filters...");
        let start_time = std::time::Instant::now();
        
        // Execute without timeout - let it run as long as needed
        let output = Command::new("gcloud")
            .args(&args)
            .output()
            .await?;
        
        let elapsed = start_time.elapsed();
        eprintln!("[GCLOUD] Command completed in {:.2}s", elapsed.as_secs_f64());

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("[GCLOUD] Command failed: {}", stderr);
            anyhow::bail!("gcloud command failed: {}", stderr);
        }

        let stdout = String::from_utf8(output.stdout)?;
        if stdout.trim().is_empty() {
            eprintln!("[GCLOUD] No logs returned (empty output)");
            return Ok(Vec::new());
        }

        let logs: Vec<LogEntry> = serde_json::from_str(&stdout)?;
        eprintln!("[GCLOUD] Successfully parsed {} log entries", logs.len());
        Ok(logs)
    }
}
