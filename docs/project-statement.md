# Audio DSP Experimentation Project

## Mission Statement

**"Building Sonic Literacy Through Implementation"**

A hands-on exploration of audio synthesis, analysis, and manipulation through building working plugins and tools in Rust. This project is a journey of discovery—developing deep understanding of DSP fundamentals, audio programming patterns, and creative sound design possibilities through iterative experimentation and implementation.

## Philosophy

This is a learning-focused endeavor where **the process matters more than perfection**. Each project serves as a vehicle for understanding how different parameters, algorithms, and architectural decisions shape sound. The goal is to build sonic intuition through direct engagement with code and audio.

### Core Principles

- **Incremental Learning**: Start with fundamentals, build complexity gradually
- **Creative Constraints**: Use limitations to drive focused exploration
- **Building Reusable Components**: Extract patterns and utilities for future projects
- **Experimentation Over Perfection**: Prioritize trying new ideas over polished products
- **Understanding Through Making**: Deep comprehension comes from implementation

## Learning Objectives

Over the next few months, this project aims to explore:

### 1. Synthesis Fundamentals
- Basic oscillator types (sine, saw, square, triangle, noise)
- Advanced synthesis methods (FM, wavetable, additive, granular)
- Envelopes and modulation systems
- Voice management and polyphony

### 2. Audio Effects & Processing
- Filters (low-pass, high-pass, band-pass, resonance)
- Time-based effects (delay, reverb, chorus)
- Dynamic processing (compression, distortion, saturation)
- Spectral processing

### 3. Analysis & Visualization
- Frequency domain analysis (FFT, spectral visualization)
- Amplitude and envelope detection
- MIDI and audio event processing
- Real-time metering and visualization

### 4. System Architecture
- Plugin architecture (VST, standalone applications)
- Audio callback optimization and real-time safety
- Parameter management and automation
- GUI frameworks and user interaction patterns

## Technical Approach

### Technology Stack
- **Primary Language**: Rust (for performance, safety, and learning modern systems programming)
- **Plugin Format**: VST3 (with potential expansion to AU, CLAP)
- **Target DAW**: Reaper (primary testing environment)
- **Version Control**: Git + GitHub
- **Project Management**: GitHub Projects

### Code Organization
Projects will be organized as a monorepo with shared utilities:
- Individual projects as submodules/subdirectories
- Shared DSP utilities and common code
- Reusable GUI components
- Testing and benchmarking infrastructure

### Development Workflow
1. **Conceptualize**: Define learning goals for each project
2. **Prototype**: Build minimal viable implementation
3. **Experiment**: Iterate on parameters and features
4. **Refactor**: Extract reusable patterns and components
5. **Document**: Capture learnings and insights
6. **Reflect**: Evaluate what worked and what to try next

## Timeline & Scope

**Phase 1 (Months 1-2): Synthesis Foundations**
- First synthesizer plugin (naughty-and-tender)
- Explore oscillator types and modulation
- Establish development workflow and tooling

**Phase 2 (Months 2-3): Effects & Processing**
- Build 2-3 focused effect plugins
- Experiment with different processing approaches
- Develop shared filter and effect utilities

**Phase 3 (Month 4+): Creative Exploration**
- Combine synthesis and processing techniques
- Explore generative and algorithmic approaches
- Build tools for specific creative workflows

## Success Metrics

This project will be successful when:

1. **Technical Competence**: Comfortable implementing common DSP algorithms from scratch
2. **Working Toolkit**: Collection of functional plugins and tools used in actual music-making
3. **Sonic Intuition**: Can predict how parameter changes will affect sound
4. **Reusable Library**: Well-organized collection of DSP utilities and patterns
5. **Deep Understanding**: Can explain the "why" behind DSP decisions, not just the "how"

## Projects

### Current Projects
- [naughty-and-tender](./naughty-and-tender.md) - First MIDI synthesizer plugin

### Future Project Ideas
- Granular synthesis explorer
- Multi-mode filter playground
- Spectral analyzer/visualizer
- Generative MIDI sequencer
- Convolution reverb
- Tape saturation emulation

## Resources & Learning

Key resources for this journey:
- "The Audio Programming Book" - comprehensive DSP fundamentals
- "Designing Software Synthesizer Plug-Ins in C++" - synthesis architecture patterns
- Will Pirkle's DSP texts - practical algorithm implementations
- musicdsp.org - algorithm cookbook
- Rust audio community (Discord, forums) - practical guidance

## Notes

This document is a living guide that will evolve as the project progresses. Insights, pivots, and new directions will be captured here as they emerge.

---

*Last updated: October 2025*
