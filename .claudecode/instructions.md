# Claude Code Instructions for Audio DSP Experiments

## Essential Context Documents

**Before starting any work session, read these documents to understand project context:**

### Always Review
- [README.md](../README.md) - Project overview, technology stack, repository structure, philosophy
- [docs/project-statement.md](../docs/project-statement.md) - Mission statement, learning objectives, success metrics, technical approach

### Current Project Work
- [naughty-and-tender/README.md](../naughty-and-tender/README.md) - Active project architecture, features, and implementation status
- [docs/naughty-and-tender.md](../docs/naughty-and-tender.md) - Detailed phases, learning goals, technical scope

### Project Management & Workflow
- [docs/project-management.md](../docs/project-management.md) - GitHub workflow, issue management, branching strategy, commit conventions

### Shared Components
- [shared/README.md](../shared/README.md) - Reusable DSP utilities strategy and extraction workflow

**Note:** When switching to different projects in the future, update the "Current Project Work" section above to reference the new active project's documentation.

## Project Context

This is a learning-focused audio DSP project. The primary goal is **understanding through implementation**, not perfection. Each plugin and tool serves as a vehicle for exploring synthesis, effects, and audio analysis concepts.

## Code Style & Conventions

### Rust Guidelines
- **Real-time safety is critical**: No allocations, locks, or blocking operations in audio processing callbacks
- Use `f32` for audio samples (industry standard for plugins)
- Prefer composition over inheritance (Rust idiom)
- Document public APIs with doc comments (`///`)
- Use `#[inline]` for hot-path functions (oscillators, filters)

### DSP Code Requirements
- **Always explain the math**: Complex DSP algorithms should have comments explaining the theory
- **Include references**: Link to papers, musicdsp.org, or other sources for non-trivial algorithms
- **Unit tests for accuracy**: Oscillator frequency, filter response, envelope timing should be tested
- **Performance matters**: Profile before optimizing, but keep real-time constraints in mind

### Example DSP Comment Style
```rust
/// State-variable filter implementation
/// Based on Hal Chamberlin's "Musical Applications of Microprocessors" (1980)
///
/// Transfer function: H(s) = ωc² / (s² + (ωc/Q)s + ωc²)
/// Digital approximation using trapezoidal integration
///
/// # Parameters
/// - `cutoff`: Cutoff frequency in Hz (20-20000)
/// - `resonance`: Q factor (0.5-20), higher = more resonance
#[inline]
fn process_svf(&mut self, input: f32, cutoff: f32, resonance: f32) -> f32 {
    // Implementation...
}
```

## Learning & Documentation

### Capture Learnings
When implementing new DSP concepts:
1. Document what worked and what didn't
2. Note surprising behaviors or gotchas
3. Identify reusable patterns for extraction to `shared/`
4. Update project-specific docs when major features complete

### Code Organization
- Keep DSP modules separate from GUI code
- Use clear module boundaries (oscillators, filters, effects, etc.)
- Extract reusable components only after they're proven in a working project

## Testing Approach

### What to Test
- **Critical DSP accuracy**: Oscillator frequency, filter response, envelope timing
- **Edge cases**: Division by zero, denormals, extreme parameter values
- **Real-time safety**: No panics, no allocations in audio thread

### What NOT to Over-Test
- GUI code (test manually in Reaper)
- Obvious getters/setters
- Experimental features still in flux

### Example Test
```rust
#[test]
fn oscillator_frequency_accuracy() {
    let mut osc = SineOscillator::new(44100.0);
    osc.set_frequency(440.0); // A4

    // Generate one second of audio
    let samples: Vec<f32> = (0..44100)
        .map(|_| osc.process())
        .collect();

    // Should complete exactly 440 cycles
    let zero_crossings = count_zero_crossings(&samples);
    assert!((zero_crossings - 880).abs() < 2); // 880 = 440 cycles * 2 crossings
}
```

## IDE Diagnostics & Error Detection

### Proactive Issue Detection
- **Always check IDE diagnostics** before and during coding
- Use type errors, syntax warnings, and linter feedback to guide implementation
- Fix diagnostic issues immediately rather than accumulating technical debt
- Leverage IDE's real-time feedback for DSP algorithm correctness

### Test Coverage Verification
- **After implementing any DSP feature**: Verify test coverage
- Critical DSP algorithms (oscillators, filters, envelopes) require comprehensive tests
- Run tests before marking feature complete
- Document test results in PR/commit messages

## Workflow Preferences

### When Starting New Features
1. Check if there's a GitHub issue for it
2. Create a feature branch if it's non-trivial
3. Start with the simplest implementation that could work
4. Test in Reaper early and often
5. Refactor after it works

### When Stuck
- Refer to `docs/` for project context
- Check musicdsp.org or DSP references
- Implement a naive version first, optimize later
- It's okay to ask for clarification on DSP theory

### Commit Style
Use conventional commits:
- `feat(naughty-and-tender): add saw oscillator`
- `fix(filters): prevent resonance explosion at high Q`
- `docs: update naughty-and-tender progress`
- `refactor(oscillators): extract phase accumulator`
- `test: add filter frequency response tests`

## DSP Domain Knowledge

### Key Concepts to Keep in Mind
- **Nyquist frequency**: Half the sample rate; frequencies above this alias
- **Anti-aliasing**: Required for anything but sine waves
- **Phase accumulation**: Core of oscillator implementation
- **Denormals**: Very small floats that kill performance; flush to zero
- **Parameter smoothing**: Prevent clicks when parameters change

### Common Pitfalls
- Forgetting to normalize phase (keep 0-1 or 0-2π)
- Not handling note-off (voices stuck on)
- Allocating in audio callback (use pre-allocated buffers)
- Dividing by zero (always check Q, frequencies, etc.)
- Ignoring denormals (use DAZ/FTZ flags)

## Communication Style

### Explanations
- **Complex DSP**: Explain the theory and reference sources
- **Standard patterns**: Brief comments sufficient
- **Tricky code**: Explain *why*, not just *what*

### When Implementing Features
- Mention which GitHub issue you're working on
- Note any deviations from the planned approach
- Highlight areas that might need future refactoring

## Project-Specific Notes

### naughty-and-tender
- Target: 8-16 voice polyphony
- Focus on clarity and learning over optimization (initially)
- Use nih-plug framework (chosen for good docs and active maintenance)
- GUI should be functional, not fancy

### shared/
- Only extract proven, stable components
- Each module should be its own crate
- Comprehensive tests required
- Clear API documentation

## Tools & Resources

### Testing in Reaper
- Use MIDI keyboard or piano roll for testing
- Check spectrum analyzer for harmonic content
- Use tuner plugin to verify oscillator pitch
- Test with automation to ensure parameter smoothing

### Performance Profiling
- Use `cargo flamegraph` for profiling
- Watch CPU usage in Reaper's performance meter
- Aim for <10% CPU with 8 voices active

### VST Validation
- Run pluginval before major releases
- Fix all errors, warnings are okay initially
- Test in Reaper with real musical use cases

---

**Remember**: The goal is learning through doing. Mistakes are expected and valuable. Document what you learn!
