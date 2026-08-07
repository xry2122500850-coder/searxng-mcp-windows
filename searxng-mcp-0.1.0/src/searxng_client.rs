//! SearXNG API Client
//!
//! A comprehensive client for the SearXNG API that exposes all search capabilities.

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// SearXNG API Client
pub struct SearXNGClient {
    client: reqwest::Client,
    base_url: String,
}

/// Search categories supported by SearXNG
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchCategory {
    General,
    Images,
    Videos,
    News,
    Music,
    Files,
    It,
    Science,
    SocialMedia,
}

impl SearchCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchCategory::General => "general",
            SearchCategory::Images => "images",
            SearchCategory::Videos => "videos",
            SearchCategory::News => "news",
            SearchCategory::Music => "music",
            SearchCategory::Files => "files",
            SearchCategory::It => "it",
            SearchCategory::Science => "science",
            SearchCategory::SocialMedia => "social media",
        }
    }
}

/// Time range filters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeRange {
    Day,
    Week,
    Month,
    Year,
}

impl TimeRange {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeRange::Day => "day",
            TimeRange::Week => "week",
            TimeRange::Month => "month",
            TimeRange::Year => "year",
        }
    }
}

/// Safe search levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeSearch {
    None = 0,
    Moderate = 1,
    Strict = 2,
}

impl SafeSearch {
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

/// Image format/size filters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImageType {
    All,
    Photo,
    Clipart,
    LineDrawing,
    Gif,
    Transparent,
}

/// Video duration filters
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VideoDuration {
    Short,    // < 4 min
    Medium,   // 4-20 min
    Long,     // > 20 min
}

impl VideoDuration {
    pub fn as_str(&self) -> &'static str {
        match self {
            VideoDuration::Short => "short",
            VideoDuration::Medium => "medium",
            VideoDuration::Long => "long",
        }
    }
}

/// Search query parameters
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub query: String,
    pub category: Option<SearchCategory>,
    pub language: Option<String>,
    pub time_range: Option<TimeRange>,
    pub safe_search: Option<SafeSearch>,
    pub page: Option<u32>,
    pub engines: Option<Vec<String>>,
    pub max_results: Option<usize>,
}

/// A single search result
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub url: String,
    pub title: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub engine: Option<String>,
    #[serde(default)]
    pub engines: Vec<String>,
    #[serde(default)]
    pub score: Option<f64>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    #[serde(rename = "publishedDate")]
    pub published_date: Option<String>,
    #[serde(default, rename = "img_src")]
    pub image_source: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub template: Option<String>,
}

/// Search response from SearXNG
#[derive(Debug, Clone, Deserialize)]
pub struct SearchResponse {
    pub query: String,
    #[serde(default)]
    pub number_of_results: Option<i32>,
    #[serde(default)]
    pub results: Vec<SearchResult>,
    #[serde(default)]
    pub suggestions: Vec<String>,
    #[serde(default)]
    pub unresponsive_engines: Vec<Vec<String>>,
}

/// Engine information
#[derive(Debug, Clone, Deserialize)]
pub struct EngineInfo {
    pub name: String,
    pub engine: String,
    pub categories: Vec<String>,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub shortcut: Option<String>,
}

/// Autocomplete suggestion
#[derive(Debug, Clone, Deserialize)]
pub struct AutocompleteResponse {
    #[serde(flatten)]
    pub suggestions: Vec<String>,
}

