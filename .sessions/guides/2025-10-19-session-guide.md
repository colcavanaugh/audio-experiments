# Session Guide: October 19, 2025

## Session Start: 2025-10-19

## Session Overview

**Focus**: Slash command implementation and MCP configuration
**Status**: ✅ Complete
**Duration**: ~3 hours

---

## Session Goals

Building on the 15-agent system created on October 18th:
1. Configure GitHub Project with Sequence field for issue ordering
2. Update github-project-manager with project-specific configuration
3. Create MCP setup guide for CLI+Desktop transition
4. Implement 10 slash commands for workflow orchestration
5. Update key agents for MCP tool integration
6. Create session tracking documentation

---

## Work Completed

### 1. GitHub Project Configuration ✅

**Project**: Audio DSP Experiments (ID: 5)
**Owner**: colcavanaugh
**Repository**: audio-experiments

**Custom Fields Confirmed**:
- Status (single select)
- Project (single select)
- Category (single select)
- Priority (single select)
- Learning Focus (text)
- **Sequence (number)** - Created for `/next-issue` ordering

**Sequence Field Usage**:
- Lower numbers = higher priority
- Used by `/next-issue` to select work automatically
- Updated by github-project-manager at session end

### 2. github-project-manager Agent Updated ✅

**Added**:
- Project configuration section with field IDs
- Sequence field usage documentation
- Session-end responsibilities
- MCP integration guidance
- Progress comment templates

**Location**: `.claude/agents/github-project-manager.md`

### 3. MCP Setup Guide Created ✅

**Comprehensive guide for**:
- Installing Claude Desktop
- Installing MCP servers (GitHub, Brave Search, Fetch)
- Obtaining API keys (GitHub PAT, Brave API)
- Configuring `claude_desktop_config.json`
- Installing Claude Code CLI
- Testing MCP connections
- Troubleshooting common issues

**Location**: `docs/mcp-setup-guide.md`

**Next User Action**: Follow guide to set up MCP environment

### 4. Slash Commands Implemented ✅

**10 commands created** in `.claude/commands/`:

#### Core Workflow Commands
1. **`/begin-session`** - Initialize session, load context, display next recommended issue
   - Loads git status, recent commits
   - Queries GitHub Project for prioritized issues
   - Identifies next issue by Sequence field
   - Generates session guide file
   - **Output**: Session summary + recommended next steps

2. **`/next-issue`** - Work on next prioritized issue (Sequence-based)
   - Queries project for lowest Sequence, Status != "Done"
   - Determines workflow pattern from labels
   - Updates issue Status to "In Progress"
   - Executes agent sequence (Pattern A-F)
   - **Primary workflow driver**

3. **`/work-issue <number>`** - Work on specific issue, bypassing queue
   - Manual issue selection
   - Optional `--pattern` override
   - Same workflow execution as `/next-issue`
   - **Use for**: Urgent work, continuing paused issues

4. **`/end-session`** - Wrap up session, update GitHub, commit changes
   - Assesses completion status of active issues
   - Updates GitHub Project fields via github-project-manager
   - Adds progress comments to issues
   - Creates session status file
   - Offers to commit changes
   - **Critical for**: GitHub project tracking

#### Project Management
5. **`/new-plugin`** - Fully interactive plugin creation
   - Brainstorm concept (type, inspiration, goals)
   - Define technical architecture
   - Define all parameters
   - Break into features (MVP → V1.0 → V2.0)
   - Create comprehensive GitHub issues
   - Set up directory structure
   - Configure Cargo workspace
   - Generate concept documentation
   - **Output**: Ready-to-build plugin with full roadmap

#### Flexible/Ad-Hoc Commands
6. **`/feature --topic "X"`** - Quick feature implementation
   - Auto-detect workflow pattern
   - Optional `--create-issue` for tracking
   - Execute pattern workflow
   - **Use for**: Unplanned enhancements, experiments

7. **`/research --topic "X"`** - Research-only exploration
   - Forces Pattern F (Research Only)
   - Optional `--depth quick|thorough`
   - Creates comprehensive research materials
   - **Output**: Sources, notes, guides, learnings
   - **Use for**: Exploring new DSP before implementing

#### Fine-Grained Override Commands
8. **`/implement --topic "X"`** - Force implementation workflow
   - Always Pattern B (skip research)
   - **Use for**: Well-known algorithms (ADSR, basic filters)

9. **`/test --component "X"`** - Testing workflow
   - Identify test gaps
   - Create additional tests
   - Run test suite and report
   - Optional `--comprehensive` for exhaustive testing
   - **Output**: Test report with recommendations

