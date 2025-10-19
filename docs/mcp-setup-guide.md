# MCP Setup Guide for Claude Code

**Date**: October 19, 2025
**Purpose**: Configure MCP servers for Claude Code to enhance GitHub, research, and web capabilities

---

## Important: Claude Code vs Claude Desktop

**Claude Code and Claude Desktop use SEPARATE MCP configurations.**

- **Claude Desktop**: Uses `%APPDATA%\Claude\claude_desktop_config.json` (Windows) or `~/Library/Application Support/Claude/claude_desktop_config.json` (macOS)
- **Claude Code**: Uses `~/.claude.json` (managed via CLI commands)

This guide is for **Claude Code only**.

---

## Why MCPs?

MCPs provide specialized tools that enhance Claude Code's capabilities:

- **GitHub MCP**: Rich issue/PR management, inline code comments, project board manipulation
- **Tavily Search**: Better web research than built-in WebSearch
- **Exa**: Documentation and content retrieval
- **Fetch MCP**: Enhanced web content extraction with better JavaScript handling

---

## Prerequisites

- Node.js installed (for npx commands)
- Docker installed (for GitHub MCP server)
- Claude Code CLI installed: `npm install -g @anthropic-ai/claude-code`
- GitHub Personal Access Token (get from https://github.com/settings/tokens)
- Tavily API Key (optional - get from https://tavily.com/)

---

## Setup Steps

### 1. Configure MCP Servers via CLI

Use the `claude mcp add` command to configure each server. Replace placeholders with your actual tokens/keys.

**GitHub MCP (Docker-based):**
```bash
claude mcp add github -s user -e GITHUB_PERSONAL_ACCESS_TOKEN=your_token_here -- docker run -i --rm -e GITHUB_PERSONAL_ACCESS_TOKEN ghcr.io/github/github-mcp-server
```

**Tavily Search MCP:**
```bash
claude mcp add tavily-remote-mcp -s user -- npx -y mcp-remote "https://mcp.tavily.com/mcp/?tavilyApiKey=your_key_here"
```

**Exa MCP:**
```bash
claude mcp add exa -s user -- npx -y mcp-remote "https://mcp.exa.ai/mcp"
```

**Fetch MCP:**
```bash
claude mcp add fetch -s user -e DEFAULT_LIMIT=50000 -- npx mcp-fetch-server
```

### 2. Verify Configuration

Check that all servers are connected:
```bash
claude mcp list
```

You should see all four servers with ✓ Connected status.

### 3. Restart Claude Code

**Completely exit and restart Claude Code** for the MCP servers to become available.

---

## Testing MCP Servers

After restarting Claude Code, test each server:

**GitHub MCP:**
```
List the open issues in your-username/your-repo
```

**Tavily Search:**
```
Search for "PolyBLEP antialiasing algorithm"
```

**Exa:**
```
Find Rust audio DSP documentation
```

**Fetch MCP:**
```
Fetch content from https://www.musicdsp.org
```

---

## Managing MCP Servers

**List all servers:**
```bash
claude mcp list
```

**Remove a server:**
```bash
claude mcp remove github
```

**Test a specific server:**
```bash
claude mcp get github
```

**Debug MCP issues:**
```bash
claude-code --mcp-debug
```

---

## Configuration File Location

Your MCP configuration is stored at:
- **Windows**: `%USERPROFILE%\.claude.json`
- **macOS/Linux**: `~/.claude.json`

You can manually edit this file if needed, but using the CLI is recommended.

---

## Troubleshooting

**MCPs not showing up in Claude Code:**
- Ensure you fully restarted Claude Code (quit and relaunch)
- Run `claude mcp list` to verify servers are connected
- Check `~/.claude.json` exists and has correct syntax

**GitHub MCP connection errors:**
- Verify Docker is running
- Check your GitHub token has `repo`, `project`, `read:org` scopes
- Test token: `gh auth status`

**Tavily/Exa not working:**
- Verify API keys are correct (no extra quotes/spaces)
- Check you haven't exceeded rate limits

**Permission denied errors:**
- Ensure Docker has proper permissions
- On Unix systems, may need to run Docker commands with appropriate permissions

---

## Agent Benefits

Once configured, these MCPs enhance your Claude Code agents:

**github-project-manager:**
- Direct project board manipulation
- Inline PR comments for code review
- Richer issue management

**research-assistant:**
- Better search results via Tavily
- Faster research gathering workflow

**All agents:**
- More reliable web content fetching
- Better handling of modern websites
- Enhanced documentation retrieval

---

## Resources

- [Claude Code MCP Documentation](https://docs.claude.com/en/docs/claude-code/mcp)
- [MCP Protocol Specification](https://modelcontextprotocol.io/)
- [GitHub MCP Server](https://github.com/github/github-mcp-server)
- [Tavily Search API](https://tavily.com/)
