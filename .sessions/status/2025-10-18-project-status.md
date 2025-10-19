# Project Status: October 18, 2025

## Session Overview

**Date**: October 18, 2025
**Focus**: Complete agent system setup and architecture design
**Status**: ✅ Agent system complete, ready for command implementation

---

## What Was Accomplished Today

### 1. Agent System Architecture (15 Agents Created)

Successfully designed and implemented a comprehensive 4-tier agent system for research-driven audio DSP development:

#### **Tier 1: Research & Knowledge Foundation (4 agents)**
1. **research-assistant** - Web resource gathering, saves markdown + JSON metadata
2. **dsp-expert** - Analyzes research, creates technical notes and educational articles
3. **dsp-peer-reviewer** - Validates mathematical accuracy and citations
4. **audio-engineer** - Creates practical implementation guides from theory

#### **Tier 2: Core Development (6 agents)**
5. **dsp-test-writer** - TDD specialist (RED → COMPREHENSIVE phases)
6. **dsp-implementer** - Makes tests GREEN, then REFACTORs
7. **rust-audio-reviewer** - Real-time safety & code quality gatekeeper
8. **audio-rust-debugger** - Combined audio + Rust debugging
9. **gui-designer** - Plugin GUI architecture and implementation
10. **performance-optimizer** - CPU profiling and optimization

#### **Tier 3: Post-Development (2 agents)**
11. **learning-documenter** - Captures implementation insights
12. **daw-test-coordinator** - Generates Reaper test plans

#### **Tier 4: Workflow & Project Management (3 agents)**
13. **github-project-manager** - Issue routing, PR automation
14. **vst-validator** - VST3 compliance checking
15. **dependency-manager** - Cargo dependency tracking

**Location**: `.claude/agents/` (15 agent files + comprehensive README)

---

### 2. Documentation Created

#### **Core Documentation**
- **[docs/architecture.md](../../docs/architecture.md)** - Comprehensive workspace structure guide
  - Cargo workspace organization (shared/ vs plugin-specific)
  - "What code goes where" decision trees
  - Real-time safety requirements
  - Integration patterns and examples
  - Testing strategies (60+ pages)

- **[.claude/agents/README.md](../../.claude/agents/README.md)** - Agent directory & workflows
  - Agent quick reference table
  - 6 workflow patterns (A-F) with issue label routing
  - TDD workflow integration
  - Best practices and troubleshooting

- **[docs/brainstorming-claude.md](../../docs/brainstorming-claude.md)** - Planning discussion
  - Complete architectural rationale
  - All key decisions documented
  - User feedback impact analysis
  - Design patterns
  - Future enhancements roadmap

#### **Updated Documentation**
- **[README.md](../../README.md)** - Added workspace architecture overview
- **[.claudecode/prompt-playbook.md](../../.claudecode/prompt-playbook.md)** - Added agent section with examples

---

### 3. Directory Structure Established

```
Experiments/
├── .claude/
│   └── agents/              # 15 agent files + README
├── .sessions/               # NEW - Session tracking
│   ├── status/              # Dated project status files
│   └── guides/              # Dated session guide files
├── library/                 # NEW - Research knowledge base
│   ├── sources/             # Raw research materials
│   ├── research-notes/      # DSP analysis
│   ├── educational-articles/# Teaching materials
│   └── implementation-guides/# Practical guides
├── shared/                  # Shared DSP crates (planned)
│   ├── dsp-core/            # Pure DSP algorithms
│   ├── audio-utils/         # Cross-cutting utilities
│   └── modulation/          # Modulation system
├── naughty-and-tender/      # First synthesizer plugin
└── docs/
    ├── architecture.md      # NEW - Code organization
    ├── brainstorming-claude.md # NEW - Planning discussion
    └── learnings/           # Post-implementation docs
```

---

### 4. Key Architectural Decisions

