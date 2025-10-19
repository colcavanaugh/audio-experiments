# Brainstorming Claude Code Use

> **Planning Discussion: Specialized Agent System for Audio DSP Development**
>
> **Date**: October 18, 2025
> **Participants**: User (colcavanaugh), Claude (Sonnet 4.5)

---

## Context

This document captures the architectural decisions, rationale, and future considerations for the 15-agent system designed to support research-driven audio DSP plugin development in Rust.

---

## Project Overview

**Audio DSP Experiments** is a learning-focused project building audio plugins (VST3) in Rust. The primary goal is understanding through implementation - building sonic literacy by creating synthesizers, effects, and analysis tools.

**Current focus**: naughty-and-tender - first MIDI synthesizer plugin with polyphonic voices, multiple oscillator types, modulation, filtering, and effects.

**Tech stack**: Rust, VST3, nih-plug framework, Reaper DAW for testing

---

## Agent System Architecture

### Design Philosophy

The agent system was designed around a core insight: **audio DSP development requires rigorous research before implementation**. Each DSP algorithm should trace back to validated theoretical foundations, ensuring code is both correct and educational.

### Four-Tier Architecture

#### Tier 1: Research & Knowledge Foundation (Pre-Development)
**Goal**: Establish rigorous theoretical foundation before writing any code

1. **research-assistant** - Gathers DSP resources from web (musicdsp.org, CCRMA, papers)
2. **dsp-expert** - Analyzes research, drafts technical notes and educational articles
3. **dsp-peer-reviewer** - Validates mathematical accuracy and citation precision
4. **audio-engineer** - Bridges theory to practice with implementation guides

**Rationale**: This tier prevents "coding blind" - every implementation references validated research, building a permanent knowledge base in `library/`. Future implementations can reference existing research, reducing redundant work.

#### Tier 2: Core Development (Implementation)
**Goal**: Implement DSP with TDD, real-time safety, and code quality

5. **dsp-test-writer** - Creates failing tests (RED), then comprehensive tests (after GREEN)
6. **dsp-implementer** - Makes tests pass (GREEN), then refactors
7. **rust-audio-reviewer** - Code quality gatekeeper (real-time safety, DSP correctness)
8. **audio-rust-debugger** - Combined audio + Rust debugging
9. **gui-designer** - Plugin GUI architecture and implementation
10. **performance-optimizer** - CPU profiling and optimization

**Rationale**: TDD ensures correctness before optimization. Separated concerns (testing, implementation, review, debugging) create quality gates. Combined debugger (audio + Rust) reduces context-switching overhead.

#### Tier 3: Post-Development (Documentation & Testing)
**Goal**: Capture learnings and validate in real DAW

11. **learning-documenter** - Captures implementation insights and gotchas
12. **daw-test-coordinator** - Generates Reaper test plans for manual validation

**Rationale**: Learning capture builds institutional knowledge. DAW testing catches issues unit tests miss (parameter automation, MIDI handling, CPU in real context).

#### Tier 4: Workflow & Project Management (Cross-Cutting)
**Goal**: Automate GitHub workflow and ensure quality/compliance

13. **github-project-manager** - Issue routing, PR generation, workflow coordination
14. **vst-validator** - VST3 compliance (pluginval automation)
15. **dependency-manager** - Cargo dependency tracking

**Rationale**: Workflow automation reduces cognitive load. Issue label-based routing enables automatic workflow pattern selection. Compliance checking prevents DAW compatibility issues.

---

## Key Architectural Decisions

### Decision 1: Tier 1 Research Pipeline (4 agents vs original 3)

**Original Proposal**:
- dsp-researcher (combined gathering + analysis)
- dsp-research-validator
- synthesis-researcher

**Final Design**:
- research-assistant (gathering only)
- dsp-expert (analysis only)
- dsp-peer-reviewer (validation)
- audio-engineer (NEW - implementation guidance)

**Rationale**:
- **Separation of concerns**: Gathering vs analysis are distinct skillsets
- **Cleaner pipeline**: gather → analyze → validate → translate-to-practice
- **audio-engineer addition**: Critical bridge from theory to implementation - reviews existing code, drafts practical guides with gotchas and Rust patterns
- **Dropped synthesis-researcher**: Merged into research-assistant (no need for separate algorithm-focused researcher)