impl SearXNGClient {
    /// Create a new SearXNG client
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(45))
            .connect_timeout(Duration::from_secs(15))
            .pool_idle_timeout(Duration::from_secs(60))
            .build()
            .expect("Failed to create HTTP client");

        let base_url = std::env::var("SEARXNG_URL")
            .or_else(|_| std::env::var("SEARXNG_ENDPOINT"))
            .unwrap_or_else(|_| "http://localhost:8888".to_string());

        tracing::info!("SearXNG client initialized with endpoint: {}", base_url);

        Self { client, base_url }
    }

    /// Get the base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Perform a search with comprehensive parameters
    pub async fn search(&self, params: SearchParams) -> anyhow::Result<SearchResponse> {
        let mut url = format!("{}/search", self.base_url);
        
        // Build query parameters
        let mut query_parts = vec![
            format!("q={}", urlencoding::encode(&params.query)),
            "format=json".to_string(),
        ];

        // Add optional parameters
        if let Some(category) = params.category {
            query_parts.push(format!("category={}", category.as_str()));
        }

        if let Some(lang) = params.language {
            query_parts.push(format!("language={}", lang));
        }

        if let Some(time_range) = params.time_range {
            query_parts.push(format!("time_range={}", time_range.as_str()));
        }

        if let Some(safe_search) = params.safe_search {
            query_parts.push(format!("safesearch={}", safe_search.as_u8()));
        }

        if let Some(page) = params.page {
            query_parts.push(format!("pageno={}", page));
        }

        if let Some(engines) = params.engines {
            if !engines.is_empty() {
                query_parts.push(format!("engines={}", engines.join(",")));
            }
        }

        url.push('?');
        url.push_str(&query_parts.join("&"));

        tracing::debug!("SearXNG search URL: {}", url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .header("User-Agent", "searxng-mcp/1.0")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("SearXNG returned error {}: {}", status, body);
        }

        let mut search_response: SearchResponse = response.json().await?;

        // Limit results if requested
        if let Some(max) = params.max_results {
            if search_response.results.len() > max {
                search_response.results.truncate(max);
            }
        }

        tracing::debug!(
            "Search found {} results for query: {}",
            search_response.results.len(),
            search_response.query
        );

        Ok(search_response)
    }

    /// Quick web search
    pub async fn web_search(
        &self,
        query: &str,
        max_results: usize,
    ) -> anyhow::Result<SearchResponse> {
        self.search(SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::General),
            max_results: Some(max_results),
            ..Default::default()
        })
        .await
    }

    /// Image search
    pub async fn image_search(
        &self,
        query: &str,
        max_results: usize,
    ) -> anyhow::Result<SearchResponse> {
        self.search(SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::Images),
            max_results: Some(max_results),
            ..Default::default()
        })
        .await
    }

    /// News search
    pub async fn news_search(
        &self,
        query: &str,
        max_results: usize,
        time_range: Option<TimeRange>,
    ) -> anyhow::Result<SearchResponse> {
        self.search(SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::News),
            time_range,
            max_results: Some(max_results),
            ..Default::default()
        })
        .await
    }

    /// Video search
    pub async fn video_search(
        &self,
        query: &str,
        max_results: usize,
    ) -> anyhow::Result<SearchResponse> {
        self.search(SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::Videos),
            max_results: Some(max_results),
            ..Default::default()
        })
        .await
    }

    /// Technical/IT search (code, documentation)
    pub async fn technical_search(
        &self,
        query: &str,
        max_results: usize,
    ) -> anyhow::Result<SearchResponse> {
        self.search(SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::It),
            max_results: Some(max_results),
            ..Default::default()
        })
        .await
    }

    /// Science/academic search
    pub async fn science_search(
        &self,
        query: &str,
        max_results: usize,
    ) -> anyhow::Result<SearchResponse> {
        self.search(SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::Science),
            max_results: Some(max_results),
            ..Default::default()
        })
        .await
    }

    /// Social media search
    pub async fn social_search(
        &self,
        query: &str,
        max_results: usize,
    ) -> anyhow::Result<SearchResponse> {
        self.search(SearchParams {
            query: query.to_string(),
            category: Some(SearchCategory::SocialMedia),
            max_results: Some(max_results),
            ..Default::default()
        })
        .await
    }

    /// Get autocomplete suggestions
    pub async fn autocomplete(&self, query: &str) -> anyhow::Result<Vec<String>> {
        let url = format!(
            "{}/autocompleter?q={}",
            self.base_url,
            urlencoding::encode(query)
        );

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            return Ok(Vec::new());
        }

        // SearXNG returns suggestions in format: [query, [suggestion1, suggestion2, ...]]
        let data: serde_json::Value = response.json().await?;
        
        if let Some(array) = data.as_array() {
            if array.len() > 1 {
                if let Some(suggestions) = array[1].as_array() {
                    return Ok(suggestions
                        .iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect());
                }
            }
        }

        Ok(Vec::new())
    }

    /// Get available engines
    pub async fn get_engines(&self) -> anyhow::Result<Vec<EngineInfo>> {
        let url = format!("{}/config", self.base_url);

        let response = self
            .client
            .get(&url)
            .header("Accept", "application/json")
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to get engine configuration");
        }

        let config: serde_json::Value = response.json().await?;
        
        let engines: Vec<EngineInfo> = config
            .get("engines")
            .and_then(|e| e.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| serde_json::from_value(v.clone()).ok())
                    .collect()
            })
            .unwrap_or_default();

        Ok(engines)
    }

    /// Get enabled engines for a category
    pub async fn get_engines_for_category(
        &self,
        category: &str,
    ) -> anyhow::Result<Vec<String>> {
        let engines = self.get_engines().await?;
        
        let filtered: Vec<String> = engines
            .into_iter()
            .filter(|e| e.enabled && e.categories.iter().any(|c| c == category))
            .map(|e| e.name)
            .collect();

        Ok(filtered)
    }

    /// Health check
    pub async fn health_check(&self) -> anyhow::Result<bool> {
        let url = format!("{}/healthz", self.base_url);
        
        match self
            .client
            .get(&url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
}

impl Default for SearXNGClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_as_str() {
        assert_eq!(SearchCategory::General.as_str(), "general");
        assert_eq!(SearchCategory::Images.as_str(), "images");
        assert_eq!(SearchCategory::News.as_str(), "news");
    }

    #[test]
    fn test_time_range_as_str() {
        assert_eq!(TimeRange::Day.as_str(), "day");
        assert_eq!(TimeRange::Week.as_str(), "week");
        assert_eq!(TimeRange::Month.as_str(), "month");
        assert_eq!(TimeRange::Year.as_str(), "year");
    }

    #[test]
    fn test_safe_search_values() {
        assert_eq!(SafeSearch::None.as_u8(), 0);
        assert_eq!(SafeSearch::Moderate.as_u8(), 1);
        assert_eq!(SafeSearch::Strict.as_u8(), 2);
    }
}