#### **Decision 1: Research Pipeline (4 agents)**
- Separated gathering (research-assistant) from analysis (dsp-expert)
- Added audio-engineer as theory-to-practice bridge
- Creates permanent, citable knowledge base in `library/`

#### **Decision 2: Workspace Architecture**
- Proactive architecture vs reactive component extraction
- Cargo workspace with shared crates from day 1
- Clear "what goes where" rules prevent extraction debt

#### **Decision 3: TDD Integration**
- Tight collaboration: dsp-test-writer ↔ dsp-implementer
- RED → GREEN → REFACTOR → COMPREHENSIVE cycle
- Tests define behavior before code exists

#### **Decision 4: Combined Debugger**
- Single audio-rust-debugger handles both domains
- Reduces context-switching overhead
- Many Rust bugs manifest as audio issues

#### **Decision 5: Multiple Workflow Patterns**
- 6 patterns (A-F) for different issue types
- GitHub issue labels trigger automatic routing
- Flexible iteration within patterns

#### **Decision 6: Issue-Centric Development**
- Pre-planned issues (like existing 20 for naughty-and-tender)
- `/next-issue` as primary workflow driver
- Workflow commands leverage existing issue structure

**Full rationale**: See [docs/brainstorming-claude.md](../../docs/brainstorming-claude.md)

---

### 5. Workflow Patterns Defined

**Pattern A: Full Research Cycle** (`type:feature-new-dsp`)
- research-assistant → dsp-expert → dsp-peer-reviewer → audio-engineer
- → dsp-test-writer (RED) → dsp-implementer (GREEN) → rust-audio-reviewer
- → dsp-implementer (REFACTOR) → dsp-test-writer (comprehensive)
- → performance-optimizer → learning-documenter

**Pattern B: Implementation Only** (`type:feature-existing-dsp`)
- dsp-test-writer (RED) → dsp-implementer (GREEN) → rust-audio-reviewer
- → dsp-implementer (REFACTOR) → dsp-test-writer (comprehensive)

**Pattern C: Bug Fix** (`type:bug`)
- audio-rust-debugger → dsp-implementer → dsp-test-writer → rust-audio-reviewer

**Pattern D: GUI Work** (`type:gui`)
- gui-designer → rust-audio-reviewer → daw-test-coordinator

**Pattern E: Optimization** (`type:optimization`)
- performance-optimizer → dsp-implementer → dsp-test-writer → rust-audio-reviewer

**Pattern F: Research Only** (`type:research`)
- research-assistant → dsp-expert → dsp-peer-reviewer → audio-engineer

**GitHub issue labels automatically route to appropriate patterns!**

---

## Where We Left Off

### Completed Today ✅
- [x] 15 specialized agents created with detailed SOPs
- [x] Complete agent directory with workflow patterns
- [x] Architecture documentation (workspace structure)
- [x] Planning discussion documented
- [x] Prompt playbook updated with agent examples
- [x] Directory structures created (`.sessions/`, `library/`)

### Next Session: Commands & MCP Configuration

We're ready to implement **slash commands** and **MCP integrations** to automate workflow orchestration.

---

## Open Questions for Next Session

Before building commands and configuring MCPs, need to clarify:

### 1. MCP Migration Decision
**Should we switch from VSCode Extension to CLI + Desktop?**

**Context**: VSCode extension doesn't support MCPs. Claude Code CLI + Claude Desktop gives full MCP support.

**Options**:
- **A**: Switch to CLI+Desktop (full MCP support) ← Recommended
- **B**: Stay with VSCode extension (no MCPs)
- **C**: Build agnostic (works both ways)

**Benefits of switching**:
- ✅ GitHub MCP (richer issue/PR management than `gh` CLI)
- ✅ Brave Search MCP (seamless web research for research-assistant)
- ✅ Fetch MCP (better web content extraction)
- ✅ Future-proof (MCPs are future of AI tool integration)

**Setup effort**: ~15 minutes