**User feedback**: "I think we could clarify the division of labor for Tier 1... a `research-assistant` for gathering and organizing resources... a `dsp-expert` for analyzing those sources... a `dsp-peer-reviewer` for reviewing... and finally, an `audio-engineer` for reviewing existing implementations... then drafting expert guidance for our Core Development team."

### Decision 2: Workspace Architecture vs Component Extraction

**Original Proposal**: `component-extractor` agent to reactively extract shared code

**Final Design**: Proactive workspace architecture with clear rules

**Rationale**:
- **Architectural clarity > reactive extraction**: Define where code goes from day 1
- **Cargo workspace**: `shared/dsp-core/`, `shared/audio-utils/`, `shared/modulation/` as separate crates
- **Plugin directories**: Import shared crates, only contain plugin-specific integration
- **Natural reuse**: Future plugins just `use dsp_core::oscillators::PolyBLEP;`
- **No "extraction debt"**: Correct structure from start prevents refactoring later

**User feedback**: "Rather than requiring an agent to extract shared components from the plugin code, what if we instruct our Core Development team on what types of code should be put where... This could obviate the need for a dedicated `component-extractor`... we may want to think more carefully about the architecture of a plugin..."

**Implementation**: Created [docs/architecture.md](../architecture.md) with comprehensive "what goes where" rules and integration patterns.

### Decision 3: TDD Integration (dsp-test-writer ↔ dsp-implementer)

**Design**: Tight collaboration in RED → GREEN → REFACTOR cycle

**Workflow**:
1. **RED**: dsp-test-writer creates failing tests defining behavior
2. **GREEN**: dsp-implementer makes tests pass (minimal implementation)
3. **REVIEW**: rust-audio-reviewer provides feedback
4. **REFACTOR**: dsp-implementer improves code, tests still pass
5. **COMPREHENSIVE**: dsp-test-writer adds edge cases, performance benchmarks

**Rationale**:
- Tests define requirements before implementation exists
- Prevents over-engineering ("minimal to pass")
- Refactoring safe because tests catch regressions
- Comprehensive tests added after basics work

**User feedback**: "We may want to clarify the relationship between our dsp-test-writer and dsp-implementer to ensure they are ready to work together in harmony based on TDD principles."

### Decision 4: Combined audio-rust-debugger (vs separate)

**Design**: Single debugger for both audio-specific and Rust-specific issues

**Rationale**:
- **Significant overlap**: Many Rust bugs manifest as audio issues
  - Borrow checker violation → audio glitch
  - Panic in audio thread → plugin crash
  - Type mismatch → parameter scaling bug
- **Reduced context-switching**: Single agent understands full stack
- **Holistic debugging**: Best debuggers see the whole picture

**When to reconsider**: If debugging becomes bottleneck or if non-audio Rust projects added to repo

**User feedback**: "Should our debugger be competent with respect to both audio-specific AND Rust-specific debugging? Or do you see benefit to focusing solely on audio bugs?" → Confirmed combined approach.

### Decision 5: No team-manager (for now)

**Proposed**: Meta-agent evaluating team readiness and suggesting agent "training"

**Decision**: Skip initially, revisit after 10-20 features

**Rationale**:
- **Agents can self-report limitations**: "I'm not qualified for X, recommend consulting dsp-expert first"
- **User oversight valuable**: Natural feedback loop when agents struggle
- **Lighter alternative**: github-project-manager includes "readiness check" before assigning work
- **Premature optimization**: Add complexity only when proven necessary

**Future trigger**: If agents frequently miss context or lack needed expertise → Add team-manager

**User feedback**: "This sounds good, but who knows whether these styles of expertise... will be appropriate as we explore diverse DSP topics?... perhaps we could implement `team-manager`... experimental idea -- I'm completely open to any suggestions against this idea."

### Decision 6: Multiple Workflow Patterns (not single rigid pipeline)

**Design**: 6 workflow patterns triggered by GitHub issue labels

