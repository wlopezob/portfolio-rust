use anyhow::Result;
use rmcp::{
    ErrorData as McpError, ServerHandler,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*, tool, tool_handler, tool_router,
};

use crate::auth::AuthProvider;
use crate::filters::FilterBuilder;
use crate::formatter::{LogFormatter, TextLogFormatter};
use crate::gcloud::{GcloudClient, LoggingClient, RetryPolicy};
use crate::request::ListLogsRequest;

/// Cloud Logging MCP Service
#[derive(Clone)]
pub struct CloudLoggingService<A: AuthProvider + Clone> {
    tool_router: ToolRouter<Self>,
    auth_provider: A,
}

#[tool_router]
impl<A: AuthProvider + Clone + Send + Sync + 'static> CloudLoggingService<A> {
    pub fn new(auth_provider: A) -> Self {
        Self {
            tool_router: Self::tool_router(),
            auth_provider,
        }
    }

    #[tool(description = "Retrieve logs from Google Cloud Logging with various filters. Supports filtering by resource type, log name, namespace, pod name, severity, and time range.")]
    async fn list_logs(
        &self,
        Parameters(req): Parameters<ListLogsRequest>,
    ) -> Result<CallToolResult, McpError> {
        eprintln!("\n=== LIST_LOGS REQUEST ===");
        self.log_request(&req);

        // Build filter string
        let filter = FilterBuilder::new(&req)
            .build()
            .map_err(|e| {
                eprintln!("[ERROR] Failed to build filter: {}", e);
                McpError::internal_error(format!("Failed to build filter: {}", e), None)
            })?;

        eprintln!(
            "[FILTER] Generated filter: {}",
            if filter.is_empty() { "<empty>" } else { &filter }
        );

        // Get effective values
        let limit = req.effective_limit();
        let order = req.normalized_order();
        
        eprintln!("[LIMIT] Using limit: {} (requested: {})", limit, req.limit);
        eprintln!("[ORDER] Using order: {}", order);

        // Create client and execute with retry
        let client = GcloudClient::new(self.auth_provider.clone());
        let retry_policy = RetryPolicy::default();
        
        let logs = retry_policy
            .execute(|| async {
                client
                    .fetch_logs(&req.project_id, &filter, limit, order)
                    .await
            })
            .await
            .map_err(|e| {
                eprintln!("[ERROR] Failed to fetch logs after retries: {}", e);
                McpError::internal_error(format!("Failed to fetch logs: {}", e), None)
            })?;

        eprintln!("[SUCCESS] Retrieved {} logs", logs.len());

        // Format output
        let formatter = TextLogFormatter::new();
        let formatted = formatter.format(&logs);
        eprintln!("=== LIST_LOGS COMPLETE ===\n");

        Ok(CallToolResult::success(vec![Content::text(formatted)]))
    }
    
    fn log_request(&self, req: &ListLogsRequest) {
        eprintln!("[REQUEST] project_id: {}", req.project_id);
        eprintln!("[REQUEST] resource_type: {:?}", req.resource_type);
        eprintln!("[REQUEST] log_name: {:?}", req.log_name);
        eprintln!("[REQUEST] namespace: {:?}", req.namespace);
        eprintln!("[REQUEST] pod_name: {:?}", req.pod_name);
        eprintln!("[REQUEST] severity: {:?}", req.severity);
        eprintln!("[REQUEST] since: {:?}", req.since);
        eprintln!("[REQUEST] until: {:?}", req.until);
        eprintln!("[REQUEST] limit: {}", req.limit);
        eprintln!("[REQUEST] order: {}", req.order);
    }
}

#[tool_handler]
impl<A: AuthProvider + Clone + Send + Sync + 'static> ServerHandler for CloudLoggingService<A> {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Google Cloud Logging MCP Server. Use list_logs to query logs from GCP projects. \
                Requires gcloud CLI to be installed and authenticated."
                    .to_string(),
            ),
        }
    }
}
