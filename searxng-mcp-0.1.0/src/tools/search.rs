//! Comprehensive Search Tools for SearXNG
//!
//! This module provides search tools that rival Brave Search:
//! - web_search: General web search with advanced filters
//! - image_search: Image search
//! - news_search: News search with recency
//! - video_search: Video search
//! - suggestions: Autocomplete
//! - technical_search: Code/IT search
//! - social_search: Social media search

use super::{ToolExecutor, error_result, format_search_results, format_image_results, format_video_results, format_news_results, format_suggestions};
use crate::mcp::{CallToolResult, Tool, ToolContent};
use crate::searxng_client::{
    SearXNGClient, SearchParams, SearchCategory, TimeRange, SafeSearch,
    SearchResult,
};
use async_trait::async_trait;
// use serde::Deserialize;
use serde_json::{Value, json};
use std::sync::Arc;

/// Comprehensive search tool with all SearXNG features
pub struct SearchTool {
    client: Arc<SearXNGClient>,
}

impl SearchTool {
    pub fn new() -> Self {
        Self {
            client: Arc::new(SearXNGClient::new()),
        }
    }

    /// Convert API result to display format
    fn convert_result(&self, result: &SearchResult) -> super::SearchResultItem {
        super::SearchResultItem {
            title: result.title.clone(),
            url: result.url.clone(),
            snippet: result.content.clone(),
            published_date: result.published_date.clone(),
            engines: Some(result.engines.clone()),
            score: result.score,
            category: result.category.clone(),
        }
    }

    /// Convert API result to image format
    fn convert_image_result(&self, result: &SearchResult) -> super::ImageResultItem {
        super::ImageResultItem {
            title: result.title.clone(),
            image_url: result.image_source.clone().unwrap_or_else(|| result.url.clone()),
            source_url: result.url.clone(),
            thumbnail_url: result.thumbnail.clone(),
            engines: Some(result.engines.clone()),
        }
    }

    /// Convert API result to video format
    fn convert_video_result(&self, result: &SearchResult) -> super::VideoResultItem {
        super::VideoResultItem {
            title: result.title.clone(),
            url: result.url.clone(),
            thumbnail_url: result.thumbnail.clone(),
            description: result.content.clone(),
            engines: Some(result.engines.clone()),
        }
    }

    /// Convert API result to news format
    fn convert_news_result(&self, result: &SearchResult) -> super::NewsResultItem {
        super::NewsResultItem {
            title: result.title.clone(),
            url: result.url.clone(),
            snippet: result.content.clone(),
            published_date: result.published_date.clone(),
            source: result.engine.clone(),
            engines: Some(result.engines.clone()),
        }
    }
}

impl Default for SearchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolExecutor for SearchTool {
    fn name(&self) -> &str {
        "web_search"
    }

    fn get_definition(&self) -> Tool {
        Tool {
            name: "web_search".to_string(),
            description: concat!(
                "Search the web using SearXNG with advanced filtering. This is a comprehensive web search ",
                "that aggregates results from multiple search engines including Google, Bing, DuckDuckGo, ",
                "Brave, and more. Supports time-based filtering, language selection, and safe search options."
            ).to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query"
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum results (1-50, default: 10)",
                        "minimum": 1,
                        "maximum": 50,
                        "default": 10
                    },
                    "time_range": {
                        "type": "string",
                        "description": "Time filter: 'day', 'week', 'month', 'year', or omit for all time",
                        "enum": ["day", "week", "month", "year"]
                    },
                    "language": {
                        "type": "string",
                        "description": "Language code (e.g., 'en', 'en-US', 'de', 'fr', 'es', 'ja')"
                    },
                    "safe_search": {
                        "type": "string",
                        "description": "Safe search level",
                        "enum": ["none", "moderate", "strict"],
                        "default": "moderate"
                    },
                    "engines": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Specific engines to use (e.g., ['google', 'bing', 'duckduckgo']). Omit to use all."
                    },
                    "page": {
                        "type": "integer",
                        "description": "Page number for pagination (default: 1)",
                        "minimum": 1,
                        "default": 1
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn execute(&self, args: Value) -> anyhow::Result<CallToolResult> {
        // Parse arguments
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: query"))?;

        let max_results = args
            .get("max_results")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(10)
            .clamp(1, 50);

        let time_range = args
            .get("time_range")
            .and_then(|v| v.as_str())
            .and_then(|v| match v {
                "day" => Some(TimeRange::Day),
                "week" => Some(TimeRange::Week),
                "month" => Some(TimeRange::Month),
                "year" => Some(TimeRange::Year),
                _ => None,
            });

        let language = args.get("language").and_then(|v| v.as_str().map(|s| s.to_string()));

        let safe_search = args
            .get("safe_search")
            .and_then(|v| v.as_str())
            .map(|v| match v {
                "none" => SafeSearch::None,
                "strict" => SafeSearch::Strict,
                _ => SafeSearch::Moderate,
            });

        let engines = args
            .get("engines")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            });