**Patterns**:
- **A (Full Research)**: type:feature-new-dsp - Complete research → implementation cycle
- **B (Implementation)**: type:feature-existing-dsp - Use existing research
- **C (Bug Fix)**: type:bug - Diagnose → fix → regression test
- **D (GUI)**: type:gui - GUI work without DSP
- **E (Optimization)**: type:optimization - Profile → optimize → verify
- **F (Research Only)**: type:research - Learn without implementing

**Rationale**:
- **Flexibility**: Not every task needs full research pipeline
- **Efficiency**: Bug fixes don't need Tier 1 research
- **Appropriate rigor**: Match workflow to task complexity
- **Iterative freedom**: Agents can ping-pong as needed within patterns

**User feedback**: "This sounds like one of many potential workflows that our agents could perform! For GitHub issues that are not feature requests, however, we may want to follow a different workflow!... we may want our dsp-implementer and test writers/reviewers/debuggers to iterate between each other more flexibly."

---

## Critical Design Patterns

### Pattern: Research-Backed Implementation

**Flow**: library/sources/ → library/research-notes/ → library/implementation-guides/ → shared/dsp-core/

**Example**:
1. research-assistant saves PolyBLEP paper as markdown
2. dsp-expert analyzes, creates `library/research-notes/oscillators/polyblep.md`
3. dsp-peer-reviewer validates against source
4. audio-engineer drafts `library/implementation-guides/oscillators/polyblep-guide.md`
5. dsp-implementer references guide while coding
6. Implementation includes: `/// Reference: library/research-notes/oscillators/polyblep.md`

**Benefit**: Permanent, citable knowledge base. Future oscillators reference existing research.

### Pattern: Real-Time Safety Gates

**Multiple checkpoints**:
1. **dsp-implementer**: Aware of no-allocation rules
2. **rust-audio-reviewer**: Scans for `Vec::new()`, `unwrap()`, `panic!()` in process()
3. **dsp-test-writer**: Includes real-time safety tests

**Benefit**: Layered defense against real-time violations. Catching early (implementation) better than late (production bug).

### Pattern: TDD Cycle Enforcement

**Structured handoffs**:
1. dsp-test-writer signals: "🔴 RED - 5 tests failing, ready for implementation"
2. dsp-implementer signals: "🟢 GREEN - all tests pass, ready for review"
3. rust-audio-reviewer signals: "Feedback provided, ready for refactor"
4. dsp-implementer signals: "♻️ REFACTOR complete, tests still pass"
5. dsp-test-writer signals: "✅ Comprehensive tests added, feature complete"

**Benefit**: Clear state transitions. No ambiguity about "done" criteria.

---

## Technology Integration

### Library Directory Structure

```
library/
├── sources/                  # Raw materials (research-assistant)
│   ├── oscillators/
│   │   ├── polyblep-valimaki-2007.md
│   │   └── polyblep-valimaki-2007-metadata.json
│   └── filters/
├── research-notes/           # Analysis (dsp-expert)
│   ├── oscillators/
│   │   └── polyblep.md
│   └── filters/
├── educational-articles/     # Teaching (dsp-expert)
│   └── understanding-polyblep.md
└── implementation-guides/    # Practical (audio-engineer)
    ├── oscillators/
    │   └── polyblep-guide.md
    └── filters/
```

### Workspace Cargo Structure

```
Cargo.toml (workspace)
├── shared/dsp-core/          # Pure DSP algorithms
├── shared/audio-utils/       # Cross-cutting utilities
├── shared/modulation/        # Modulation system
└── naughty-and-tender/      # Plugin (imports shared crates)
```

### Agent-to-Directory Mapping

| Agent | Primary Output Location |
|-------|------------------------|
| research-assistant | library/sources/ |
| dsp-expert | library/research-notes/, library/educational-articles/ |
| audio-engineer | library/implementation-guides/ |
| dsp-implementer | shared/dsp-core/ (pure DSP) or plugin/ (integration) |
| learning-documenter | docs/learnings/ |
| daw-test-coordinator | docs/test-plans/ |

---

## Future Enhancements

