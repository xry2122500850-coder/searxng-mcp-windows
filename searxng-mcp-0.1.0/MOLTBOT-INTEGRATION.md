# SearXNG MCP - Moltbot/Clawdbot Integration

This guide explains how to configure **moltbot** (the engine behind clawdbot) to use **SearXNG MCP** as its default web search provider instead of Brave Search API.

## Overview

By default, moltbot uses Brave Search API for web search. This integration replaces it with SearXNG MCP, providing:

- **89+ search engines** aggregated (Google, Bing, DuckDuckGo, Brave, etc.)
- **Self-hosted** - No API keys or usage limits
- **Free** - No paid plans required
- **Privacy-focused** - Your searches stay local
- **More features** - News, images, videos, technical search

## Architecture

```
┌─────────────┐     ┌──────────────────┐     ┌──────────────┐
│   Moltbot   │────▶│  moltbot-web-tool │────▶│ SearXNG MCP  │
│  (clawdbot) │     │   (wrapper)       │     │   Server     │
└─────────────┘     └──────────────────┘     └──────────────┘
                                                      │
                                                      ▼
                                               ┌──────────────┐
                                               │ SearXNG      │
                                               │ Docker       │
                                               └──────────────┘
```

## Installation

### 1. Install SearXNG MCP Server

If you haven't already:

```bash
cd /home/jackson/dev/searxng-mcp
cargo build --release
```

### 2. Install Wrapper Scripts

The following scripts have been installed to `~/.local/bin/`:

- `searxng-mcp-launcher` - Auto-starts Docker and MCP server
- `moltbot-web-tool` - Moltbot-compatible web search/fetch interface
- `searxng-moltbot-bridge` - Node.js bridge (alternative)

Verify installation:

```bash
ls -la ~/.local/bin/moltbot-web-tool
ls -la ~/.local/bin/searxng-*
```

### 3. Configure Moltbot

#### Option A: Direct Configuration (Recommended)

Create or edit `~/.moltbot/config.json`:

```json
{
  "version": 1,
  "tools": {
    "web": {
      "search": {
        "enabled": true,
        "provider": "searxng",
        "apiKey": "not-required",
        "maxResults": 10,
        "timeoutSeconds": 45,
        "cacheTtlMinutes": 15
      },
      "fetch": {
        "enabled": true,
        "maxChars": 50000,
        "timeoutSeconds": 30,
        "cacheTtlMinutes": 15,
        "readability": true
      }
    }
  },
  "preferences": {
    "defaultSearchProvider": "searxng"
  }
}
```

#### Option B: Environment Variables

Add to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
# Disable Brave Search
export BRAVE_API_KEY=""

# Configure SearXNG
export SEARXNG_URL="http://localhost:8888"
export SEARXNG_CACHE_TTL="15"
```

#### Option C: Using moltbot configure Command

If moltbot provides a configure command:

```bash
# Configure to use custom provider
moltbot configure --section web

# Set provider to 'custom' or edit config directly
# Then point web_search to: /home/jackson/.local/bin/moltbot-web-tool
```

### 4. Configure Tool Aliases (if needed)

If moltbot expects specific command names, create symlinks:

```bash
# If moltbot looks for 'web_search' command
ln -sf ~/.local/bin/moltbot-web-tool ~/.local/bin/web_search

# If moltbot looks for 'web_fetch' command  
ln -sf ~/.local/bin/moltbot-web-tool ~/.local/bin/web_fetch
```

## Usage

### Command Line Interface

The `moltbot-web-tool` provides a CLI compatible with moltbot's expectations:

```bash
# Search the web
moltbot-web-tool search "machine learning" 10

# Search with time filter (day, week, month, year)
moltbot-web-tool search "latest news" 5 week

# Search with language
moltbot-web-tool search "programmation" 10 "" fr