        let page = args
            .get("page")
            .and_then(|v| v.as_u64())
            .map(|v| v as u32)
            .unwrap_or(1);

        // Build search params
        let params = SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::General),
            time_range,
            language,
            safe_search,
            page: Some(page),
            engines,
            max_results: Some(max_results),
        };

        // Execute search
        match self.client.search(params).await {
            Ok(response) => {
                let items: Vec<_> = response.results.iter()
                    .map(|r| self.convert_result(r))
                    .collect();
                
                let formatted = format_search_results(query, &items, response.number_of_results);
                
                // Include suggestions if available
                let mut content = vec![ToolContent::Text { text: formatted }];
                
                if !response.suggestions.is_empty() {
                    let suggestions_text = format_suggestions(query, &response.suggestions);
                    content.push(ToolContent::Text { text: suggestions_text });
                }
                
                // Add unresponsive engines warning if any
                if !response.unresponsive_engines.is_empty() {
                    let engine_list: Vec<String> = response.unresponsive_engines
                        .iter()
                        .map(|e| {
                            if e.len() >= 2 {
                                format!("{} ({})", e[0], e[1])
                            } else if e.len() == 1 {
                                e[0].clone()
                            } else {
                                "unknown".to_string()
                            }
                        })
                        .collect();
                    let warning = format!(
                        "\n*Note: Some engines were unresponsive: {}*",
                        engine_list.join(", ")
                    );
                    content.push(ToolContent::Text { text: warning });
                }

                Ok(CallToolResult {
                    content,
                    is_error: Some(false),
                })
            }
            Err(e) => {
                tracing::error!("Search failed: {}", e);
                Ok(error_result(format!("Search failed: {}", e)))
            }
        }
    }
}

/// Image search tool
pub struct ImageSearchTool {
    client: Arc<SearXNGClient>,
}

impl ImageSearchTool {
    pub fn new() -> Self {
        Self {
            client: Arc::new(SearXNGClient::new()),
        }
    }

    fn convert_result(&self, result: &SearchResult) -> super::ImageResultItem {
        super::ImageResultItem {
            title: result.title.clone(),
            image_url: result.image_source.clone().unwrap_or_else(|| result.url.clone()),
            source_url: result.url.clone(),
            thumbnail_url: result.thumbnail.clone(),
            engines: Some(result.engines.clone()),
        }
    }
}

impl Default for ImageSearchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolExecutor for ImageSearchTool {
    fn name(&self) -> &str {
        "image_search"
    }

    fn get_definition(&self) -> Tool {
        Tool {
            name: "image_search".to_string(),
            description: concat!(
                "Search for images using SearXNG. Searches across multiple image sources including ",
                "Google Images, Bing Images, DuckDuckGo Images, and more. Returns direct image URLs, ",
                "thumbnails, and source pages."
            ).to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The image search query"
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum results (1-50, default: 10)",
                        "minimum": 1,
                        "maximum": 50,
                        "default": 10
                    },
                    "safe_search": {
                        "type": "string",
                        "description": "Safe search level",
                        "enum": ["none", "moderate", "strict"],
                        "default": "moderate"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn execute(&self, args: Value) -> anyhow::Result<CallToolResult> {
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: query"))?;

        let max_results = args
            .get("max_results")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(10)
            .clamp(1, 50);

        let safe_search = args
            .get("safe_search")
            .and_then(|v| v.as_str())
            .map(|v| match v {
                "none" => SafeSearch::None,
                "strict" => SafeSearch::Strict,
                _ => SafeSearch::Moderate,
            });

        let params = SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::Images),
            safe_search,
            max_results: Some(max_results),
            ..Default::default()
        };

        match self.client.search(params).await {
            Ok(response) => {
                let items: Vec<_> = response.results.iter()
                    .map(|r| self.convert_result(r))
                    .collect();
                
                let formatted = format_image_results(query, &items);

                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: formatted }],
                    is_error: Some(false),
                })
            }
            Err(e) => Ok(error_result(format!("Image search failed: {}", e))),
        }
    }
}

/// News search tool
pub struct NewsSearchTool {
    client: Arc<SearXNGClient>,
}

impl NewsSearchTool {
    pub fn new() -> Self {
        Self {
            client: Arc::new(SearXNGClient::new()),
        }
    }

