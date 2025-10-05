# Project Management Guide

## Overview

This document outlines the workflow for managing multiple audio DSP projects within the Experiments monorepo using GitHub Projects and issues.

## Repository Structure

```
Experiments/
├── docs/                    # Project documentation
│   ├── project-statement.md
│   ├── naughty-and-tender.md
│   └── project-management.md
├── naughty-and-tender/      # First synthesizer project
├── shared/                  # Shared DSP utilities (future)
└── README.md
```

## GitHub Projects Setup

### Project Board Structure

We use a single GitHub Project board for the entire Experiments repository with custom organization.

### Custom Fields

**1. Project** (Single Select)
- naughty-and-tender
- shared-utilities
- future-project-2
- *(add new projects as they're created)*

**2. Category** (Single Select)
- Synthesis
- Effects
- Analysis
- GUI/UX
- Infrastructure
- Documentation
- Testing

**3. Learning Focus** (Text)
- Brief description of the DSP concept being explored
- Examples: "FM synthesis", "IIR filters", "voice stealing", "real-time safety"

**4. Priority** (Single Select)
- Critical
- High
- Medium
- Low

**5. Status** (Auto-managed)
- Backlog
- Todo
- In Progress
- Done

### Project Views

**View 1: Current Sprint**
- Filter: Status = "Todo" OR "In Progress"
- Group by: Status
- Sort by: Priority

**View 2: By Project**
- Filter: Status ≠ "Done"
- Group by: Project
- Sort by: Priority

**View 3: Learning Roadmap**
- Filter: All issues
- Group by: Category
- Sort by: Project, then Priority

**View 4: Completed**
- Filter: Status = "Done"
- Group by: Project
- Sort by: Closed date (newest first)

## Issue Management Workflow

### Creating Issues

**Issue Template Structure**:
```markdown
## Description
Brief description of the task or feature

## Learning Goals
What DSP concepts or programming patterns will this explore?

## Acceptance Criteria
- [ ] Specific, testable criteria
- [ ] Another criterion

## Resources
Links to relevant documentation, papers, or examples

## Notes
Additional context or implementation ideas
```

### Labels

Use GitHub labels for quick filtering:

**Type Labels**
- `bug` - Something isn't working
- `feature` - New functionality
- `enhancement` - Improvement to existing feature
- `documentation` - Documentation updates
- `refactor` - Code cleanup/reorganization

**Domain Labels**
- `dsp` - Core DSP algorithm work
- `audio` - Audio processing/callback work
- `midi` - MIDI handling
- `gui` - User interface
- `performance` - Optimization work

**Project Labels**
- `naughty-and-tender`
- `shared` - Affects shared utilities
- *(create new labels for each project)*

### Issue Lifecycle

1. **Create**: New issue added to Backlog
2. **Triage**: Add custom fields (Project, Category, Learning Focus, Priority)
3. **Plan**: Move to Todo when ready to work on
4. **Work**: Move to In Progress when actively developing
5. **Complete**: Move to Done when finished
6. **Review**: Periodically review Done items for learning extraction

## Milestones

Use GitHub Milestones to track major project phases:

### naughty-and-tender Milestones
- `v0.1 - Foundation` - Basic plugin loading and MIDI handling
- `v0.2 - Core Synthesis` - Oscillators and voice management
- `v0.3 - Modulation` - LFOs, envelopes, routing
- `v0.4 - Polish` - Effects, GUI refinement, optimization

### General Milestones
- `Shared Utilities v0.1` - First reusable components extracted
- `Infrastructure` - Development tooling and testing setup

## Workflow Best Practices

### Starting Work on an Issue
1. Assign yourself to the issue
2. Move to "In Progress" on the project board
3. Create a feature branch: `git checkout -b feature/issue-number-description`
4. Reference the issue in commits: `feat: implement sine oscillator (#12)`

### Completing Work
1. Ensure acceptance criteria are met
2. Write tests if applicable
3. Update documentation
4. Create pull request (even if self-merging, for history)
5. Link PR to issue using keywords: `Closes #12`
6. Move issue to Done

### Regular Maintenance

**Weekly Review**
- Triage new issues
- Update priorities based on current focus
- Review In Progress items
- Clean up stale issues

**Monthly Retrospective**
- Review completed work
- Extract learnings to project documentation
- Identify reusable components for `shared/`
- Plan next phase/milestone

## Cross-Project Considerations

### Shared Utilities
When building something reusable:
1. Create issue in both the current project AND `shared-utilities`
2. Initial implementation in project directory
3. Refactor and extract to `shared/` once proven
4. Update both issues when work is complete

### Dependencies Between Projects
- Use issue comments to link related issues across projects
- Tag dependencies: "Blocked by #X" or "Depends on #Y"
- Use GitHub's "Track with Projects" to connect related work

## Integration with Development

### Branch Strategy
- `main` - Stable, working code
- `develop` - Integration branch (optional, may not need initially)
- `feature/*` - Feature development
- `bugfix/*` - Bug fixes
- `docs/*` - Documentation updates

### Commit Message Format
```
type(scope): brief description

Longer description if needed

Refs: #issue-number
```

**Types**: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`
**Scope**: Project name or component (e.g., `naughty-and-tender`, `oscillators`, `gui`)

### Example
```
feat(naughty-and-tender): add FM oscillator implementation

Implements basic 2-operator FM synthesis with carrier/modulator
ratio controls and modulation depth parameter.

Learning focus: Understanding frequency modulation synthesis

Refs: #23
```

## Tools & Automation

### GitHub CLI
Use `gh` for efficient issue management:
```bash
# Create issue from template
gh issue create --project "Audio Experiments"

# List issues for current project
gh issue list --label "naughty-and-tender" --state open

# Update issue status
gh issue edit 23 --add-project "Audio Experiments"
```

### Project Automation
- Auto-add issues to project board when created
- Auto-move to "Done" when issue closed via PR
- Auto-assign based on labels (configure in repo settings)

## Documentation Integration

### Linking Issues to Docs
- Reference issues in documentation: `See issue #23 for FM implementation details`
- Link documentation in issues: Provide context with doc links

### Capturing Learnings
After completing major features:
1. Update project-specific doc (e.g., `naughty-and-tender.md`)
2. Add notes to `project-statement.md` if broadly applicable
3. Create `docs/learnings/` directory for detailed write-ups

## Measuring Progress

### Metrics to Track
- Issues closed per week/month
- Time spent per category (synthesis vs. effects vs. infrastructure)
- Number of reusable components extracted
- Project milestones completed

### Success Indicators
- Consistent forward progress across projects
- Clean, well-documented codebase
- Growing library of reusable utilities
- Demonstrable learning through completed features

---

*This guide evolves as the project scales. Update as new workflows emerge.*
