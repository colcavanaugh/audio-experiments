# Shared DSP Utilities

This directory will house reusable DSP components, utilities, and patterns extracted from individual projects.

## Purpose

As projects are built (naughty-and-tender, future plugins, etc.), common patterns and algorithms will emerge. Rather than duplicating code across projects, well-tested and reusable components will be extracted here.

## Future Contents

Expected utilities to be developed:

### Oscillators (`oscillators/`)
- Anti-aliased waveform generators
- Wavetable infrastructure
- Phase management utilities

### Filters (`filters/`)
- Common filter topologies (SVF, ladder, etc.)
- Filter coefficient calculators
- Resonance and drive implementations

### Modulation (`modulation/`)
- Envelope generators (ADSR, AD, multi-stage)
- LFO implementations
- Modulation routing utilities

### Effects (`effects/`)
- Delay lines and feedback structures
- Reverb algorithms
- Distortion/saturation functions

### Utilities (`utils/`)
- Parameter smoothing
- Voice management
- MIDI helpers
- DSP math utilities (interpolation, mapping, etc.)

### Audio I/O (`audio/`)
- Buffer management
- Sample rate conversion helpers
- Real-time safety utilities

## Development Approach

**Extract, Don't Predict**: Components will be extracted from working projects only after they've been proven useful, not created speculatively.

**Workflow**:
1. Build feature in specific project
2. Identify reusable pattern
3. Refactor for generality
4. Extract to `shared/`
5. Add comprehensive tests
6. Update project to use shared version
7. Document usage and API

## Organization

```
shared/
├── oscillators/
│   ├── src/
│   ├── tests/
│   └── Cargo.toml
├── filters/
├── modulation/
├── effects/
├── utils/
└── README.md  (this file)
```

Each module will be its own Rust crate, allowing projects to selectively depend on only what they need.

## Testing

All shared utilities must have:
- Unit tests for core functionality
- Integration tests demonstrating usage
- Benchmark tests for performance-critical code
- Documentation examples that compile and run

## Documentation

Each utility should document:
- Purpose and use cases
- API reference with examples
- Performance characteristics
- Known limitations
- References to papers/resources

---

*This directory will grow as the project matures. Check back as components are extracted from working plugins!*
