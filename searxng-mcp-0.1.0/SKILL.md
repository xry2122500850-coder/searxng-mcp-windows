# SKILL: Web Search & Research with SearXNG MCP

> **Purpose**: Comprehensive guide for conducting effective web searches and research using the SearXNG MCP server tools.

## Overview

The SearXNG MCP server provides 7 powerful search tools that aggregate results from 89+ search engines:

- `web_search` - General web search with advanced filters
- `image_search` - Image search  
- `news_search` - News articles with recency filters
- `video_search` - Video search
- `search_suggestions` - Query autocomplete
- `technical_search` - Code and documentation search
- `fetch_url` - Extract content from URLs

---

## Core Principles

### 1. Choose the Right Tool

| What you need | Tool to use |
|--------------|-------------|
| General information | `web_search` |
| Current events, news | `news_search` with `time_range: "day"` or `"week"` |
| Images, diagrams | `image_search` |
| Tutorials, talks | `video_search` |
| Code examples, APIs | `technical_search` |
| Refining a query | `search_suggestions` |
| Full article content | `fetch_url` after finding URL |

### 2. Search Strategy: Start Broad, Then Narrow

**Bad**: One specific search with many constraints  
**Good**: Progressive refinement

```
Step 1: web_search({"query": "machine learning frameworks", "max_results": 5})
Step 2: technical_search({"query": "machine learning python", "max_results": 5})
Step 3: news_search({"query": "machine learning 2024", "time_range": "month"})
```

### 3. Use Time Filters for Recency

Always use `time_range` when freshness matters:

| Use case | Time range |
|----------|------------|
| Breaking news | `"day"` |
| Recent developments | `"week"` |
| Current trends | `"month"` |
| Annual reviews | `"year"` |
| Historical context | omit (all time) |

---

## Tool-Specific Patterns

### web_search

**Basic usage**:
```json
{
  "query": "your search terms",
  "max_results": 10
}
```

**Advanced filtering**:
```json
{
  "query": "rust async programming",
  "max_results": 10,
  "time_range": "month",
  "language": "en",
  "safe_search": "moderate",
  "engines": ["google", "bing", "duckduckgo"]
}
```

**Use specific engines when**:
- You need comprehensive results: omit `engines` (uses all)
- Quick results: `["google", "bing"]`
- Privacy-focused: `["duckduckgo", "brave"]`
- Academic: `["google_scholar"]` (if available)

### news_search

**Current events pattern**:
```json
{
  "query": "AI regulation",
  "time_range": "week",
  "max_results": 10
}
```

**Industry monitoring**:
```json
{
  "query": "tech layoffs",
  "time_range": "day",
  "max_results": 20
}
```

### technical_search

**Finding code examples**:
```json
{
  "query": "async await",
  "language": "rust",
  "max_results": 10
}
```

**API documentation**:
```json
{
  "query": "react hooks useEffect",
  "max_results": 10
}
```

**Language-specific search**:
```json
{
  "query": "data structures",
  "language": "python",
  "time_range": "year"
}
```

### image_search

**Finding logos/icons**:
```json
{
  "query": "rust programming logo transparent",
  "max_results": 5
}
```

**Diagrams/infographics**:
```json
{
  "query": "machine learning workflow diagram",
  "max_results": 10
}
```

### fetch_url

**Always use after finding relevant URLs**:

1. Search first: `web_search({"query": "topic", "max_results": 5})`
2. Extract content: `fetch_url({"url": "found_url", "max_length": 5000})`

**Content extraction strategy**:
- For articles: `max_length: 5000-10000`
- For documentation: `max_length: 10000-50000`
- For full pages: omit `max_length` or set high

---

## Multi-Step Research Patterns

### Pattern 1: Deep Dive on Topic

```
1. search_suggestions({"query": "topic"}) → Get related queries
2. web_search({"query": "topic overview", "max_results": 5})
3. For each relevant result:
   - fetch_url({"url": "result_url", "max_length": 3000})
4. news_search({"query": "topic", "time_range": "month"})
5. technical_search({"query": "topic code examples"})
```

### Pattern 2: Compare Technologies

```
1. web_search({"query": "technology A vs technology B", "max_results": 10})
2. For each technology:
   - web_search({"query": "technology A features benefits"})
   - news_search({"query": "technology A", "time_range": "year"})
3. technical_search({"query": "technology A examples", "max_results": 10})
```

### Pattern 3: Stay Updated

```
1. news_search({"query": "your industry", "time_range": "day", "max_results": 20})
2. For interesting articles:
   - fetch_url({"url": "article_url"})
3. web_search({"query": "trending topic details", "time_range": "week"})
```

### Pattern 4: Learning Path

```
1. web_search({"query": "learn topic beginner guide", "max_results": 5})
2. video_search({"query": "topic tutorial", "max_results": 5})
3. technical_search({"query": "topic examples", "max_results": 10})
4. image_search({"query": "topic cheat sheet"})
```

---

## Query Optimization

### Use Quotes for Exact Phrases

```json
{"query": "\"machine learning\" tutorial"}
```

### Use Site Filters (via query)

```json
{"query": "site:github.com rust async"}
{"query": "site:stackoverflow.com python list comprehension"}
```

### Combine with Technical Terms

