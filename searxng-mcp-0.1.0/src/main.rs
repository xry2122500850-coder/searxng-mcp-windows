//! SearXNG MCP Server - Comprehensive Search Tools
//!
//! A Model Context Protocol (MCP) server providing comprehensive search capabilities
//! that rival Brave Search API. Powered by SearXNG, aggregating results from 89+
//! search engines.
//!
//! ## Available Tools
//!
//! - `web_search` - General web search with advanced filters (time, language, safe search)
//! - `image_search` - Image search across multiple sources
//! - `news_search` - News search with recency filters
//! - `video_search` - Video search with thumbnails
//! - `search_suggestions` - Autocomplete/suggestions
//! - `technical_search` - Code and documentation search
//! - `fetch_url` - Fetch and extract content from URLs
//!
//! ## Environment Variables
//!
//! - `SEARXNG_URL` or `SEARXNG_ENDPOINT`: SearXNG instance URL (default: http://localhost:8888)
//! - `RUST_LOG`: Logging level (error, warn, info, debug, trace)
//!
//! ## Claude Desktop Configuration
//!
//! ```json
//! {
//!   "mcpServers": {
//!     "searxng": {
//!       "command": "/path/to/searxng-mcp",
//!       "env": {
//!         "SEARXNG_URL": "http://localhost:8888",
//!         "RUST_LOG": "info"
//!       }
//!     }
//!   }
//! }
//! ```

use tracing::{error, info};

mod mcp;
mod tools;
mod searxng_client;

use mcp::server::McpServer;
use tools::{
    FetchTool, SearchTool, ImageSearchTool, NewsSearchTool,
    VideoSearchTool, SuggestionsTool, TechnicalSearchTool,
};
use std::sync::Arc;

fn create_server() -> McpServer {
    let mut server = McpServer::new("searxng-mcp", env!("CARGO_PKG_VERSION"));

    // Register all search tools
    server.register_tool(Arc::new(SearchTool::new()));
    server.register_tool(Arc::new(ImageSearchTool::new()));
    server.register_tool(Arc::new(NewsSearchTool::new()));
    server.register_tool(Arc::new(VideoSearchTool::new()));
    server.register_tool(Arc::new(SuggestionsTool::new()));
    server.register_tool(Arc::new(TechnicalSearchTool::new()));
    server.register_tool(Arc::new(FetchTool::new()));

    info!("Registered {} tools", 7);

    server
}

#[tokio::main]
async fn main() {
    // Initialize logging to stderr (stdout is reserved for MCP protocol)
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    info!("╔═══════════════════════════════════════════╗");
    info!("║     SearXNG MCP Server v{}              ║", env!("CARGO_PKG_VERSION"));
    info!("║  Comprehensive Search Tools for AI      ║");
    info!("╚═══════════════════════════════════════════╝");
    info!("");
    info!("Powered by SearXNG - Aggregating 89+ search engines");
    info!("Server communicates over stdio using MCP protocol");
    info!("");

    // Create and run the server
    let mut server = create_server();

    if let Err(e) = server.run_stdio().await {
        error!("Server error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let _server = create_server();
    }
}