    fn convert_result(&self, result: &SearchResult) -> super::NewsResultItem {
        super::NewsResultItem {
            title: result.title.clone(),
            url: result.url.clone(),
            snippet: result.content.clone(),
            published_date: result.published_date.clone(),
            source: result.engine.clone(),
            engines: Some(result.engines.clone()),
        }
    }
}

impl Default for NewsSearchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolExecutor for NewsSearchTool {
    fn name(&self) -> &str {
        "news_search"
    }

    fn get_definition(&self) -> Tool {
        Tool {
            name: "news_search".to_string(),
            description: concat!(
                "Search for news articles using SearXNG. Aggregates news from multiple sources ",
                "including Bing News, Google News, and more. Supports recency filtering and ",
                "returns publication dates."
            ).to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The news search query"
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum results (1-50, default: 10)",
                        "minimum": 1,
                        "maximum": 50,
                        "default": 10
                    },
                    "time_range": {
                        "type": "string",
                        "description": "Recency filter: 'day', 'week', 'month', 'year'",
                        "enum": ["day", "week", "month", "year"]
                    },
                    "language": {
                        "type": "string",
                        "description": "Language code (e.g., 'en', 'en-US', 'de')"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn execute(&self, args: Value) -> anyhow::Result<CallToolResult> {
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: query"))?;

        let max_results = args
            .get("max_results")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(10)
            .clamp(1, 50);

        let time_range = args
            .get("time_range")
            .and_then(|v| v.as_str())
            .and_then(|v| match v {
                "day" => Some(TimeRange::Day),
                "week" => Some(TimeRange::Week),
                "month" => Some(TimeRange::Month),
                "year" => Some(TimeRange::Year),
                _ => None,
            });

        let language = args.get("language").and_then(|v| v.as_str().map(|s| s.to_string()));

        let params = SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::News),
            time_range,
            language,
            max_results: Some(max_results),
            ..Default::default()
        };

        match self.client.search(params).await {
            Ok(response) => {
                let items: Vec<_> = response.results.iter()
                    .map(|r| self.convert_result(r))
                    .collect();
                
                let formatted = format_news_results(query, &items);

                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: formatted }],
                    is_error: Some(false),
                })
            }
            Err(e) => Ok(error_result(format!("News search failed: {}", e))),
        }
    }
}

/// Video search tool
pub struct VideoSearchTool {
    client: Arc<SearXNGClient>,
}

impl VideoSearchTool {
    pub fn new() -> Self {
        Self {
            client: Arc::new(SearXNGClient::new()),
        }
    }

    fn convert_result(&self, result: &SearchResult) -> super::VideoResultItem {
        super::VideoResultItem {
            title: result.title.clone(),
            url: result.url.clone(),
            thumbnail_url: result.thumbnail.clone(),
            description: result.content.clone(),
            engines: Some(result.engines.clone()),
        }
    }
}

impl Default for VideoSearchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolExecutor for VideoSearchTool {
    fn name(&self) -> &str {
        "video_search"
    }

    fn get_definition(&self) -> Tool {
        Tool {
            name: "video_search".to_string(),
            description: concat!(
                "Search for videos using SearXNG. Searches across YouTube, Bing Videos, ",
                "and other video platforms. Returns video URLs, thumbnails, and descriptions."
            ).to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The video search query"
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum results (1-50, default: 10)",
                        "minimum": 1,
                        "maximum": 50,
                        "default": 10
                    },
                    "safe_search": {
                        "type": "string",
                        "description": "Safe search level",
                        "enum": ["none", "moderate", "strict"],
                        "default": "moderate"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn execute(&self, args: Value) -> anyhow::Result<CallToolResult> {
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: query"))?;

        let max_results = args
            .get("max_results")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(10)
            .clamp(1, 50);

        let safe_search = args
            .get("safe_search")
            .and_then(|v| v.as_str())
            .map(|v| match v {
                "none" => SafeSearch::None,
                "strict" => SafeSearch::Strict,
                _ => SafeSearch::Moderate,
            });

        let params = SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::Videos),
            safe_search,
            max_results: Some(max_results),
            ..Default::default()
        };

        match self.client.search(params).await {
            Ok(response) => {
                let items: Vec<_> = response.results.iter()
                    .map(|r| self.convert_result(r))
                    .collect();
                
                let formatted = format_video_results(query, &items);

                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: formatted }],
                    is_error: Some(false),
                })
            }
            Err(e) => Ok(error_result(format!("Video search failed: {}", e))),
        }
    }
}

/// Suggestions/Autocomplete tool
pub struct SuggestionsTool {
    client: Arc<SearXNGClient>,
}

impl SuggestionsTool {
    pub fn new() -> Self {
        Self {
            client: Arc::new(SearXNGClient::new()),
        }
    }
}

impl Default for SuggestionsTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolExecutor for SuggestionsTool {
    fn name(&self) -> &str {
        "search_suggestions"
    }

