//! MCP Tools for SearXNG Search
//!
//! Comprehensive search tools that rival Brave Search API:
//! - web_search: General web search with advanced filters
//! - image_search: Image search with size/type filters
//! - news_search: News search with recency filters
//! - video_search: Video search
//! - suggestions: Autocomplete/suggestions
//! - technical_search: Code and documentation search
//! - social_search: Social media search

use crate::mcp::{CallToolResult, Tool, ToolContent};
use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;

mod fetch;
mod search;

pub use fetch::FetchTool;
pub use search::{SearchTool, ImageSearchTool, NewsSearchTool, VideoSearchTool, SuggestionsTool, TechnicalSearchTool};

/// Trait for tool execution
#[async_trait]
pub trait ToolExecutor: Send + Sync {
    /// Get the tool name
    fn name(&self) -> &str;

    /// Get the tool definition for MCP
    fn get_definition(&self) -> Tool;

    /// Execute the tool with given arguments
    async fn execute(&self, args: Value) -> anyhow::Result<CallToolResult>;
}

/// Helper function to create text content result
pub fn text_result(text: impl Into<String>) -> CallToolResult {
    CallToolResult {
        content: vec![ToolContent::Text { text: text.into() }],
        is_error: Some(false),
    }
}

/// Helper function to create error result
pub fn error_result(error: impl Into<String>) -> CallToolResult {
    CallToolResult {
        content: vec![ToolContent::Text { text: error.into() }],
        is_error: Some(true),
    }
}

/// Format search results as markdown
pub fn format_search_results(query: &str, results: &[SearchResultItem], total: Option<i32>) -> String {
    let mut output = format!("## Search Results for: \"{}\"\n\n", query);

    if let Some(total) = total {
        output.push_str(&format!("Estimated total results: {}\n\n", total));
    }

    if results.is_empty() {
        output.push_str("*No results found.*\n");
        return output;
    }

    output.push_str(&format!("Showing {} results:\n\n", results.len()));

    for (i, result) in results.iter().enumerate() {
        output.push_str(&format!(
            "### {}. {}\n\n",
            i + 1,
            result.title
        ));
        
        output.push_str(&format!("**URL:** {}\n\n", result.url));
        
        if let Some(snippet) = &result.snippet {
            output.push_str(&format!("{}\n\n", snippet));
        }
        
        // Add metadata if available
        let mut meta_parts = Vec::new();
        
        if let Some(date) = &result.published_date {
            meta_parts.push(format!("📅 {}", date));
        }
        
        if let Some(engines) = &result.engines {
            if !engines.is_empty() {
                meta_parts.push(format!("🔍 Found by: {}", engines.join(", ")));
            }
        }
        
        if let Some(score) = result.score {
            meta_parts.push(format!("⭐ Score: {:.1}", score));
        }
        
        if let Some(category) = &result.category {
            meta_parts.push(format!("📂 Category: {}", category));
        }
        
        if !meta_parts.is_empty() {
            output.push_str(&format!("\n*{}*\n", meta_parts.join(" | ")));
        }
        
        output.push_str("\n---\n\n");
    }

    output
}

/// Format image search results
pub fn format_image_results(query: &str, results: &[ImageResultItem]) -> String {
    let mut output = format!("## Image Search Results for: \"{}\"\n\n", query);

    if results.is_empty() {
        output.push_str("*No images found.*\n");
        return output;
    }

    output.push_str(&format!("Found {} images:\n\n", results.len()));

    for (i, result) in results.iter().enumerate() {
        output.push_str(&format!(
            "### {}. {}\n\n",
            i + 1,
            result.title
        ));
        
        output.push_str(&format!("**Image URL:** {}\n\n", result.image_url));
        output.push_str(&format!("**Source:** {}\n\n", result.source_url));
        
        if let Some(thumbnail) = &result.thumbnail_url {
            output.push_str(&format!("**Thumbnail:** {}\n\n", thumbnail));
        }
        
        if let Some(engines) = &result.engines {
            if !engines.is_empty() {
                output.push_str(&format!("\n*🔍 Found by: {}*\n", engines.join(", ")));
            }
        }
        
        output.push_str("\n---\n\n");
    }

    output
}