### MCP Server Integrations (Discussed, not implemented)

**Candidates**:
1. **Rust Documentation MCP**: Instant access to std/crates docs for rust-audio-reviewer, dsp-implementer
2. **Web Search MCP**: Enhanced search for dsp-researcher (beyond WebSearch/WebFetch)
3. **GitHub MCP**: Richer integration for github-project-manager
4. **PDF/File System MCP**: Let research-assistant save PDFs directly (currently markdown only)

**Decision**: Start with built-in tools (WebSearch, WebFetch, Bash with gh), add MCPs as specific needs emerge.

### Custom Skills (Discussed, not implemented)

**Candidates**:
1. **DSP Math Utilities**: LaTeX rendering, formula validation
2. **Audio Analysis**: FFT analysis, harmonic detection
3. **Code Generation**: Boilerplate for oscillators, filters, envelopes

**Decision**: Build after seeing repetitive patterns emerge in actual use.

### Custom Slash Commands (Discussed, not implemented)

**Candidates**:
1. `/research <topic>` - Trigger full research pipeline
2. `/implement <feature>` - Full development pipeline
3. `/optimize <component>` - Performance workflow
4. `/test-reaper <feature>` - Generate comprehensive Reaper test plan
5. `/extract <component>` - Component extraction guidance
6. `/validate-vst` - Run pluginval and fix issues

**Decision**: Document workflow patterns in README first, add slash commands if frequently used patterns emerge.

---

## Lessons from Planning Process

### What Worked Well

1. **Iterative refinement**: Started with rough proposal, refined based on user feedback
2. **Clear tier separation**: Research → Development → Documentation → Workflow
3. **Specific responsibilities**: Each agent has focused role
4. **Workflow flexibility**: Multiple patterns for different task types
5. **Architecture-first**: Workspace structure prevents extraction debt

### Design Challenges Resolved

1. **Research complexity**: Split into 4 agents (gather, analyze, validate, guide) instead of monolithic researcher
2. **Component extraction**: Replaced reactive extraction with proactive architecture
3. **Workflow rigidity**: Added 6 flexible patterns vs single pipeline
4. **Debugger scope**: Combined audio + Rust vs separate
5. **Team management**: Deferred meta-agent until proven necessary

### User Input Impact

User feedback significantly improved the design:
- Clarified Tier 1 research pipeline (4 agents vs 3)
- Shifted from component extraction to workspace architecture
- Emphasized TDD harmony between test-writer and implementer
- Confirmed combined debugger approach
- Validated multiple workflow patterns vs rigid pipeline
- Deferred team-manager (revisit later)

### Guiding Principles Established

1. **Research rigor first**: Don't code without theoretical foundation
2. **TDD always**: Tests define behavior before implementation
3. **Real-time safety gates**: Multiple checkpoints prevent audio thread violations
4. **Workspace architecture**: Correct structure from day 1
5. **Flexible workflows**: Match pattern to task complexity
6. **Learning capture**: Document insights for future reference
7. **Agent specialization**: Clear boundaries, no overlap

---

## Success Metrics

### How We'll Know This Works

**Research Tier Success**:
- [ ] library/ populated with high-quality, citable resources
- [ ] Implementation guides are actionable (developers can code from them)
- [ ] Educational articles aid understanding of concepts
- [ ] Research validated before coding begins

**Development Tier Success**:
- [ ] All DSP implementations pass TDD tests
- [ ] Zero real-time safety violations in production
- [ ] Code reviews catch issues before merging
- [ ] Performance targets met (<10% CPU with 8 voices)

**Documentation Tier Success**:
- [ ] Learnings captured after each feature
- [ ] Reaper test plans prevent DAW compatibility issues
- [ ] Future developers can understand past decisions

**Workflow Tier Success**:
- [ ] Issues routed to correct workflow automatically
- [ ] PRs include comprehensive descriptions
- [ ] VST validation catches compliance issues
- [ ] Dependencies stay current without conflicts

### Failure Modes to Watch

