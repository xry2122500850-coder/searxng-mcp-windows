//! MCP Server Implementation
//!
//! Handles the lifecycle and request routing for an MCP server over stdio.

use crate::mcp::*;
use crate::tools::{FetchTool, SearchTool, ToolExecutor};
use anyhow::Result;
use serde_json::json;
use std::io::{BufRead, Write};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info, warn};

/// MCP Server state and configuration
pub struct McpServer {
    /// Server name
    name: String,
    /// Server version
    version: String,
    /// Available tools
    tools: Vec<Arc<dyn ToolExecutor>>,
    /// Whether server is initialized
    initialized: bool,
}

impl McpServer {
    pub fn new(name: impl Into<String>, version: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            tools: Vec::new(),
            initialized: false,
        }
    }

    /// Register a tool with the server
    pub fn register_tool(&mut self, tool: Arc<dyn ToolExecutor>) {
        self.tools.push(tool);
    }

    /// Get tool definitions for MCP protocol
    fn get_tool_definitions(&self) -> Vec<Tool> {
        self.tools.iter().map(|t| t.get_definition()).collect()
    }

    /// Find a tool by name
    fn find_tool(&self, name: &str) -> Option<Arc<dyn ToolExecutor>> {
        self.tools.iter().find(|t| t.name() == name).cloned()
    }

    /// Handle initialize request
    fn handle_initialize(&mut self, params: InitializeParams) -> Result<InitializeResult> {
        info!(
            "Initializing MCP client: {} (protocol version: {})",
            params.client_info.name, params.protocol_version
        );

        self.initialized = true;

        Ok(InitializeResult {
            protocol_version: MCP_VERSION.to_string(),
            capabilities: ServerCapabilities {
                tools: Some(ToolsCapability { list_changed: false }),
                resources: None,
                prompts: None,
                logging: None,
            },
            server_info: Implementation {
                name: self.name.clone(),
                version: self.version.clone(),
            },
        })
    }

    /// Handle tools/list request
    fn handle_list_tools(&self) -> Result<ListToolsResult> {
        Ok(ListToolsResult {
            tools: self.get_tool_definitions(),
            next_cursor: None,
        })
    }

    /// Handle tools/call request
    async fn handle_call_tool(&self, params: CallToolParams) -> Result<CallToolResult> {
        let tool = self.find_tool(&params.name).ok_or_else(|| {
            anyhow::anyhow!("Tool not found: {}", params.name)
        })?;

        let args = params.arguments.unwrap_or(json!({}));
        tool.execute(args).await
    }

    /// Process a single JSON-RPC request
    async fn process_request(
        &mut self,
        request: JsonRpcRequest,
    ) -> Option<JsonRpcResponse> {
        let id = request.id.clone();

        // Validate JSON-RPC version
        if request.jsonrpc != "2.0" {
            return Some(JsonRpcResponse::error(
                id,
                error_codes::INVALID_REQUEST,
                "Invalid JSON-RPC version".to_string(),
                None,
            ));
        }

        // Handle notifications (no id)
        let is_notification = id.is_none();

        let result = match request.method.as_str() {
            "initialize" => {
                match serde_json::from_value::<InitializeParams>(
                    request.params.unwrap_or(json!({})),
                ) {
                    Ok(params) => match self.handle_initialize(params) {
                        Ok(result) => Ok(json!(result)),
                        Err(e) => Err((error_codes::INTERNAL_ERROR, e.to_string())),
                    },
                    Err(e) => Err((error_codes::INVALID_PARAMS, format!("Invalid params: {}", e))),
                }
            }
            "initialized" => {
                // Notification from client that initialization is complete
                debug!("Client confirmed initialization");
                Ok(json!({}))
            }
            "tools/list" => {
                if !self.initialized {
                    return Some(JsonRpcResponse::error(
                        id,
                        error_codes::SERVER_ERROR_START,
                        "Server not initialized".to_string(),
                        None,
                    ));
                }
                match self.handle_list_tools() {
                    Ok(result) => Ok(json!(result)),
                    Err(e) => Err((error_codes::INTERNAL_ERROR, e.to_string())),
                }
            }
            "tools/call" => {
                if !self.initialized {
                    return Some(JsonRpcResponse::error(
                        id,
                        error_codes::SERVER_ERROR_START,
                        "Server not initialized".to_string(),
                        None,
                    ));
                }
                match serde_json::from_value::<CallToolParams>(
                    request.params.unwrap_or(json!({})),
                ) {
                    Ok(params) => match self.handle_call_tool(params).await {
                        Ok(result) => Ok(json!(result)),
                        Err(e) => Err((error_codes::INTERNAL_ERROR, e.to_string())),
                    },
                    Err(e) => Err((error_codes::INVALID_PARAMS, format!("Invalid params: {}", e))),
                }
            }
            _ => Err((
                error_codes::METHOD_NOT_FOUND,
                format!("Method not found: {}", request.method),
            )),
        };

        // Don't send response for notifications
        if is_notification {
            return None;
        }

        Some(match result {
            Ok(value) => JsonRpcResponse::success(id, value),
            Err((code, message)) => JsonRpcResponse::error(id, code, message, None),
        })
    }

    /// Run the server over stdio
    pub async fn run_stdio(&mut self) -> Result<()> {
        info!("Starting MCP server: {} v{}", self.name, self.version);

        let stdin = std::io::stdin();
        let stdout = Arc::new(Mutex::new(std::io::stdout()));
        let mut stderr = std::io::stderr();

        // Write startup message to stderr (not stdout, which is for MCP protocol)
        writeln!(
            stderr,
            "SearXNG MCP Server started. Waiting for MCP client connections..."
        )?;
        writeln!(
            stderr,
            "Configure your MCP client with: {{ \"command\": \"{}\", \"args\": [] }}",
            std::env::current_exe()?.display()
        )?;

        for line in stdin.lock().lines() {
            let line = match line {
                Ok(l) => l,
                Err(e) => {
                    error!("Error reading from stdin: {}", e);
                    continue;
                }
            };

            // Skip empty lines
            if line.trim().is_empty() {
                continue;
            }

            debug!("Received: {}", line);

            // Parse the JSON-RPC request
            let request: JsonRpcRequest = match serde_json::from_str(&line) {
                Ok(req) => req,
                Err(e) => {
                    warn!("Failed to parse JSON-RPC request: {}", e);
                    let response = JsonRpcResponse::error(
                        None,
                        error_codes::PARSE_ERROR,
                        format!("Parse error: {}", e),
                        None,
                    );
                    self.send_response(&stdout, response).await?;
                    continue;
                }
            };

            // Process the request
            if let Some(response) = self.process_request(request).await {
                self.send_response(&stdout, response).await?;
            }
        }

        info!("MCP server shutting down");
        Ok(())
    }

    /// Send a JSON-RPC response over stdout
    async fn send_response(
        &self,
        stdout: &Arc<Mutex<std::io::Stdout>>,
        response: JsonRpcResponse,
    ) -> Result<()> {
        let json = serde_json::to_string(&response)?;
        debug!("Sending: {}", json);

        let mut stdout = stdout.lock().await;
        writeln!(stdout, "{}", json)?;
        stdout.flush()?;

        Ok(())
    }
}

/// Create a configured MCP server with all SearXNG tools
pub fn create_server() -> McpServer {
    let mut server = McpServer::new("searxng-mcp", env!("CARGO_PKG_VERSION"));

    // Register search tool
    server.register_tool(Arc::new(SearchTool::new()));

    // Register fetch URL tool
    server.register_tool(Arc::new(FetchTool::new()));

    server
}
