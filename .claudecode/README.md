# Claude Code Configuration

This directory contains Claude Code configuration for the Audio DSP Experiments project.

## Files

### `settings.json`
Project-specific settings that help Claude understand:
- Project structure and focus
- Code style preferences (Rust, DSP-specific)
- Real-time safety requirements
- Testing approach
- Git workflow preferences

### `instructions.md`
Detailed guidelines for working on this project:
- DSP code conventions and commenting style
- Learning and documentation practices
- Testing strategy for audio code
- Common pitfalls and domain knowledge
- Workflow preferences

## Using These Settings

Claude Code will automatically read these files when working in this directory. The settings help ensure:

- **Real-time safety**: No allocations in audio callbacks
- **DSP documentation**: Complex algorithms explained with references
- **Learning focus**: Code organized for understanding, not just functionality
- **Testing**: Critical DSP accuracy validated with unit tests
- **Consistency**: Conventional commits, idiomatic Rust, clear structure

## Customization

Feel free to update these files as the project evolves:
- Add new conventions as patterns emerge
- Update focus as you move between subprojects
- Add domain-specific knowledge as you learn

---

*These settings help maintain consistency and capture best practices as the project grows.*
