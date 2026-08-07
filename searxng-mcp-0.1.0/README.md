# SearXNG MCP Server

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org)
[![MCP](https://img.shields.io/badge/MCP-Protocol-blue.svg)](https://modelcontextprotocol.io/)

> A powerful [Model Context Protocol (MCP)](https://modelcontextprotocol.io/) server providing comprehensive search capabilities that rival **Brave Search API**, **Perplexity**, and other commercial search services. Powered by [SearXNG](https://docs.searxng.org/), aggregating results from **89+ search engines** including Google, Bing, DuckDuckGo, Brave, and more.

## 🌟 Features

### Search Tools (7 Total)

| Tool | Description | Parameters |
|------|-------------|------------|
| **`web_search`** | General web search with advanced filters | query, max_results, time_range, language, safe_search, engines, page |
| **`image_search`** | Image search across multiple sources | query, max_results, safe_search |
| **`news_search`** | News articles with recency filtering | query, max_results, time_range, language |
| **`video_search`** | Video search with thumbnails | query, max_results, safe_search |
| **`search_suggestions`** | Autocomplete and query suggestions | query |
| **`technical_search`** | Code and documentation search | query, max_results, time_range, language |
| **`fetch_url`** | Fetch and extract content from URLs | url, extract_content, max_length |

### Key Capabilities

- 🔒 **Privacy-First**: Self-hosted, no data leaves your machine
- 💰 **Free Forever**: No API keys, no usage limits, no credit cards
- 🌐 **89+ Search Engines**: Aggregates Google, Bing, DDG, Brave, Qwant, Startpage, Mojeek, and more
- ⚡ **Fast**: Local caching, concurrent requests
- 🎯 **Accurate**: Multiple engine verification, relevance scoring
- 🌍 **Multi-Language**: Support for 20+ languages
- 🔍 **Advanced Filters**: Time ranges, safe search, specific engines
- 📊 **Rich Results**: Images, videos, news, code, and web content

## 📋 Table of Contents

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Configuration](#configuration)
- [Usage](#usage)
- [Integrations](#integrations)
  - [Claude Desktop](#claude-desktop)
  - [Kimi Code CLI](#kimi-code-cli)
  - [Clawdbot / Moltbot](#clawdbot--moltbot)
  - [Other MCP Clients](#other-mcp-clients)
- [API Reference](#api-reference)
- [Docker Setup](#docker-setup)
- [Environment Variables](#environment-variables)
- [Troubleshooting](#troubleshooting)
- [Comparison](#comparison-with-commercial-apis)
- [Contributing](#contributing)
- [License](#license)

## 🚀 Installation

### Prerequisites

- [Rust](https://rustup.rs/) 1.75+ (for building from source)
- [Docker](https://docs.docker.com/get-docker/) (for running SearXNG)
- [Git](https://git-scm.com/)

### Option 1: Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/searxng-mcp.git
cd searxng-mcp

# Build release binary
cargo build --release

# The binary will be at target/release/searxng-mcp
```

### Option 2: Install via Cargo

```bash
cargo install --path .
```

### Option 3: Using Install Script

```bash
curl -fsSL https://raw.githubusercontent.com/yourusername/searxng-mcp/main/install.sh | bash
```

## ⚡ Quick Start

### 1. Start SearXNG (Docker)

```bash
# Create config directory
mkdir -p ~/.config/searxng

# Run SearXNG container
docker run -d \
  --name searxng \
  -p 8888:8080 \
  -v "$HOME/.config/searxng:/etc/searxng" \
  --restart unless-stopped \
  searxng/searxng:latest

# Verify it's working
curl http://localhost:8888/healthz
```

### 2. Configure Your MCP Client

Add to your MCP client configuration:

```json
{
  "mcpServers": {
    "searxng": {
      "command": "/path/to/searxng-mcp",
      "env": {
        "SEARXNG_URL": "http://localhost:8888",
        "RUST_LOG": "info"
      }
    }
  }
}
```

### 3. Start Searching!

Your AI assistant can now use these tools:
- `web_search` - Search the web
- `image_search` - Find images
- `news_search` - Get news articles
- `video_search` - Search videos
- `technical_search` - Find code and docs
- `fetch_url` - Extract content from URLs

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SEARXNG_URL` | SearXNG instance URL | `http://localhost:8888` |
| `SEARXNG_ENDPOINT` | Alternative to `SEARXNG_URL` | - |
| `RUST_LOG` | Log level (error/warn/info/debug/trace) | `info` |

### SearXNG Settings

Create `~/.config/searxng/settings.yml`:

```yaml
use_default_settings: true

general:
  instance_name: "SearXNG MCP"

server:
  port: 8080
  bind_address: "0.0.0.0"

search:
  safe_search: 0
  autocomplete: "duckduckgo"
  formats:
    - html
    - json

engines:
  - name: google
    engine: google
    disabled: false
  
  - name: bing
    engine: bing
    disabled: false
  
  - name: duckduckgo
    engine: duckduckgo
    disabled: false
  
  - name: brave
    engine: brave
    disabled: false
  
  - name: github
    engine: github
    disabled: false
  
  - name: stackoverflow
    engine: stackoverflow
    disabled: false
  
  - name: wikipedia
    engine: wikipedia
    disabled: false
```

## 💡 Usage Examples

### Web Search

```json
{
  "name": "web_search",
  "arguments": {
    "query": "machine learning frameworks",
    "max_results": 10,
    "time_range": "month",
    "language": "en"
  }
}
```

### News Search

```json
{
  "name": "news_search",
  "arguments": {
    "query": "AI regulation",
    "time_range": "week",
    "max_results": 10
  }
}
```

### Image Search

```json
{
  "name": "image_search",
  "arguments": {
    "query": "rust programming logo transparent",
    "max_results": 5
  }
}
```

### Fetch URL Content

```json
{
  "name": "fetch_url",
  "arguments": {
    "url": "https://www.rust-lang.org/",
    "max_length": 5000
  }
}
```

## 🔌 Integrations

### Claude Desktop

**macOS:** `~/Library/Application Support/Claude/claude_desktop_config.json`

**Windows:** `%APPDATA%\Claude\claude_desktop_config.json`

**Linux:** `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "searxng": {
      "command": "/path/to/searxng-mcp",
      "env": {
        "SEARXNG_URL": "http://localhost:8888"
      }
    }
  }
}
```

### Kimi Code CLI

Add to Kimi Code CLI MCP settings:

```json
{
  "mcpServers": {
    "searxng": {
      "command": "/path/to/searxng-mcp",
      "env": {
        "SEARXNG_URL": "http://localhost:8888"
      }
    }
  }
}
```

### Clawdbot / Moltbot

See [MOLTBOT-INTEGRATION.md](./MOLTBOT-INTEGRATION.md) for detailed setup.

Quick setup:

```bash
# Install wrapper tools
cp scripts/moltbot-web-tool ~/.local/bin/
chmod +x ~/.local/bin/moltbot-web-tool

# Configure clawdbot to use SearXNG
echo 'export SEARXNG_URL=http://localhost:8888' >> ~/.clawdbot/.env
```

### Other MCP Clients

Any MCP-compatible client can use this server:

1. Configure the client with the server path
2. Set `SEARXNG_URL` environment variable
3. The client will auto-discover available tools

## 📚 API Reference

### Tool Schemas

#### web_search

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `query` | string | ✅ | - | Search query |
| `max_results` | integer | ❌ | 10 | Results 1-50 |
| `time_range` | string | ❌ | all | `day`, `week`, `month`, `year` |
| `language` | string | ❌ | - | Language code (en, de, fr, etc.) |
| `safe_search` | string | ❌ | moderate | `none`, `moderate`, `strict` |
| `engines` | array | ❌ | all | Specific engines |
| `page` | integer | ❌ | 1 | Page number |

#### news_search

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `query` | string | ✅ | - | News query |
| `max_results` | integer | ❌ | 10 | Results 1-50 |
| `time_range` | string | ❌ | - | `day`, `week`, `month`, `year` |
| `language` | string | ❌ | - | Language code |

#### fetch_url

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `url` | string | ✅ | - | URL to fetch |
| `extract_content` | boolean | ❌ | true | Extract main content |
| `max_length` | integer | ❌ | unlimited | Max characters |

### Response Format

All tools return markdown-formatted text:

```markdown
## Search Results for: "query"

Estimated total results: 150

Showing 10 results:

### 1. Result Title

**URL:** https://example.com

Result snippet text...

*📅 2024-01-15 | 🔍 Found by: google, bing | ⭐ Score: 8.5 | 📂 Category: general*

---
```

## 🐳 Docker Setup

### Docker Compose

```yaml
version: '3.8'

services:
  searxng:
    image: searxng/searxng:latest
    container_name: searxng
    ports:
      - "8888:8080"
    volumes:
      - ./config:/etc/searxng
    environment:
      - BASE_URL=http://localhost:8888/
    restart: unless-stopped
```

Run with: `docker-compose up -d`

### Advanced Docker Configuration

```bash
# With custom settings
docker run -d \
  --name searxng \
  -p 8888:8080 \
  -v "$HOME/.config/searxng/settings.yml:/etc/searxng/settings.yml:ro" \
  -e "BASE_URL=http://localhost:8888/" \
  -e "INSTANCE_NAME=SearXNG MCP" \
  --restart unless-stopped \
  searxng/searxng:latest
```

## 🔍 Comparison with Commercial APIs

| Feature | SearXNG MCP | Brave API | Perplexity | SerpAPI |
|---------|-------------|-----------|------------|---------|
| **Cost** | ✅ Free | Free tier | Paid | Paid |
| **Engines** | ✅ 89+ | 1 | Multiple | Multiple |
| **Privacy** | ✅ Local | External | External | External |
| **Rate Limits** | ✅ None | 2000/mo | Varies | Varies |
| **Image Search** | ✅ | ✅ | ❌ | ✅ |
| **News Search** | ✅ | ✅ | ✅ | ✅ |
| **Video Search** | ✅ | ❌ | ❌ | ✅ |
| **Code Search** | ✅ | ❌ | ❌ | ❌ |
| **Self-hosted** | ✅ | ❌ | ❌ | ❌ |
| **No API Key** | ✅ | ❌ | ❌ | ❌ |

## 🐛 Troubleshooting

### Server won't start

```bash
# Check SearXNG is running
curl http://localhost:8888/healthz

# Check SearXNG logs
docker logs searxng

# Verify binary
cargo build --release
./target/release/searxng-mcp --version
```

### No search results

```bash
# Verify engines are enabled
curl http://localhost:8888/config | jq '.engines[] | select(.enabled) | .name'

# Test direct search
curl "http://localhost:8888/search?q=test&format=json" | jq '.results | length'
```

### Connection refused

```bash
# Ensure SEARXNG_URL has protocol
export SEARXNG_URL=http://localhost:8888  # ✅
export SEARXNG_URL=localhost:8888          # ❌
```

### MCP Client not finding tools

```bash
# Check server responds to initialization
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"test","version":"1.0"}}}' | ./target/release/searxng-mcp
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [SearXNG](https://docs.searxng.org/) - The privacy-respecting metasearch engine
- [Model Context Protocol](https://modelcontextprotocol.io/) - By Anthropic
- All the search engines that make SearXNG possible

## 📞 Support

- 📧 Open an issue on GitHub
- 💬 Discussions: [GitHub Discussions](https://github.com/yourusername/searxng-mcp/discussions)

---

<p align="center">
  Made with ❤️ for the privacy-conscious AI community
</p>
