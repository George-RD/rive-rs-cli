use rmcp::{
    ErrorData as McpError, RoleServer,
    handler::server::{tool::ToolRouter, wrapper::Parameters},
    model::*,
    service::ServiceExt,
    tool, tool_handler, tool_router,
};
use std::future::Future;

#[derive(Clone)]
pub struct RiveMcpServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl RiveMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        name = "generate",
        description = "Generate a .riv file from a SceneSpec JSON string. Returns base64-encoded bytes or writes to file."
    )]
    async fn generate(
        &self,
        params: Parameters<GenerateParams>,
    ) -> Result<CallToolResult, McpError> {
        let input = &params.0.scene_json;
        let spec: crate::builder::SceneSpec = serde_json::from_str(input).map_err(|e| {
            McpError::new(
                ErrorCode::INVALID_PARAMS,
                format!("invalid JSON: {}", e),
                None,
            )
        })?;
        let scene = crate::builder::build_scene(&spec).map_err(|e| {
            McpError::new(
                ErrorCode::INVALID_PARAMS,
                format!("invalid scene: {}", e),
                None,
            )
        })?;
        let refs: Vec<&dyn crate::objects::core::RiveObject> = scene.iter().map(|o| &**o).collect();
        let file_id = params.0.file_id.unwrap_or(0);
        let bytes = crate::encoder::encode_riv(&refs, file_id);

        if let Some(path) = &params.0.output {
            std::fs::write(path, &bytes).map_err(|e| {
                McpError::new(
                    ErrorCode::INTERNAL_ERROR,
                    format!("write error: {}", e),
                    None,
                )
            })?;
            Ok(CallToolResult::success(vec![Content::text(format!(
                "wrote {} bytes to {}",
                bytes.len(),
                path
            ))]))
        } else {
            let encoded: String = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
            Ok(CallToolResult::success(vec![Content::text(encoded)]))
        }
    }

    #[tool(
        name = "validate",
        description = "Validate a .riv file at the given path. Returns validation report."
    )]
    async fn validate(
        &self,
        params: Parameters<ValidateParams>,
    ) -> Result<CallToolResult, McpError> {
        let bytes = std::fs::read(&params.0.file).map_err(|e| {
            McpError::new(
                ErrorCode::INVALID_PARAMS,
                format!("read error: {}", e),
                None,
            )
        })?;
        match crate::validator::validate_riv(&bytes) {
            Ok(report) => {
                let json = serde_json::to_string_pretty(&report)
                    .map_err(|e| McpError::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::success(vec![Content::text(format!(
                "invalid: {}",
                e
            ))])),
        }
    }

    #[tool(
        name = "inspect",
        description = "Inspect a .riv file and return its object tree as JSON."
    )]
    async fn inspect(&self, params: Parameters<InspectParams>) -> Result<CallToolResult, McpError> {
        let bytes = std::fs::read(&params.0.file).map_err(|e| {
            McpError::new(
                ErrorCode::INVALID_PARAMS,
                format!("read error: {}", e),
                None,
            )
        })?;
        let filter = crate::validator::InspectFilter::default();
        match crate::validator::parse_riv(&bytes, &filter) {
            Ok(parsed) => {
                let json = serde_json::to_string_pretty(&parsed)
                    .map_err(|e| McpError::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;
                Ok(CallToolResult::success(vec![Content::text(json)]))
            }
            Err(e) => Ok(CallToolResult::success(vec![Content::text(format!(
                "parse error: {}",
                e
            ))])),
        }
    }

    #[tool(
        name = "list_templates",
        description = "List available animation templates."
    )]
    async fn list_templates(&self) -> Result<CallToolResult, McpError> {
        let templates = crate::ai::templates::list_templates();
        let json = serde_json::to_string_pretty(&templates)
            .map_err(|e| McpError::new(ErrorCode::INTERNAL_ERROR, e.to_string(), None))?;
        Ok(CallToolResult::success(vec![Content::text(json)]))
    }
}

#[tool_handler]
impl rmcp::handler::server::ServerHandler for RiveMcpServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation {
                name: "rive-cli".into(),
                title: None,
                version: env!("CARGO_PKG_VERSION").into(),
                description: None,
                icons: None,
                website_url: None,
            },
            instructions: Some("Rive CLI MCP server. Use 'generate' to create .riv files from SceneSpec JSON, 'validate' to check .riv files, 'inspect' to dump object trees, and 'list_templates' for animation templates.".into()),
        }
    }

    fn list_resources(
        &self,
        _request: Option<PaginatedRequestParams>,
        _context: rmcp::service::RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ListResourcesResult, McpError>> + Send + '_ {
        std::future::ready(Ok(ListResourcesResult {
            resources: vec![Resource {
                raw: RawResource {
                    uri: "schema://scene/v1".into(),
                    name: "SceneSpec JSON Schema v1".into(),
                    title: None,
                    description: Some("JSON Schema for rive-cli scene input format".into()),
                    mime_type: Some("application/json".into()),
                    size: None,
                    icons: None,
                    meta: None,
                },
                annotations: None,
            }],
            next_cursor: None,
            meta: None,
        }))
    }

    fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: rmcp::service::RequestContext<RoleServer>,
    ) -> impl Future<Output = Result<ReadResourceResult, McpError>> + Send + '_ {
        if request.uri.as_str() == "schema://scene/v1" {
            let schema = include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/docs/scene.schema.v1.json"
            ));
            std::future::ready(Ok(ReadResourceResult {
                contents: vec![ResourceContents::text(schema, "schema://scene/v1")],
            }))
        } else {
            std::future::ready(Err(McpError::new(
                ErrorCode::INVALID_PARAMS,
                format!("unknown resource: {}", request.uri),
                None,
            )))
        }
    }
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct GenerateParams {
    #[schemars(description = "SceneSpec JSON string")]
    pub scene_json: String,
    #[schemars(description = "Optional output file path")]
    pub output: Option<String>,
    #[schemars(description = "Optional Rive file ID (default: 0)")]
    pub file_id: Option<u64>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct ValidateParams {
    #[schemars(description = "Path to .riv file to validate")]
    pub file: String,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct InspectParams {
    #[schemars(description = "Path to .riv file to inspect")]
    pub file: String,
}

pub fn run_server() {
    let runtime = match tokio::runtime::Runtime::new() {
        Ok(runtime) => runtime,
        Err(e) => {
            eprintln!("failed to create tokio runtime: {}", e);
            std::process::exit(1);
        }
    };

    runtime.block_on(async {
        let server = RiveMcpServer::new();
        let service = match server.serve(rmcp::transport::io::stdio()).await {
            Ok(service) => service,
            Err(e) => {
                eprintln!("failed to start MCP server: {}", e);
                std::process::exit(1);
            }
        };
        if let Err(e) = service.waiting().await {
            eprintln!("MCP server error: {}", e);
            std::process::exit(1);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = RiveMcpServer::new();
        drop(server);
    }
}