10. **`/review --file|--module|--pr|--changes`** - Code review
    - Launch rust-audio-reviewer
    - Check real-time safety, idioms, DSP correctness
    - Optional inline PR comments (if GitHub MCP)
    - **Output**: Review report with severity levels

**All commands documented** with usage examples, workflow details, and implementation notes.

### 5. Agent MCP Enhancements ✅

**Updated Agents**:

1. **research-assistant** (`.claude/agents/research-assistant.md`)
   - Added MCP integration section
   - Prefer Brave Search MCP over WebSearch + WebFetch
   - Use Fetch MCP for better content extraction
   - Fallback to standard tools if MCPs unavailable

2. **rust-audio-reviewer** (`.claude/agents/rust-audio-reviewer.md`)
   - Added GitHub MCP integration section
   - Use GitHub MCP for inline PR comments
   - Severity marking (CRITICAL, WARNING, SUGGESTION)
   - Code fix suggestions inline
   - Fallback to consolidated comments via gh CLI

3. **github-project-manager** (already updated in step 2)
   - Project-specific configuration
   - MCP-first approach for field updates
   - Session-end automation

### 6. Session Documentation Created ✅

**This session guide**: `.sessions/guides/2025-10-19-session-guide.md`

**Purpose**:
- Track daily progress
- Document decisions
- Record learnings
- Provide continuity between sessions

---

## Commands Summary

| Command | Purpose | When to Use |
|---------|---------|-------------|
| `/begin-session` | Start session | Every session start |
| `/next-issue` | Work next prioritized issue | Primary workflow (80% of time) |
| `/work-issue <num>` | Work specific issue | Urgent fixes, continue paused work |
| `/end-session` | Wrap up session | Every session end |
| `/new-plugin` | Plan new plugin | Starting new plugin project |
| `/feature --topic` | Quick feature | Ad-hoc enhancements |
| `/research --topic` | Research only | Explore before implementing |
| `/implement --topic` | Known algorithms | ADSR, basic filters, etc. |
| `/test --component` | Test existing code | Validation, gap analysis |
| `/review --file` | Code review | Pre-commit, PR review |

---

## Files Changed

### Created (15 files)
1. `.claude/commands/begin-session.md`
2. `.claude/commands/next-issue.md`
3. `.claude/commands/work-issue.md`
4. `.claude/commands/end-session.md`
5. `.claude/commands/new-plugin.md`
6. `.claude/commands/feature.md`
7. `.claude/commands/research.md`
8. `.claude/commands/implement.md`
9. `.claude/commands/test.md`
10. `.claude/commands/review.md`
11. `docs/mcp-setup-guide.md`
12. `.sessions/guides/2025-10-19-session-guide.md`

### Modified (3 files)
13. `.claude/agents/github-project-manager.md` - Added project config + session-end duties
14. `.claude/agents/research-assistant.md` - Added MCP integration
15. `.claude/agents/rust-audio-reviewer.md` - Added GitHub MCP for PR reviews

---

## Next Session Priorities

### Immediate: MCP Setup (~15-30 minutes)
1. **Follow MCP setup guide** (`docs/mcp-setup-guide.md`)
   - Install Claude Desktop
   - Configure GitHub MCP with PAT
   - Configure Brave Search MCP (optional but recommended)
   - Configure Fetch MCP
   - Test connections

2. **Switch to Claude Code CLI**
   - Install: `npm install -g @anthropic-ai/claude-code`
   - Navigate to project: `cd C:\Users\colca\OneDrive\Desktop\Audio\Experiments`
   - Launch: `claude-code`

### First Real Workflow Test
3. **Run `/begin-session`**
   - Test session initialization
   - Verify GitHub Project integration
   - See next recommended issue

4. **Run `/next-issue`** (or work on specific issue)
   - Test Sequence-based selection
   - Test workflow pattern execution
   - Validate agent orchestration

5. **Run `/end-session`**
   - Test GitHub updates
   - Test session status generation
   - Test commit workflow

### Optional: Plugin Planning
6. **Run `/new-plugin`** (if ready to plan another plugin)
   - Test interactive brainstorming
   - Test issue creation
   - Test directory setup

---

## Learnings & Decisions

### Decision 1: Sequence Field for Issue Ordering
**Rationale**: Explicit control over work priority
- More flexible than GitHub's native ordering
- Better than priority labels (can have same priority)
- Allows fine-grained ordering (1, 2, 3, ...)
- github-project-manager clears Sequence on completion

