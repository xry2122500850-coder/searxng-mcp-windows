# Clawdbot Integration Guide

Complete guide for integrating SearXNG MCP with [Clawdbot](https://docs.clawd.bot/), enabling free, private web search without API keys.

## Overview

By default, Clawdbot uses Brave Search API for web search, which requires an API key and has usage limits. This integration replaces it with SearXNG MCP, providing:

- ✅ **89+ search engines** aggregated (Google, Bing, DuckDuckGo, Brave, etc.)
- ✅ **Self-hosted** - No API keys or usage limits
- ✅ **Free forever** - No paid plans required
- ✅ **Privacy-focused** - Your searches stay local
- ✅ **More features** - News, images, videos, technical search

## Quick Start

```bash
# 1. Install SearXNG MCP
cd ~/dev/searxng-mcp
cargo build --release
cp target/release/searxng-mcp ~/.local/bin/

# 2. Install wrapper
cp scripts/searxng-search ~/.local/bin/
chmod +x ~/.local/bin/searxng-search
ln -sf ~/.local/bin/searxng-search ~/.local/bin/moltbot-web-tool

# 3. Start SearXNG
docker run -d --name searxng -p 8888:8080 searxng/searxng:latest

# 4. Configure Clawdbot
echo 'SEARXNG_URL=http://localhost:8888' >> ~/.clawdbot/.env
echo 'BRAVE_API_KEY=""' >> ~/.clawdbot/.env

# 5. Add system instructions
cat > ~/.clawdbot/agents/main/agent/SYSTEM.md << 'EOF'
# Use SearXNG for web search
When user asks for web search, use: searxng-search search "query" 10
EOF

# 6. Test
searxng-search search "rust programming" 5
```

## Detailed Setup

### Step 1: Install SearXNG MCP

See [main README](../README.md) for build instructions.

### Step 2: Install Wrapper Script

The `searxng-search` wrapper provides a CLI interface compatible with Clawdbot:

```bash
cp scripts/searxng-search ~/.local/bin/
chmod +x ~/.local/bin/searxng-search

# Link as moltbot-web-tool for compatibility
ln -sf ~/.local/bin/searxng-search ~/.local/bin/moltbot-web-tool
```

### Step 3: Configure Clawdbot

Add to `~/.clawdbot/.env`:

```bash
# SearXNG Configuration
SEARXNG_URL=http://localhost:8888
SEARXNG_CACHE_TTL=15

# Disable Brave
BRAVE_API_KEY=""
```

Create `~/.clawdbot/agents/main/agent/SYSTEM.md`:

```markdown
## Web Search

Use SearXNG (self-hosted) instead of Brave API:
- Command: `searxng-search search "query" 10`
- Time filters: `searxng-search search "query" 10 week`
- Fetch URL: `searxng-search fetch "URL" 5000`

DO NOT use built-in web_search tool (requires Brave API key).
```

### Step 4: Restart Clawdbot

```bash
source ~/.clawdbot/.env
clawdbot service restart
```

## Usage

### Command Line

```bash
# General search
searxng-search search "machine learning" 10

# News search with time filter
searxng-search search "AI news" 10 week

# Fetch URL
searxng-search fetch "https://example.com" 5000
```

### Via Telegram

```
Search for rust programming
Search news for AI regulation this week
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "Brave API key required" | Ensure SYSTEM.md instructs to use searxng-search |
| No results | Check `curl http://localhost:8888/healthz` |
| Command not found | Add `~/.local/bin` to PATH |

## Migration from Brave

| Before (Brave) | After (SearXNG) |
|----------------|-----------------|
| `BRAVE_API_KEY=...` | `BRAVE_API_KEY=""` |
| Built-in web_search | exec tool + searxng-search |
| 2000 queries/month | Unlimited |
| 1 engine | 89+ engines |

## Support

- [GitHub Issues](https://github.com/yourusername/searxng-mcp/issues)
- Clawdbot docs: https://docs.clawd.bot/