1. **Research bottleneck**: If research tier becomes too slow, consider lighter-weight option
2. **Agent confusion**: If agents frequently unsure of their role, refine descriptions
3. **Workflow overhead**: If simple tasks get buried in process, add fast-path
4. **Quality issues**: If bugs slip through gates, add checkpoints
5. **Knowledge gaps**: If agents lack expertise, add team-manager

---

## Next Steps

### Immediate (Completed)

- [x] Create 15 agent files with detailed SOPs
- [x] Create [.claude/agents/README.md](./.claude/agents/README.md) with workflow patterns
- [x] Create [docs/architecture.md](../architecture.md) with workspace structure
- [x] Update [.claudecode/prompt-playbook.md](../.claudecode/prompt-playbook.md) with agent examples
- [x] Create library/ directory structure
- [x] Document this planning discussion

### Near-Term (Next Session)

- [ ] Test agent system with first DSP feature (e.g., PolyBLEP oscillator)
- [ ] Validate workflow patterns in practice
- [ ] Identify any gaps in agent capabilities
- [ ] Refine agent descriptions based on usage

### Medium-Term (After 5-10 Features)

- [ ] Evaluate MCP integration needs
- [ ] Consider custom skills for repetitive tasks
- [ ] Assess if slash commands would improve workflow
- [ ] Review if team-manager needed

### Long-Term (After 10-20 Features)

- [ ] Comprehensive retrospective on agent effectiveness
- [ ] Identify underutilized agents (consolidate or remove)
- [ ] Identify missing capabilities (new agents or expand existing)
- [ ] Document best practices that emerged
- [ ] Share agent system with Rust audio community

---

## Open Questions

### For Future Consideration

1. **Agent iteration frequency**: How many back-and-forth cycles typical in TDD? (watch for bottlenecks)
2. **Research depth**: How deep should dsp-expert analyze before implementation? (balance rigor vs speed)
3. **Performance targets**: Are <10% CPU targets realistic? (adjust based on profiling)
4. **GUI framework**: Which Rust GUI framework best for audio plugins? (evaluate in practice)
5. **Workflow rigidity**: Do 6 patterns cover all cases? (add patterns as needs emerge)

### Questions to Answer Through Use

1. How often does Pattern A (full research) actually get used vs Pattern B (existing research)?
2. Does dsp-peer-reviewer catch meaningful errors or is it overhead?
3. Is audio-engineer's implementation guidance actionable enough?
4. Do developers actually reference library/ docs or are they ignored?
5. Is combined audio-rust-debugger effective or should it split?

---

## Reflections

### Why This Approach?

This agent system represents a **learning-focused, research-driven approach** to audio DSP development. Unlike typical "ship fast, iterate" software development, audio DSP requires:

1. **Mathematical correctness**: Bugs manifest as incorrect sound, not crashes
2. **Real-time constraints**: No room for allocations, locks, or unpredictable latency
3. **Deep understanding**: Can't debug what you don't understand
4. **Permanent knowledge**: Research is reusable across projects

The 4-tier architecture ensures:
- **Rigor** (Tier 1: research foundation)
- **Quality** (Tier 2: TDD + code review)
- **Learning** (Tier 3: documentation)
- **Efficiency** (Tier 4: workflow automation)

### Alignment with Project Goals

From [README.md](../README.md):
> "The goal is not to build the perfect synth, but to understand synthesis perfectly."

The agent system supports this by:
- Making research a **first-class activity** (Tier 1)
- Capturing learnings **systematically** (learning-documenter)
- Creating **educational materials** alongside code (dsp-expert)
- Building **permanent knowledge base** (library/)

---

## Acknowledgments

This agent system was designed collaboratively between:
- **User (colcavanaugh)**: Vision, requirements, architectural feedback, domain expertise
- **Claude (Sonnet 4.5)**: Agent design, workflow patterns, SOPs, documentation

Special thanks to:
- Anthropic for Claude Code agent capabilities
- Rust audio community for inspiring rigorous DSP development
- Open DSP resources (musicdsp.org, CCRMA Stanford) that make learning possible

---

**Document Status**: Planning complete, implementation phase beginning
**Last Updated**: October 18, 2025
**Next Review**: After first 5 features implemented with agent system
