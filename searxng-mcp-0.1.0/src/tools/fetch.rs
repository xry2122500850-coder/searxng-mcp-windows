//! URL Fetch Tool
//!
//! Fetches and extracts content from web pages, converting HTML to readable markdown.

use super::{ToolExecutor, error_result, extract_main_content};
use crate::mcp::{CallToolResult, Tool, ToolContent};
use async_trait::async_trait;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::time::Duration;

/// Fetch URL tool
pub struct FetchTool {
    client: reqwest::Client,
}

impl FetchTool {
    /// Create a new fetch tool
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .connect_timeout(Duration::from_secs(15))
            .pool_idle_timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    /// Fetch content from a URL
    pub async fn fetch(
        &self,
        url: &str,
        extract_content: bool,
        max_length: Option<usize>,
    ) -> anyhow::Result<FetchResult> {
        tracing::debug!("Fetching URL: {}", url);

        // Validate URL
        let parsed_url = url.parse::<reqwest::Url>()?;
        if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
            anyhow::bail!("Only HTTP and HTTPS URLs are supported");
        }

        // Build request
        let request = self
            .client
            .get(url)
            .header("User-Agent", "searxng-mcp/1.0 (Content Fetcher)")
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
            .header("Accept-Language", "en-US,en;q=0.9")
            .header("DNT", "1");

        // Send request
        let response = request.send().await?;
        let status = response.status();
        let headers = response.headers().clone();

        // Get content type
        let content_type = headers
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("text/html")
            .to_string();

        // Get the body
        let body = response.text().await?;

        if !status.is_success() {
            anyhow::bail!("HTTP error {}: {}", status, body);
        }

        tracing::debug!("Fetched {} bytes of {}", body.len(), content_type);

        // Process based on content type
        let (content, is_truncated) = if content_type.contains("text/html") && extract_content {
            let extracted = extract_main_content(&body);
            let truncated = max_length
                .map(|max| truncate_text(&extracted, max))
                .unwrap_or(extracted);
            (truncated, max_length.is_some())
        } else if content_type.contains("text/") || content_type.contains("application/json") {
            let truncated = max_length
                .map(|max| truncate_text(&body, max))
                .unwrap_or(body);
            (truncated, max_length.is_some())
        } else {
            (format!("[Binary content: {}]", content_type), false)
        };

        Ok(FetchResult {
            url: url.to_string(),
            content,
            content_type,
            status_code: status.as_u16(),
            is_truncated,
            headers: headers
                .iter()
                .filter_map(|(k, v)| v.to_str().ok().map(|v| (k.to_string(), v.to_string())))
                .collect(),
        })
    }

    /// Fetch multiple URLs in parallel
    pub async fn fetch_multiple(
        &self,
        urls: Vec<String>,
        extract_content: bool,
        max_length: Option<usize>,
    ) -> Vec<(String, anyhow::Result<FetchResult>)> {
        use futures::future::join_all;

        let futures = urls.into_iter().map(|url| async move {
            let result = self.fetch(&url, extract_content, max_length).await;
            (url, result)
        });

        join_all(futures).await
    }
}

impl Default for FetchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolExecutor for FetchTool {
    fn name(&self) -> &str {
        "fetch_url"
    }

    fn get_definition(&self) -> Tool {
        Tool {
            name: "fetch_url".to_string(),
            description: concat!(
                "Fetch and extract content from a web page. Converts HTML to readable markdown format. ",
                "Use this after web_search to retrieve full content from specific URLs. ",
                "The tool automatically extracts the main article content and removes navigation, ads, and scripts."
            ).to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The full URL to fetch (must start with http:// or https://)"
                    },
                    "extract_content": {
                        "type": "boolean",
                        "description": "Extract main content from HTML (default: true). Set to false to get raw HTML.",
                        "default": true
                    },
                    "max_length": {
                        "type": "integer",
                        "description": "Maximum content length in characters (default: no limit). Content will be truncated if longer.",
                        "minimum": 1000,
                        "maximum": 500000
                    }
                },
                "required": ["url"]
            }),
        }
    }

    async fn execute(&self, args: Value) -> anyhow::Result<CallToolResult> {
        // Parse arguments
        let url = args
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: url"))?;

        let extract_content = args
            .get("extract_content")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let max_length = args
            .get("max_length")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize);

        // Execute the fetch
        match self.fetch(url, extract_content, max_length).await {
            Ok(result) => {
                let mut output = format!(
                    "# Content from: {}\n\n",
                    result.url
                );
                
                output.push_str(&format!(
                    "**Status:** {} | **Content-Type:** {}\n\n",
                    result.status_code, result.content_type
                ));

                if result.is_truncated {
                    output.push_str("*Note: Content was truncated to fit within limits.*\n\n");
                }

                output.push_str("---\n\n");
                output.push_str(&result.content);

                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: output }],
                    is_error: Some(false),
                })
            }
            Err(e) => {
                tracing::error!("Fetch failed for {}: {}", url, e);
                Ok(error_result(format!("Failed to fetch {}: {}", url, e)))
            }
        }
    }
}

/// Result of a fetch operation
#[derive(Debug, Clone)]
pub struct FetchResult {
    pub url: String,
    pub content: String,
    pub content_type: String,
    pub status_code: u16,
    pub is_truncated: bool,
    pub headers: HashMap<String, String>,
}

/// Truncate text to approximately max_chars, trying to end at a sentence boundary
fn truncate_text(text: &str, max_chars: usize) -> String {
    if text.len() <= max_chars {
        return text.to_string();
    }

    // Find a good breaking point (end of sentence or paragraph)
    let truncate_point = text
        .char_indices()
        .take(max_chars)
        .collect::<Vec<_>>()
        .last()
        .map(|(i, _)| *i)
        .unwrap_or(max_chars);

    let truncated = &text[..truncate_point];

    // Try to find a sentence ending
    let sentence_endings = [". ", "! ", "? ", "\n\n"];
    let mut best_end = truncate_point;

    for ending in &sentence_endings {
        if let Some(pos) = truncated.rfind(ending) {
            if pos > best_end.saturating_sub(500) {
                best_end = pos + ending.len() - 1;
            }
        }
    }

    format!("{}...\n\n[Content truncated]", &text[..best_end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_tool_name() {
        let tool = FetchTool::new();
        assert_eq!(tool.name(), "fetch_url");
    }

    #[test]
    fn test_truncate_text() {
        let long_text = "This is a very long text. It has multiple sentences. ".repeat(100);
        let truncated = truncate_text(&long_text, 500);
        assert!(truncated.len() <= 600); // Some buffer for the truncation message
        assert!(truncated.ends_with("[Content truncated]"));
    }

    #[test]
    fn test_truncate_text_short() {
        let short_text = "Short text.";
        let truncated = truncate_text(short_text, 1000);
        assert_eq!(truncated, short_text);
    }
}