```json
{"query": "rust programming tutorial beginners"}
{"query": "react typescript best practices 2024"}
```

### Use Suggestions for Refinement

```json
{"query": "programming"} → suggestions
{"query": "programming languages comparison"} → refined
```

---

## Common Mistakes to Avoid

### ❌ Don't

1. **Search once and stop** - Always iterate
2. **Ignore time filters** - Old info for current topics
3. **Use generic queries** - Be specific
4. **Skip content extraction** - URLs don't tell the full story
5. **Use wrong category** - Don't use web_search for news

### ✅ Do

1. **Progressive refinement** - Start broad, narrow down
2. **Time-range awareness** - Use filters appropriately
3. **Specific keywords** - Include relevant technical terms
4. **Extract content** - Use fetch_url for key sources
5. **Right tool for job** - Use specialized searchers

---

## Advanced Techniques

### Cross-Reference Sources

```
1. web_search({"query": "topic", "engines": ["google"]})
2. web_search({"query": "topic", "engines": ["bing"]})
3. Compare results for comprehensive view
```

### Find Primary Sources

```
1. web_search({"query": "topic official documentation"})
2. Look for official domains (.org, .edu, official sites)
3. fetch_url official sources
```

### Track Changes Over Time

```
1. news_search({"query": "topic", "time_range": "year"})
2. Note publication dates
3. Identify trends and developments
```

### Find Code Examples

```
1. technical_search({"query": "topic", "language": "python"})
2. Look for GitHub links
3. fetch_url({"url": "github_readme_or_example"})
```

---

## Output Interpretation

### Understanding Results

Each result includes:
- **Title** - Page/article title
- **URL** - Direct link
- **Snippet** - Description/preview
- **Score** - Relevance ranking (higher = more relevant)
- **Engines** - Which search engines found it
- **Published Date** - For news (when available)
- **Category** - Content classification

### Evaluating Sources

**High credibility indicators**:
- Multiple engines found it (diverse verification)
- High score (relevant to query)
- Official domains (.edu, .gov, known orgs)
- Recent date (for current topics)

**Red flags**:
- Single engine result only
- Very low score
- Unknown domains
- Very old dates (for current topics)

---

## Integration Examples

### Example 1: Research a New Technology

```
User: "Tell me about the latest developments in quantum computing"

1. search_suggestions({"query": "quantum computing"})
2. news_search({
     "query": "quantum computing breakthrough",
     "time_range": "month",
     "max_results": 10
   })
3. web_search({
     "query": "quantum computing explained 2024",
     "max_results": 5
   })
4. For top 2-3 relevant results: fetch_url()
5. technical_search({
     "query": "quantum computing programming",
     "max_results": 5
   })
```

### Example 2: Compare Frameworks

```
User: "Should I use React or Vue for my project?"

1. web_search({
     "query": "React vs Vue 2024 comparison",
     "max_results": 10
   })
2. news_search({
     "query": "React Vue popularity",
     "time_range": "year"
   })
3. technical_search({
     "query": "React tutorial",
     "max_results": 5
   })
4. technical_search({
     "query": "Vue tutorial",
     "max_results": 5
   })
5. image_search({
     "query": "React vs Vue comparison infographic"
   })
```

### Example 3: Debug an Error

```
User: "I'm getting 'undefined is not a function' in JavaScript"

1. technical_search({
     "query": "undefined is not a function javascript",
     "max_results": 10
   })
2. web_search({
     "query": "\"undefined is not a function\" common causes",
     "max_results": 5
   })
3. For relevant Stack Overflow links: fetch_url()
4. video_search({
     "query": "javascript debugging undefined error"
   })
```

---

## Tips for Different Use Cases

### Academic Research

- Use `time_range: "year"` or omit for comprehensive coverage
- Look for `.edu` domains
- Cross-reference multiple sources
- Use `technical_search` for papers/code

### Market Research

- Use `news_search` with recent time ranges
- Monitor competitor mentions
- Use `web_search` with specific industry terms
- Track trends over time

### Learning New Skills

- Start with `video_search` for tutorials
- Use `technical_search` for code examples
- `image_search` for diagrams/cheat sheets
- `web_search` for comprehensive guides

### Current Events

- Always use `news_search` with `time_range: "day"` or `"week"`
- Cross-check with `web_search` for analysis
- Monitor developing stories

---

## Summary Checklist

Before starting research:
- [ ] Choose appropriate tool for query type
- [ ] Consider time sensitivity (use time_range)
- [ ] Plan multi-step approach

During research:
- [ ] Start broad, refine progressively
- [ ] Extract content from key URLs
- [ ] Cross-reference multiple sources

After research:
- [ ] Verify source credibility
- [ ] Check recency of information
- [ ] Synthesize findings from multiple searches

---

## Quick Reference

```
General search:     web_search({query, max_results, time_range, language})
Current news:       news_search({query, time_range: "day/week", max_results})
Images:             image_search({query, max_results})
Videos:             video_search({query, max_results})
Code/docs:          technical_search({query, language, max_results})
Suggestions:        search_suggestions({query})
Content extraction: fetch_url({url, max_length})
```

---

**Remember**: The key to effective research is iteration. Start with a broad search, analyze results, refine your query, and dig deeper into promising sources.
