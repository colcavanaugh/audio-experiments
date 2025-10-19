# Claude Code Prompt Playbook

**DSP-Specific Prompting Strategies for Effective Collaboration**

This playbook provides templates and strategies for getting the best results when working with Claude Code on audio DSP projects.

---

## General Principles

### 1. Be Specific About Context
✅ **Good**: "Implement a state-variable filter for the naughty-and-tender synth with resonance control"
❌ **Avoid**: "Add a filter"

### 2. Specify Learning Intent
✅ **Good**: "Help me implement FM synthesis - I want to understand the math behind modulator/carrier ratios"
❌ **Avoid**: "Add FM"

### 3. Reference Constraints
✅ **Good**: "Implement this oscillator with real-time safety - no allocations in audio callback"
❌ **Avoid**: "Make an oscillator"

---

## DSP Algorithm Implementation

### Template: New Algorithm Request
```
I need to implement [ALGORITHM_NAME] for [PROJECT_NAME].

**Learning Goals**: [What DSP concepts you want to understand]

**Requirements**:
- Sample rate: [e.g., 44.1kHz, variable]
- Real-time safety: [yes/no, specific constraints]
- Performance target: [e.g., <5% CPU for 8 voices]

**References**: [Links to papers, musicdsp.org, or other sources]

**Testing**: [What accuracy/behavior to verify]
```

### Example
```
I need to implement a BLEP anti-aliased sawtooth oscillator for naughty-and-tender.

**Learning Goals**: Understand how BLEP (Band-Limited Step) reduces aliasing artifacts

**Requirements**:
- Variable sample rate support (44.1-96kHz)
- Real-time safe (no allocations in process())
- <3% CPU per voice

**References**: musicdsp.org BLEP implementation, Stilson & Smith 1996

**Testing**: Verify harmonic content up to Nyquist, no aliasing above 20kHz
```

---

## Code Review Requests

### Template: Feature Review
```
Please review [FEATURE/FILE] for:
1. DSP correctness (math, algorithm implementation)
2. Real-time safety (allocations, locks, blocking)
3. Rust idioms and best practices
4. Performance concerns
5. Test coverage gaps

Focus especially on: [SPECIFIC_CONCERN]
```

### Template: Performance Review
```
Profile [FEATURE] and suggest optimizations.

Current performance: [MEASUREMENTS]
Target: [GOAL]

Constraints:
- Must remain real-time safe
- Accuracy within [TOLERANCE]
- [OTHER_CONSTRAINTS]
```

---

## Testing Requests

### Template: Test Implementation
```
Create comprehensive tests for [ALGORITHM/FEATURE].

**Test Coverage Required**:
- Frequency accuracy: [e.g., ±1 Hz at 440 Hz]
- Edge cases: [e.g., extreme resonance, zero frequency]
- Real-time safety: no panics, no allocations
- [OTHER_CRITERIA]

**Test approach**: [Unit tests, integration tests, or both]
```

### Example
```
Create comprehensive tests for the SVF filter implementation.

**Test Coverage Required**:
- Frequency accuracy: ±2% at cutoff frequency
- Edge cases: Q = 0, Q = 100, cutoff at Nyquist
- Real-time safety: no panics with denormal inputs
- Filter modes: LP, HP, BP all produce expected response

**Test approach**: Unit tests for each mode, integration test with modulation
```

---

## Debugging & Problem Solving

### Template: Debug Request
```
I'm experiencing [ISSUE] in [COMPONENT].

**Observed behavior**: [What's happening]
**Expected behavior**: [What should happen]
**Conditions**: [When does it occur]

**What I've tried**: [Your debugging attempts]

**Context**: [Relevant code, settings, environment]
```

### Template: Architecture Question
```
I'm designing [FEATURE] and considering these approaches:

**Option A**: [Description, pros/cons]
**Option B**: [Description, pros/cons]

**Constraints**: [Real-time, memory, complexity, etc.]
**Learning focus**: [What you want to understand]

Which approach would you recommend and why?
```

---

## Refactoring Requests

### Template: Extract Reusable Component
```
Extract [COMPONENT] from [PROJECT] to shared utilities.

**Requirements**:
- Generalize for reuse in future projects
- Maintain performance characteristics
- Add comprehensive tests
- Document API with examples

**Current usage**: [How it's used in current project]
**Future uses**: [Anticipated use cases]
```

---

## Documentation Requests