# Fetch URL content
moltbot-web-tool fetch "https://example.com" 5000
```

### JSON Output Format

Results are returned in JSON format compatible with moltbot:

```json
{
  "query": "machine learning",
  "results": [
    {
      "title": "Machine Learning - Wikipedia",
      "url": "https://en.wikipedia.org/wiki/Machine_learning",
      "snippet": "Machine learning is a field of study..."
    }
  ],
  "provider": "searxng",
  "cached": false
}
```

## Tool Comparison

### Web Search Parameters

| Parameter | Brave API | SearXNG MCP | Notes |
|-----------|-----------|-------------|-------|
| `query` | ✅ | ✅ | Search query string |
| `count` | ✅ | ✅ | Number of results (1-50) |
| `country` | ✅ | ❌ | Use `search_lang` instead |
| `search_lang` | ✅ | ✅ | Language code (en, de, fr) |
| `freshness` | ✅ | ✅ | day, week, month, year |
| `offset` | ✅ | ❌ | Use `page` for pagination |

### Web Fetch Parameters

| Parameter | Brave API | SearXNG MCP | Notes |
|-----------|-----------|-------------|-------|
| `url` | ✅ | ✅ | URL to fetch |
| `maxChars` | ✅ | ✅ | Maximum characters |
| `extractMode` | ✅ | ✅ | markdown or text |

## Advanced Configuration

### Custom SearXNG Instance

If you have a remote SearXNG instance:

```bash
export SEARXNG_URL="https://your-searxng-instance.com"
```

### Cache Configuration

Adjust cache TTL (in minutes):

```bash
export SEARXNG_CACHE_TTL="30"  # Cache for 30 minutes
```

### Debug Mode

Enable debug logging:

```bash
export MOLTBOT_DEBUG="1"
moltbot-web-tool search "test"
```

## Troubleshooting

### SearXNG Not Starting

Check Docker:

```bash
docker ps | grep searxng
docker logs searxng-moltbot-web
```

### No Search Results

1. Verify SearXNG is healthy:
   ```bash
   curl http://localhost:8888/healthz
   ```

2. Check engine configuration:
   ```bash
   curl http://localhost:8888/config | jq '.engines[] | select(.enabled) | .name'
   ```

3. Test direct search:
   ```bash
   curl "http://localhost:8888/search?q=test&format=json" | jq '.results | length'
   ```

### Tool Not Found

Ensure `~/.local/bin` is in your PATH:

```bash
echo $PATH | grep -o "/home/jackson/.local/bin"

# If not, add to ~/.bashrc or ~/.zshrc:
export PATH="$HOME/.local/bin:$PATH"
```

### Permission Denied

Fix permissions:

```bash
chmod +x ~/.local/bin/moltbot-web-tool
chmod +x ~/.local/bin/searxng-mcp-launcher
```

## Integration with Clawdbot

Since clawdbot uses moltbot as its engine, the same configuration applies:

1. **Locate clawdbot config**:
   ```bash
   ls -la ~/.clawdbot/
   ```

2. **Create/update moltbot config**:
   ```bash
   mkdir -p ~/.moltbot
   cp /home/jackson/dev/searxng-mcp/moltbot-config.json ~/.moltbot/config.json
   ```

3. **Restart clawdbot** to pick up changes

## Migration from Brave Search

To migrate existing configurations:

### Before (Brave Search)

```json
{
  "tools": {
    "web": {
      "search": {
        "enabled": true,
        "provider": "brave",
        "apiKey": "YOUR_BRAVE_API_KEY"
      }
    }
  }
}
```

### After (SearXNG MCP)

```json
{
  "tools": {
    "web": {
      "search": {
        "enabled": true,
        "provider": "searxng",
        "apiKey": "not-required",
        "searxng": {
          "baseUrl": "http://localhost:8888"
        }
      }
    }
  }
}
```

## Performance Considerations

- **First search**: May take 30-60 seconds as Docker container starts
- **Subsequent searches**: ~1-3 seconds (cached for 15 minutes by default)
- **Parallel queries**: SearXNG can handle multiple concurrent searches
- **Memory usage**: Docker container uses ~200-500MB RAM

## Security Notes

- SearXNG runs locally - searches don't leave your machine
- No API keys to leak
- Docker container runs with limited privileges
- Cache files stored in `~/.cache/moltbot-web-tool/`

## Additional Features vs Brave

SearXNG MCP provides additional tools not available in Brave Search:

- `image_search` - Search for images
- `news_search` - News-specific search with recency filters
- `video_search` - Video search
- `technical_search` - Code and documentation search
- `search_suggestions` - Autocomplete

To use these, call the MCP server directly or extend the wrapper.

## Support

For issues with:
- **SearXNG MCP**: Check logs in `~/.cache/searxng-mcp/`
- **Moltbot integration**: Check `~/.cache/moltbot-web-tool/`
- **Docker issues**: Run `docker logs searxng-moltbot-web`

---

**Note**: This integration is designed to be a drop-in replacement for Brave Search API in moltbot. If moltbot's interface changes, the wrapper scripts may need updates.