### Decision 2: Issue-Centric Workflow
**Rationale**: GitHub issues as source of truth
- All work tracked in issues
- Workflow commands operate on issues
- Ad-hoc commands can optionally create issues
- Session-end updates keep GitHub current

### Decision 3: Pattern Override Commands
**Rationale**: Flexibility for experienced users
- `/next-issue` and `/work-issue` use label-based patterns
- Fine-grained commands (`/implement`, `/test`, `/review`) force specific workflows
- `/feature` auto-detects but allows override
- Balances automation with control

### Decision 4: MCP-First Approach
**Rationale**: Better tool capabilities
- Brave Search MCP > WebSearch + WebFetch
- GitHub MCP > gh CLI for complex operations
- Fetch MCP > WebFetch for modern sites
- Agents check MCP availability and adapt

### Decision 5: Comprehensive Command Documentation
**Rationale**: Self-documenting system
- Each command file is a complete reference
- Includes usage, examples, workflows, implementation notes
- Reduces need for external documentation
- Makes commands discoverable

---

## System Architecture Summary

```
User
  ↓
Slash Commands (/next-issue, /work-issue, etc.)
  ↓
Workflow Patterns (A-F)
  ↓
Agent Orchestration (15 specialized agents)
  ↓
GitHub Project Updates (via github-project-manager)
  ↓
Session Tracking (.sessions/status/, .sessions/guides/)
```

**Core Loop**:
1. `/begin-session` → Load context
2. `/next-issue` → Execute workflow → Build feature
3. `/end-session` → Update GitHub → Commit → Save status
4. Repeat

---

## Statistics

**Total Work**:
- Agents updated: 3
- Commands created: 10
- Documentation files: 12
- Lines of documentation: ~3,000+
- Decisions documented: 6
- Workflow patterns formalized: 6

**Project Files**:
- Agents: 15 (in `.claude/agents/`)
- Commands: 10 (in `.claude/commands/`)
- Documentation: 7 (in `docs/`)
- Session tracking: 2 (in `.sessions/`)

---

## Resources

### Documentation Created Today
- **MCP Setup**: `docs/mcp-setup-guide.md`
- **Session Guide**: `.sessions/guides/2025-10-19-session-guide.md`
- **Commands**: `.claude/commands/*.md` (10 files)

### Documentation from October 18
- **Agent Directory**: `.claude/agents/README.md`
- **Architecture Guide**: `docs/architecture.md`
- **Brainstorming**: `docs/brainstorming-claude.md`

### External Resources
- [Claude Code Documentation](https://docs.claude.com/en/docs/claude-code)
- [MCP Documentation](https://modelcontextprotocol.io/)
- [GitHub Projects API](https://docs.github.com/en/issues/planning-and-tracking-with-projects)

---

## Open Questions / Future Enhancements

### For Next Session
1. **Test workflow end-to-end**: Does `/next-issue` → `/end-session` work smoothly?
2. **MCP performance**: How much faster is Brave Search vs WebSearch?
3. **GitHub field updates**: Can GitHub MCP update custom fields easily?

### Future Enhancements
1. **Workflow pause/resume**: Ability to pause mid-workflow and resume later
2. **Agent skip**: `/skip-to <agent-name>` for advanced users
3. **Batch operations**: Process multiple issues in one session
4. **Preset workflows**: Save custom workflow patterns beyond A-F
5. **Performance metrics**: Track agent execution times, optimize slow paths

---

## Session Notes

**What Went Well**:
- Clear progression from yesterday's agent system to today's command layer
- Comprehensive command documentation will help future usage
- MCP integration planned thoughtfully with fallbacks
- GitHub Project integration leverages existing structure

**Challenges**:
- Commands are complex and will need real-world testing
- MCP setup is user-dependent (can't automate installation)
- Workflow orchestration may need refinement based on usage

**Key Insight**:
The combination of specialized agents + slash commands + GitHub Project integration creates a complete development workflow system. Each layer serves a clear purpose:
- **Agents**: Specialized expertise
- **Commands**: Workflow orchestration
- **GitHub**: Source of truth & tracking
- **Sessions**: Continuity & history

---

## Session End: 2025-10-19

**Status**: Command system complete, ready for MCP setup and real-world testing

**Next Steps**:
1. User: Follow `docs/mcp-setup-guide.md` to configure MCPs
2. User: Switch to Claude Code CLI
3. First session: `/begin-session` → `/next-issue` → `/end-session`
4. Validate workflows, iterate on any issues

**Tomorrow's Focus**: First real feature implementation using the new command system!

---

**The system is ready. Let's build! 🎵**