### Template: Algorithm Explanation
```
Explain [ALGORITHM] in [FILE/LOCATION].

**Audience**: [Your background level]
**Focus on**:
- Mathematical theory
- Implementation details
- Parameter relationships
- Common pitfalls

**Include**: References to papers/resources
```

### Template: Learning Capture
```
I've completed [FEATURE]. Help me document the key learnings.

**What worked well**: [Successes]
**What was challenging**: [Difficulties]
**Key insights**: [Discoveries]

**Extract**: Implementation patterns worth reusing
**Document**: In [TARGET_DOC]
```

---

## Planning & Architecture

### Template: Feature Planning
```
Plan implementation of [FEATURE] for [PROJECT].

**Use Planning Mode**

**Goals**: [What this feature should accomplish]
**Constraints**: [Technical, time, complexity]
**Learning objectives**: [DSP concepts to explore]

Break into phases with clear milestones.
```

### Template: Project Start
```
Starting work on [PROJECT/FEATURE].

**Context**: Read these docs first:
- [DOC_1]
- [DOC_2]

**Current focus**: [Specific area]
**Today's goal**: [Immediate objective]

Let's plan the approach before coding.
```

---

## Workflow Optimization

### Starting a Session
```
Beginning session on [PROJECT].

**Last session**: [What was completed]
**Today's focus**: [Current goals]
**GitHub issue**: #[NUMBER] (if applicable)

Please review context docs and suggest priorities.
```

### Wrapping Up
```
Session complete. Please help me:

1. Document learnings in appropriate docs
2. Update project status in README/docs
3. Identify components ready for extraction to shared/
4. Suggest next session priorities
```

---

## Common Pitfall Avoidance

### Anti-Patterns to Avoid

❌ **Vague requests**: "Make it better"
✅ **Specific goals**: "Reduce CPU usage by optimizing the filter coefficient calculation"

❌ **Assuming context**: "Fix the oscillator"
✅ **Provide context**: "Fix the sawtooth oscillator in src/oscillators.rs - it's producing aliasing above 15kHz"

❌ **No success criteria**: "Add tests"
✅ **Clear criteria**: "Add tests verifying frequency accuracy within ±1 Hz for 220-880 Hz range"

---

## Project-Specific Tips

### For naughty-and-tender
- Always mention real-time safety requirements
- Reference target voice count (8-16 voices)
- Specify Reaper as test environment
- Link to relevant sections of docs/naughty-and-tender.md

### For shared utilities
- Emphasize generality and reusability
- Request comprehensive API documentation
- Specify performance benchmarks
- Plan for diverse use cases

---

## Working with Specialized Agents

This project uses 15 specialized agents organized in 4 tiers. See [.claude/agents/README.md](../.claude/agents/README.md) for complete documentation.

### Agent Invocation Patterns

#### Explicit Invocation
```
"Use research-assistant to gather resources on PolyBLEP antialiasing"
"Have dsp-expert analyze the filter research materials"
"Ask rust-audio-reviewer to review the oscillator implementation"
```

#### TDD Workflow
```
"Use dsp-test-writer to create failing tests for PolyBLEP oscillator"
# → After tests written
"Use dsp-implementer to make the tests pass"
# → After implementation
"Use rust-audio-reviewer to review the code"
# → After review feedback
"Use dsp-implementer to refactor based on feedback"
```

#### Full Feature Workflow (New DSP Concept)
```
"Research and implement PolyBLEP antialiasing following Pattern A workflow"
# → Automatically routes through:
#    research-assistant → dsp-expert → dsp-peer-reviewer → audio-engineer
#    → dsp-test-writer (RED) → dsp-implementer (GREEN)
#    → rust-audio-reviewer → dsp-implementer (REFACTOR)
#    → dsp-test-writer (comprehensive) → performance-optimizer
#    → learning-documenter
```

#### Bug Fixing Workflow
```
"Debug the stuck note issue using audio-rust-debugger"
# → Automatically follows Pattern C:
#    audio-rust-debugger → dsp-implementer → dsp-test-writer (regression)
#    → rust-audio-reviewer
```

### Workflow Pattern Selection

**Use Pattern A (Full Research)** when:
- Implementing new DSP concept you haven't used before
- Need theoretical foundation before coding
- Building knowledge base for future reference

**Use Pattern B (Implementation Only)** when:
- Research already exists in `library/`
- Algorithm well-understood
- Just need to write code