    fn get_definition(&self) -> Tool {
        Tool {
            name: "search_suggestions".to_string(),
            description: concat!(
                "Get search query suggestions/autocomplete. Provides query completions ",
                "based on popular searches. Useful for refining search queries or ",
                "discovering related topics."
            ).to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The partial query to get suggestions for"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn execute(&self, args: Value) -> anyhow::Result<CallToolResult> {
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: query"))?;

        match self.client.autocomplete(query).await {
            Ok(suggestions) => {
                let formatted = format_suggestions(query, &suggestions);
                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: formatted }],
                    is_error: Some(false),
                })
            }
            Err(e) => Ok(error_result(format!("Failed to get suggestions: {}", e))),
        }
    }
}

/// Technical/Code search tool
pub struct TechnicalSearchTool {
    client: Arc<SearXNGClient>,
}

impl TechnicalSearchTool {
    pub fn new() -> Self {
        Self {
            client: Arc::new(SearXNGClient::new()),
        }
    }

    fn convert_result(&self, result: &SearchResult) -> super::SearchResultItem {
        super::SearchResultItem {
            title: result.title.clone(),
            url: result.url.clone(),
            snippet: result.content.clone(),
            published_date: result.published_date.clone(),
            engines: Some(result.engines.clone()),
            score: result.score,
            category: result.category.clone(),
        }
    }
}

impl Default for TechnicalSearchTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ToolExecutor for TechnicalSearchTool {
    fn name(&self) -> &str {
        "technical_search"
    }

    fn get_definition(&self) -> Tool {
        Tool {
            name: "technical_search".to_string(),
            description: concat!(
                "Search for technical content, code, and documentation using SearXNG. ",
                "Optimized for finding programming resources, API documentation, GitHub repos, ",
                "Stack Overflow answers, and technical articles."
            ).to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The technical search query (code, API, documentation)"
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum results (1-50, default: 10)",
                        "minimum": 1,
                        "maximum": 50,
                        "default": 10
                    },
                    "time_range": {
                        "type": "string",
                        "description": "Time filter: 'day', 'week', 'month', 'year'",
                        "enum": ["day", "week", "month", "year"]
                    },
                    "language": {
                        "type": "string",
                        "description": "Programming language or technology (e.g., 'rust', 'python', 'javascript')"
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn execute(&self, args: Value) -> anyhow::Result<CallToolResult> {
        let query = args
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing required parameter: query"))?;

        let max_results = args
            .get("max_results")
            .and_then(|v| v.as_u64())
            .map(|v| v as usize)
            .unwrap_or(10)
            .clamp(1, 50);

        let time_range = args
            .get("time_range")
            .and_then(|v| v.as_str())
            .and_then(|v| match v {
                "day" => Some(TimeRange::Day),
                "week" => Some(TimeRange::Week),
                "month" => Some(TimeRange::Month),
                "year" => Some(TimeRange::Year),
                _ => None,
            });

        // Prepend language filter if specified
        let final_query = if let Some(lang) = args.get("language").and_then(|v| v.as_str()) {
            format!("{} {}", query, lang)
        } else {
            query.to_string()
        };

        let params = SearchParams {
            query: final_query,
            category: Some(SearchCategory::It),
            time_range,
            max_results: Some(max_results),
            ..Default::default()
        };

        match self.client.search(params).await {
            Ok(response) => {
                let items: Vec<_> = response.results.iter()
                    .map(|r| self.convert_result(r))
                    .collect();
                
                let formatted = format_search_results(query, &items, response.number_of_results);

                Ok(CallToolResult {
                    content: vec![ToolContent::Text { text: formatted }],
                    is_error: Some(false),
                })
            }
            Err(e) => Ok(error_result(format!("Technical search failed: {}", e))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_tool_name() {
        let tool = SearchTool::new();
        assert_eq!(tool.name(), "web_search");
    }

    #[test]
    fn test_image_search_tool_name() {
        let tool = ImageSearchTool::new();
        assert_eq!(tool.name(), "image_search");
    }

    #[test]
    fn test_news_search_tool_name() {
        let tool = NewsSearchTool::new();
        assert_eq!(tool.name(), "news_search");
    }

    #[test]
    fn test_video_search_tool_name() {
        let tool = VideoSearchTool::new();
        assert_eq!(tool.name(), "video_search");
    }

    #[test]
    fn test_suggestions_tool_name() {
        let tool = SuggestionsTool::new();
        assert_eq!(tool.name(), "search_suggestions");
    }

    #[test]
    fn test_technical_search_tool_name() {
        let tool = TechnicalSearchTool::new();
        assert_eq!(tool.name(), "technical_search");
    }
}
