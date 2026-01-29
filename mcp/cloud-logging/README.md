# Google Cloud Logging MCP Server

MCP server para consultar Google Cloud Logging usando `gcloud` CLI como wrapper confiable.

## Características

- 🔌 **MCP v2024-11-05**: Integración con Claude Desktop
- 🔍 **Filtros flexibles**: resource type, log name, namespace, pod, severity, tiempo
- ⏱️ **Tiempos relativos**: `1h`, `2d` o RFC3339 absolutos
- 🔄 **Retry automático**: 3 intentos con backoff exponencial
- ✅ **Confiable**: Usa `gcloud` para evitar HTTP 500 de la API directa

## Inicio Rápido

```bash
# 1. Autenticación
gcloud auth application-default login

# 2. Compilar y ejecutar
cargo run --release

# 3. Configurar Claude Desktop
# Editar: ~/Library/Application Support/Claude/claude_desktop_config.json
{
  "mcpServers": {
    "gcp-logging": {
      "type": "http",
      "url": "http://127.0.0.1:8766/mcp"
    }
  }
}
```

## Prerrequisitos

- **Rust 1.75+**: `rustc --version`
- **Google Cloud SDK**: `gcloud --version`
- **Permisos GCP**: `roles/logging.viewer`

## Herramienta `list_logs`

| Parámetro | Tipo | Requerido | Descripción |
|-----------|------|-----------|-------------|
| `project_id` | string | Sí | ID del proyecto GCP |
| `resource_type` | string | No | Tipo de recurso (ej: `k8s_node`, `k8s_pod`) |
| `log_name` | string | No | Nombre del log (substring) |
| `namespace` | string | No | Namespace de Kubernetes |
| `pod_name` | string | No | Nombre del pod (substring) |
| `severity` | string | No | Severidad (`ERROR`, `WARNING`, `INFO`) |
| `since` | string | No | Tiempo inicio (`1h`, `2d`, RFC3339) |
| `until` | string | No | Tiempo fin (RFC3339) |
| `limit` | number | No | Máx. logs (default: 20, max: 1000) |
| `order` | string | No | Orden: `asc` o `desc` (default) |

**Ejemplo**:
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

## Uso con Claude Code CLI

Una vez que el servidor esté corriendo, puedes usar Claude CLI para consultar logs:

```bash
claude "usa el mcp gcp-logging para el projectid: my-gcp-project, type: k8s_node, logName: application-logs, namespace: production, pod_name: web-server, obten los logs desde 1h y de forma descendente, quiero los primeros 20 registros"
```

**Nota**: El servidor debe estar corriendo (`cargo run --release`) antes de ejecutar comandos con Claude CLI.

## Rendimiento

| Rango de Tiempo | Duración Esperada |
|-----------------|-------------------|
| `10m` | 2-5 segundos |
| `1h` | 5-15 segundos ✅ |
| `1d` | 30-90 segundos |
| `2d` | 1-5 minutos (puede requerir retry) |
| Sin `since` | 30-120 segundos (default: 24h) |

**Optimización**:
- Usa rangos de tiempo pequeños (`1h` recomendado)
- Agrega `resource_type` para aprovechar índices GCP
- El servidor reintenta automáticamente en HTTP 500

## Arquitectura

```
src/
├── main.rs          # Setup HTTP server (axum)
├── auth.rs          # AuthProvider trait + caching
├── filters.rs       # Construcción de filtros GCP
├── formatter.rs     # Formateo de logs (text/json)
├── gcloud/
│   ├── client.rs    # GcloudClient + LoggingClient trait
│   └── retry.rs     # RetryPolicy con backoff exponencial
├── models.rs        # LogEntry, Resource
├── request.rs       # ListLogsRequest + validación
├── server.rs        # CloudLoggingService (MCP)
└── time.rs          # TimeParser (RFC3339 + relativos)
```

**¿Por qué `gcloud` CLI?**  
La API directa retorna HTTP 500 con ciertos filtros (~40% tasa de fallo). `gcloud` es 100% confiable.

**¿Por qué HTTP en vez de stdio?**  
- Soporta múltiples clientes
- Mejor para debugging (curl, inspector)
- Monitoreo más fácil
- Trade-off: requiere inicio manual

## Troubleshooting

### Queries Lentas

**Esperado**: Queries de 1-5 minutos con rangos grandes. El retry automático maneja HTTP 500.

**Soluciones**:
- Usa `since: "1h"` (rápido) en vez de `2d` (lento)
- Agrega `resource_type` para aprovechar índices
- Sin `since` → default automático 24h

### Error "gcloud command failed"

```bash
# Verificar instalación
which gcloud
gcloud --version

# Autenticar
gcloud auth application-default login
```

### Error de permisos

```bash
gcloud projects add-iam-policy-binding PROJECT_ID \
  --member="user:tu-email@example.com" \
  --role="roles/logging.viewer"
```

## Desarrollo

```bash
cargo build          # Compilar
cargo run --release  # Ejecutar
cargo test           # Tests
cargo clippy         # Linter
```

## Licencia

MIT License

---

**MCP Protocol** | Built with Rust 🦀