**Use Pattern C (Bug Fix)** when:
- Fixing issues in existing code
- Need diagnosis and targeted fix
- Regression test needed

**Use Pattern D (GUI)** when:
- GUI work with no DSP changes
- Layout, controls, visual design

**Use Pattern E (Optimization)** when:
- Code works but needs performance improvement
- CPU usage above target
- Profiling and optimization needed

**Use Pattern F (Research Only)** when:
- Learning concept without immediate implementation
- Building knowledge base for future
- Exploring options before committing

### Multi-Agent Task Examples

#### Example 1: New Oscillator Type
```
"Implement wavetable oscillator for naughty-and-tender. This is a new DSP concept for me."

# → Pattern A automatically activated:
# 1. research-assistant gathers wavetable synthesis resources
# 2. dsp-expert analyzes and creates research notes
# 3. dsp-peer-reviewer validates accuracy
# 4. audio-engineer creates implementation guide
# 5. dsp-test-writer writes RED tests
# 6. dsp-implementer makes tests GREEN
# 7. rust-audio-reviewer checks code
# 8. dsp-implementer REFACTORs
# 9. dsp-test-writer adds comprehensive tests
# 10. daw-test-coordinator generates Reaper test plan
# 11. learning-documenter captures insights
```

#### Example 2: Quick Bug Fix
```
"Fix the filter resonance explosion at high Q values"

# → Pattern C automatically activated:
# 1. audio-rust-debugger diagnoses issue
# 2. dsp-implementer applies fix
# 3. dsp-test-writer creates regression test
# 4. rust-audio-reviewer validates fix
```

#### Example 3: Performance Optimization
```
"Optimize the oscillator to reduce CPU usage - currently at 12%, target <10%"

# → Pattern E automatically activated:
# 1. performance-optimizer profiles and suggests optimizations
# 2. dsp-implementer applies optimizations
# 3. dsp-test-writer verifies correctness maintained
# 4. rust-audio-reviewer checks optimizations are safe
```

### Agent-Specific Tips

#### When working with research-assistant
- Be specific about DSP concept: "PolyBLEP antialiasing" not just "oscillators"
- Mention known good sources if you have them
- Indicate if you need Rust-specific implementations

#### When working with dsp-implementer
- Ensure RED tests exist first (TDD workflow)
- Specify which crate (shared/dsp-core vs plugin directory)
- Reference implementation guide if available

#### When working with rust-audio-reviewer
- Request review after GREEN state achieved
- Mention specific concerns if any (e.g., "especially check phase handling")
- Be ready to iterate on feedback

#### When working with gui-designer
- Describe desired layout and functionality
- Mention which GUI framework (or ask for recommendation)
- Specify parameter control types needed

#### When working with github-project-manager
- Use issue labels to trigger automatic workflow routing
- Specify which agents should be assigned
- Reference related issues/PRs

### Best Practices with Agents

**DO**:
- ✅ Let agents specialize (don't ask implementer to write tests)
- ✅ Follow TDD workflow (RED → GREEN → REFACTOR)
- ✅ Trust workflow patterns for non-trivial features
- ✅ Use explicit invocation when you want specific agent

**DON'T**:
- ❌ Skip research for new DSP concepts (builds knowledge base)
- ❌ Skip code review (catches real-time safety violations)
- ❌ Rush through TDD (write tests first!)
- ❌ Forget to document learnings (future you will thank you)

### Troubleshooting Agent Workflows

**"Agent not activating"**:
- Try explicit invocation: "Use [agent-name] to..."
- Check if agent exists: see [.claude/agents/README.md](../.claude/agents/README.md)

**"Workflow too rigid"**:
- Agents can iterate flexibly (e.g., implementer ↔ debugger)
- Skip steps if truly not needed (document why)
- Workflows are guidelines, not hard rules

**"Too many steps for simple task"**:
- For trivial tasks, invoke specific agents directly
- Use Pattern B (implementation only) if research exists
- Agent overhead justified for non-trivial features

---

## Quick Reference

**Best Time to Use Planning Mode**:
- Starting new features (>3 steps)
- Complex algorithms (FM, granular, etc.)
- Architecture decisions
- Refactoring multiple components

**Always Specify**:
- Real-time constraints
- Performance targets
- Test coverage requirements
- Learning objectives

**Link to Resources**:
- GitHub issues
- Project documentation
- DSP references (papers, musicdsp.org)
- Previous related implementations

---

*This playbook evolves with the project. Add new patterns as they emerge!*
