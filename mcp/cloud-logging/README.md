# Google Cloud Logging MCP Server

MCP server to query Google Cloud Logging using `gcloud` CLI as a reliable wrapper.

## Features

- 🔌 **MCP v2024-11-05**: Claude Desktop integration
- 🔍 **Flexible filters**: resource type, log name, namespace, pod, severity, time
- ⏱️ **Relative times**: `1h`, `2d` or absolute RFC3339
- 🔄 **Automatic retry**: 3 attempts with exponential backoff
- ✅ **Reliable**: Uses `gcloud` to avoid direct API HTTP 500 errors

## Quick Start

```bash
# 1. Authentication
gcloud auth application-default login

# 2. Build and run
cargo run --release

# 3. Configure with Claude Code CLI
claude mcp add wlopezob-gcp-logging --type http --url http://127.0.0.1:8766/mcp

# Alternatively, edit manually: ~/Library/Application Support/Claude/claude_desktop_config.json
# {
#   "mcpServers": {
#     "wlopezob-gcp-logging": {
#       "type": "http",
#       "url": "http://127.0.0.1:8766/mcp"
#     }
#   }
# }
```

## Prerequisites

- **Rust 1.75+**: `rustc --version`
- **Google Cloud SDK**: `gcloud --version`
- **GCP Permissions**: `roles/logging.viewer`

## `list_logs` Tool

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `project_id` | string | Yes | GCP project ID |
| `resource_type` | string | No | Resource type (e.g., `k8s_node`, `k8s_pod`) |
| `log_name` | string | No | Log name (substring) |
| `namespace` | string | No | Kubernetes namespace |
| `pod_name` | string | No | Pod name (substring) |
| `severity` | string | No | Severity (`ERROR`, `WARNING`, `INFO`) |
| `since` | string | No | Start time (`1h`, `2d`, RFC3339) |
| `until` | string | No | End time (RFC3339) |
| `limit` | number | No | Max logs (default: 20, max: 1000) |
| `order` | string | No | Order: `asc` or `desc` (default) |

**Example**:
```json
{
  "project_id": "my-gcp-project",
  "resource_type": "k8s_node",
  "log_name": "application-logs",
  "namespace": "production",
  "pod_name": "web-server",
  "since": "1h",
  "limit": 50
}
```

## Usage with Claude Code CLI

Once the server is running, you can use Claude CLI to query logs:

```bash
claude "use the wlopezob-gcp-logging mcp for projectid: my-gcp-project, type: k8s_node, logName: application-logs, namespace: production, pod_name: web-server, get logs from the last 1h in descending order, I want the first 20 records"
```

**Note**: The server must be running (`cargo run --release`) before executing Claude CLI commands.

## Performance

| Time Range | Expected Duration |
|------------|-------------------|
| `10m` | 2-5 seconds |
| `1h` | 5-15 seconds ✅ |
| `1d` | 30-90 seconds |
| `2d` | 1-5 minutes (may require retry) |
| No `since` | 30-120 seconds (default: 24h) |

**Optimization**:
- Use small time ranges (`1h` recommended)
- Add `resource_type` to leverage GCP indexes
- Server automatically retries on HTTP 500

## Architecture

```
src/
├── main.rs          # HTTP server setup (axum)
├── auth.rs          # AuthProvider trait + caching
├── filters.rs       # GCP filter construction
├── formatter.rs     # Log formatting (text/json)
├── gcloud/
│   ├── client.rs    # GcloudClient + LoggingClient trait
│   └── retry.rs     # RetryPolicy with exponential backoff
├── models.rs        # LogEntry, Resource
├── request.rs       # ListLogsRequest + validation
├── server.rs        # CloudLoggingService (MCP)
└── time.rs          # TimeParser (RFC3339 + relative)
```

**Why `gcloud` CLI?**  
Direct API returns HTTP 500 with certain filters (~40% failure rate). `gcloud` is 100% reliable.

**Why HTTP instead of stdio?**  
- Supports multiple clients
- Better for debugging (curl, inspector)
- Easier monitoring
- Trade-off: requires manual start

## Troubleshooting

### Slow Queries

**Expected**: 1-5 minute queries with large ranges. Automatic retry handles HTTP 500.

**Solutions**:
- Use `since: "1h"` (fast) instead of `2d` (slow)
- Add `resource_type` to leverage indexes
- No `since` → automatic 24h default

### "gcloud command failed" Error

```bash
# Verify installation
which gcloud
gcloud --version

# Authenticate
gcloud auth application-default login
```

### Permission Error

```bash
gcloud projects add-iam-policy-binding PROJECT_ID \
  --member="user:your-email@example.com" \
  --role="roles/logging.viewer"
```

## Development

```bash
cargo build          # Build
cargo run --release  # Run
cargo test           # Tests
cargo clippy         # Linter
```

## License

MIT License

---

**MCP Protocol** | Built with Rust 🦀