/// Format video search results
pub fn format_video_results(query: &str, results: &[VideoResultItem]) -> String {
    let mut output = format!("## Video Search Results for: \"{}\"\n\n", query);

    if results.is_empty() {
        output.push_str("*No videos found.*\n");
        return output;
    }

    output.push_str(&format!("Found {} videos:\n\n", results.len()));

    for (i, result) in results.iter().enumerate() {
        output.push_str(&format!(
            "### {}. {}\n\n",
            i + 1,
            result.title
        ));
        
        output.push_str(&format!("**URL:** {}\n\n", result.url));
        
        if let Some(thumbnail) = &result.thumbnail_url {
            output.push_str(&format!("**Thumbnail:** {}\n\n", thumbnail));
        }
        
        if let Some(description) = &result.description {
            output.push_str(&format!("{}\n\n", description));
        }
        
        if let Some(engines) = &result.engines {
            if !engines.is_empty() {
                output.push_str(&format!("*🔍 Found by: {}*\n", engines.join(", ")));
            }
        }
        
        output.push_str("\n---\n\n");
    }

    output
}

/// Format news results with dates
pub fn format_news_results(query: &str, results: &[NewsResultItem]) -> String {
    let mut output = format!("## News Results for: \"{}\"\n\n", query);

    if results.is_empty() {
        output.push_str("*No news articles found.*\n");
        return output;
    }

    output.push_str(&format!("Found {} news articles:\n\n", results.len()));

    for (i, result) in results.iter().enumerate() {
        output.push_str(&format!(
            "### {}. {}\n\n",
            i + 1,
            result.title
        ));
        
        output.push_str(&format!("**URL:** {}\n\n", result.url));
        
        if let Some(date) = &result.published_date {
            output.push_str(&format!("**Published:** 📅 {}\n\n", date));
        }
        
        if let Some(snippet) = &result.snippet {
            output.push_str(&format!("{}\n\n", snippet));
        }
        
        if let Some(source) = &result.source {
            output.push_str(&format!("*📰 Source: {}*\n", source));
        }
        
        output.push_str("\n---\n\n");
    }

    output
}

/// Format suggestions
pub fn format_suggestions(query: &str, suggestions: &[String]) -> String {
    let mut output = format!("## Search Suggestions for: \"{}\"\n\n", query);

    if suggestions.is_empty() {
        output.push_str("*No suggestions available.*\n");
        return output;
    }

    output.push_str(&format!("Found {} suggestions:\n\n", suggestions.len()));

    for (i, suggestion) in suggestions.iter().enumerate() {
        output.push_str(&format!("{}. {}\n", i + 1, suggestion));
    }

    output
}

/// Generic search result item
#[derive(Debug, Clone, Serialize)]
pub struct SearchResultItem {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
    pub published_date: Option<String>,
    pub engines: Option<Vec<String>>,
    pub score: Option<f64>,
    pub category: Option<String>,
}

/// Image result item
#[derive(Debug, Clone, Serialize)]
pub struct ImageResultItem {
    pub title: String,
    pub image_url: String,
    pub source_url: String,
    pub thumbnail_url: Option<String>,
    pub engines: Option<Vec<String>>,
}

/// Video result item
#[derive(Debug, Clone, Serialize)]
pub struct VideoResultItem {
    pub title: String,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub description: Option<String>,
    pub engines: Option<Vec<String>>,
}

/// News result item
#[derive(Debug, Clone, Serialize)]
pub struct NewsResultItem {
    pub title: String,
    pub url: String,
    pub snippet: Option<String>,
    pub published_date: Option<String>,
    pub source: Option<String>,
    pub engines: Option<Vec<String>>,
}

/// Parse HTML content to extract main text content
pub fn html_to_markdown(html: &str) -> String {
    // First convert HTML to text using html2text
    let text = html2text::from_read(html.as_bytes(), 80);

    // Clean up whitespace
    let lines: Vec<&str> = text.lines().collect();
    let mut cleaned = Vec::new();
    let mut prev_empty = false;

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !prev_empty {
                cleaned.push("");
                prev_empty = true;
            }
        } else {
            cleaned.push(trimmed);
            prev_empty = false;
        }
    }

    cleaned.join("\n")
}

/// Extract the main content from HTML using heuristics
pub fn extract_main_content(html: &str) -> String {
    use scraper::{Html, Selector};

    let document = Html::parse_document(html);

    // Try common content selectors
    let content_selectors = [
        "main",
        "article",
        "[role='main']",
        ".content",
        "#content",
        ".post-content",
        ".entry-content",
        ".article-content",
        "#main-content",
    ];

    for selector_str in &content_selectors {
        if let Ok(selector) = Selector::parse(selector_str) {
            if let Some(element) = document.select(&selector).next() {
                let html = element.html();
                return html_to_markdown(&html);
            }
        }
    }

    // Fallback: convert the whole body
    if let Ok(body_selector) = Selector::parse("body") {
        if let Some(body) = document.select(&body_selector).next() {
            let body_html = body.html();
            return html_to_markdown(&body_html);
        }
    }

    // Final fallback
    html_to_markdown(html)
}