**Recommendation**: Switch to CLI+Desktop for MCP support

---

### 2. Session Directory Structure
**Confirm**: `.sessions/status/` and `.sessions/guides/` subdirs?

**Proposed**:
```
.sessions/
├── status/
│   ├── 2025-10-18-project-status.md
│   └── 2025-10-19-project-status.md
└── guides/
    ├── 2025-10-18-session-guide.md
    └── 2025-10-19-session-guide.md
```

**Benefits**:
- Full development history preserved
- Easy chronological browsing
- Pattern detection over time

**User suggested this** - just confirming before implementation.

---

### 3. Issue Priority Routing
**For `/next-issue`, how should it pick the next issue?**

**Options**:
- **A**: GitHub's native order (first open issue)
- **B**: By labels (priority:high → priority:medium → priority:low)
- **C**: By issue number (oldest first)
- **D**: Custom field in GitHub Project (explicit ordering)

**Recommendation**: **Option D** (custom field) - gives full control
**Fallback**: **Option B** (priority labels)

**Question**: Do you have a GitHub Project board set up with custom fields for ordering?

---

### 4. Session Status Updates
**Should `/end-session` update GitHub issues with progress?**

**Options**:
- **A**: Update GitHub issue with progress comment
- **B**: Just local status file
- **C**: Both

**Example GitHub comment**:
```
Session 2025-10-19: Implemented PolyBLEP core, tests passing, awaiting review
```

**Recommendation**: **Option C** (both) - keeps GitHub issues as source of truth

---

### 5. `/new-plugin` Scope
**When creating a new plugin, how automated should it be?**

**Options**:
- **A**: Full interactive (brainstorm → issues → directory setup → Cargo config)
- **B**: Just issue creation (manual directory setup)
- **C**: Template-based (choose synth/effect/analyzer template)

**Recommendation**: Start with **Option C** (templates), evolve to **Option A**

**Question**: Should we create plugin templates now, or build `/new-plugin` when needed?

---

## Planned Commands (Pending Answers)

### Core Workflow (Issue-Centric)
1. **`/begin-session`** - Initialize session, load context, generate session guide
2. **`/next-issue`** - Work on next prioritized issue (PRIMARY WORKFLOW)
3. **`/work-issue <num>`** - Work on specific issue
4. **`/end-session`** - Auto-commit, update status, wrap session

### Project Management
5. **`/new-plugin`** - Interactive plugin planning, issue creation, directory setup

### Flexible/Ad-hoc
6. **`/feature --topic "X"`** - One-off feature (optional issue creation)
7. **`/research --topic "X"`** - Research-only exploration

### Fine-Grained (Override auto-routing)
8. **`/implement --topic "X"`** - Force Pattern B
9. **`/test --component "X"`** - Just testing phase
10. **`/review --file "X"`** - Just code review

**Total: 10 commands** aligned with issue-centric workflow

---

## Planned MCP Configuration (If CLI+Desktop)

### 1. GitHub MCP
**Purpose**: Enhanced issue/PR management for github-project-manager

**Installation**:
```bash
npm install -g @modelcontextprotocol/server-github
```

