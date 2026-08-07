# Publishing to GitHub

This guide will help you publish the SearXNG MCP repository to GitHub.

## Step 1: Create GitHub Repository

1. Go to https://github.com/new
2. Repository name: `searxng-mcp`
3. Description: "MCP server for SearXNG web search - free, private, multi-engine search for AI agents"
4. Make it **Public**
5. Do NOT initialize with README (we already have one)
6. Click "Create repository"

## Step 2: Add Remote and Push

```bash
# Navigate to the repository
cd ~/dev/searxng-mcp

# Add the GitHub remote (replace with your username)
git remote add origin https://github.com/YOUR_USERNAME/searxng-mcp.git

# Push to GitHub
git push -u origin main
```

## Step 3: Verify

1. Go to `https://github.com/YOUR_USERNAME/searxng-mcp`
2. Verify all files are present:
   - README.md
   - Cargo.toml
   - src/ directory
   - scripts/ directory
   - docs/ directory
   - .github/workflows/

## Step 4: Enable GitHub Actions

1. Go to "Actions" tab
2. Click "I understand my workflows, go ahead and enable them"
3. The CI workflow will run on the next push

## Step 5: Create a Release (Optional)

1. Go to "Releases" on the right sidebar
2. Click "Create a new release"
3. Tag: `v0.1.0`
4. Title: "Initial Release - SearXNG MCP v0.1.0"
5. Description:
   ```markdown
   ## SearXNG MCP v0.1.0

   First release of the SearXNG MCP server!

   ### Features
   - 7 search tools (web, image, news, video, technical, suggestions, fetch)
   - Aggregates 89+ search engines
   - No API keys required
   - Self-hosted and privacy-focused
   - Clawdbot/Moltbot integration
   - Claude Desktop support

   ### Installation
   See [README.md](README.md) for installation instructions.
   ```
6. Click "Publish release"

## Step 6: Add Topics (Optional)

1. Click the gear icon next to "About"
2. Add topics:
   - `mcp`
   - `searxng`
   - `search`
   - `llm`
   - `ai`
   - `claude`
   - `privacy`
   - `self-hosted`
3. Save changes

## Repository Structure

```
searxng-mcp/
├── .github/
│   └── workflows/
│       └── rust.yml          # CI/CD
├── docs/
│   ├── README.md             # Docs index
│   └── CLAWDBOT-INTEGRATION.md
├── scripts/
│   ├── searxng-search        # HTTP wrapper
│   └── moltbot-web-tool      # Clawdbot compatibility
├── src/
│   ├── main.rs               # Entry point
│   ├── mcp/                  # MCP protocol
│   ├── tools/                # Search tools
│   └── searxng_client.rs     # API client
├── Cargo.toml                # Rust manifest
├── README.md                 # Main documentation
├── CONTRIBUTING.md           # Contribution guide
├── LICENSE                   # MIT License
└── install.sh                # Install script
```

## Next Steps

After publishing:

1. **Share the repository** with the community
2. **Submit to awesome lists**:
   - awesome-mcp-servers
   - awesome-rust
   - awesome-selfhosted
3. **Write a blog post** about the project
4. **Create video tutorials** for setup

## Support

For help with publishing:
- GitHub Docs: https://docs.github.com/en/get-started
- GitHub Support: https://support.github.com/