**Configuration** (in Claude Desktop `claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "ghp_your_token"
      }
    }
  }
}
```

**What it enables**:
- Create/update issues with richer API
- Inline PR comments (rust-audio-reviewer can comment directly!)
- Project board manipulation
- Check run status
- Code search across repos

---

### 2. Brave Search MCP
**Purpose**: Seamless web research for research-assistant

**Installation**:
```bash
npm install -g @modelcontextprotocol/server-brave-search
```

**Configuration**:
```json
{
  "mcpServers": {
    "brave-search": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-brave-search"],
      "env": {
        "BRAVE_API_KEY": "your_api_key"
      }
    }
  }
}
```

**Get API key**: https://brave.com/search/api/

**What it enables**:
- One-step search → content extraction
- Better than WebSearch → WebFetch workflow
- Automatic relevance ranking

---

### 3. Fetch MCP
**Purpose**: Enhanced web content extraction

**Installation**:
```bash
npm install -g @modelcontextprotocol/server-fetch
```

**Configuration**:
```json
{
  "mcpServers": {
    "fetch": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-fetch"]
    }
  }
}
```

**What it enables**:
- Better JavaScript handling
- Cleaner markdown extraction
- More reliable than WebFetch for complex sites

---

## Current Project State

### Active Plugin
**naughty-and-tender** - First MIDI synthesizer
- 20 issues outlining development workflow
- Status: Pre-implementation (setup phase)
- Next: Begin implementation with `/next-issue` workflow

### Tech Stack
- **Language**: Rust
- **Plugin Format**: VST3 (nih-plug framework)
- **DAW**: Reaper (testing environment)
- **Build**: Cargo workspace

### Repository
- **GitHub**: colcavanaugh/audio-experiments
- **Branch**: main
- **Last Commit**: Config: Add Claude Code project configuration (6ad763f)

---

## Agent System Ready Status

### Fully Operational ✅
- All 15 agents created with detailed SOPs
- Workflow patterns defined (A-F)
- Architecture documented
- Directory structure in place

### Pending Implementation ⏳
- Slash commands (10 commands planned)
- MCP configuration (optional but recommended)
- Agent prompt enhancements for MCPs

### Ready to Test
Once commands are implemented, ready to validate full workflow with first DSP feature (e.g., sine oscillator or PolyBLEP)

---

## Next Session Agenda

### 1. Answer Open Questions (5 mins)
- Confirm MCP migration decision
- Confirm session directory structure
- Specify issue priority routing
- Decide GitHub status update behavior
- Define `/new-plugin` scope

### 2. MCP Setup (If switching to CLI+Desktop) (15 mins)
- Install Claude Desktop
- Configure GitHub, Brave Search, Fetch MCPs
- Install Claude Code CLI
- Test in terminal

### 3. Implement Commands (2-3 hours)
- Create 10 slash commands
- Test with existing issues
- Validate workflow routing

### 4. Agent Enhancements (1 hour)
- Update agent prompts for MCP tools
- Test research-assistant with Brave Search
- Test github-project-manager with GitHub MCP

### 5. First Feature Test (Optional)
- Run `/next-issue` on first naughty-and-tender issue
- Validate full workflow in practice
- Identify any refinements needed

---

## Resources

### Documentation
- [Agent Directory README](./../.claude/agents/README.md)
- [Architecture Guide](../../docs/architecture.md)
- [Planning Discussion](../../docs/brainstorming-claude.md)
- [Prompt Playbook](../../.claudecode/prompt-playbook.md)

### GitHub
- [All Issues](https://github.com/colcavanaugh/audio-experiments/issues)
- [naughty-and-tender Issues](https://github.com/colcavanaugh/audio-experiments/issues?q=is%3Aissue+is%3Aopen+label%3Anaughty-and-tender)

### External Resources
- [Claude Code Documentation](https://docs.claude.com/en/docs/claude-code)
- [MCP Documentation](https://modelcontextprotocol.io/)
- [Rust Audio Discord](https://discord.gg/QPdhk2u)

---

## Session Summary

**Today was all about foundation**. We built a comprehensive 15-agent system with clear responsibilities, workflow patterns, and architectural guidelines. The agent team is ready - we just need to build the commands that orchestrate them.

**Next session**: Answer 5 clarifying questions, set up MCPs (recommended), build 10 commands, and we'll be ready to start building actual audio DSP code with our new agent-powered workflow!

**The journey from here**: First DSP feature → Validate workflow → Iterate on agents → Build amazing synthesizers! 🎵

---

**Session End**: October 18, 2025
**Status**: Setup complete, ready for command implementation
**Next Session**: Answer questions → Configure MCPs → Build commands → Start coding!
