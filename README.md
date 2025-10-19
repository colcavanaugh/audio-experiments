# Audio DSP Experiments

> Building Sonic Literacy Through Implementation

A journey of exploration through audio synthesis, analysis, and manipulation. This repository houses a growing collection of audio plugins, tools, and utilities built in Rust—each one a hands-on investigation into the fundamentals of digital signal processing and sound design.

## Philosophy

This project prioritizes **learning through making**. Rather than chasing polished products, the focus is on understanding how parameters, algorithms, and architectural decisions shape sound. Each project builds on the last, developing both technical competence and sonic intuition.

**Core principles:**
- Incremental learning through iterative implementation
- Creative constraints that drive focused exploration
- Building reusable components for future work
- Experimentation over perfection
- Deep understanding through hands-on coding

## Current Projects

### [naughty-and-tender](./naughty-and-tender/)
*First MIDI Synthesizer Plugin*

A polyphonic VST synthesizer designed for parameter exploration. Features multiple oscillator types (sine, saw, square, triangle, plus FM/wavetable/additive), flexible modulation routing, filtering, and effects. Built to establish workflow and explore synthesis fundamentals.

**Status**: In development
**Tech**: Rust, VST3, Reaper
**Learning Focus**: Oscillators, modulation systems, voice management, real-time audio processing

[→ Full project documentation](./docs/naughty-and-tender.md)

## Learning Domains

This project explores DSP across several key areas:

**🎵 Synthesis**
- Oscillator algorithms and anti-aliasing
- Modulation systems (envelopes, LFOs, matrices)
- Voice management and polyphony
- FM, wavetable, additive, and granular synthesis

**🎚️ Effects & Processing**
- Filter design (IIR, FIR, state-variable)
- Time-based effects (delay, reverb, chorus)
- Dynamic processing (compression, saturation)
- Spectral processing

**📊 Analysis & Visualization**
- FFT and frequency domain analysis
- Amplitude and envelope detection
- Real-time metering and visualization
- MIDI and audio event processing

**🏗️ Architecture & Optimization**
- Plugin architecture (VST3, standalone)
- Real-time safe audio processing
- Parameter management and automation
- GUI frameworks and user interaction

## Repository Structure

This project uses a **Cargo workspace** architecture to maximize code reuse between plugins. Shared DSP components live in `shared/`, while plugin-specific code stays in individual plugin directories.

```
Experiments/
├── docs/                           # Project documentation
│   ├── project-statement.md       # Overall vision and goals
│   ├── architecture.md            # Code organization & workspace structure
│   ├── naughty-and-tender.md      # Specific project docs
│   └── project-management.md      # Workflow and processes
│
├── shared/                        # Shared DSP crates (reusable components)
│   ├── dsp-core/                  # Core DSP algorithms (oscillators, filters, envelopes)
│   ├── audio-utils/               # Audio utilities (smoothing, denormals, etc.)
│   └── modulation/                # Modulation system (LFOs, routing)
│
├── naughty-and-tender/            # First synthesizer plugin
│   ├── src/                       # Plugin-specific code
│   ├── Cargo.toml                 # Depends on shared crates
│   └── README.md                  # Project-specific readme
│
├── library/                       # Research & knowledge base
│   ├── sources/                   # Research materials
│   ├── research-notes/            # DSP analysis & theory
│   ├── educational-articles/      # Learning materials
│   └── implementation-guides/     # Practical implementation guidance
│
├── Cargo.toml                     # Workspace definition
└── README.md                      # This file
```

**[→ See architecture.md for detailed code organization rules](./docs/architecture.md)**

## Technology Stack

**Primary Language**: Rust
- Type safety and memory safety without garbage collection
- Excellent performance for real-time audio
- Growing ecosystem of audio libraries

**Plugin Format**: VST3
- Industry standard plugin format
- Supports automation, MIDI, and modern DAW integration

**Target DAW**: Reaper
- Lightweight, flexible, great for testing
- Excellent VST support

**Development Tools**
- Cargo (Rust build system)
- GitHub Projects (task management)
- pluginval (VST validation)
- Unit tests for DSP algorithms

## Quick Start

### Prerequisites
- Rust 1.75+ ([Install Rust](https://rustup.rs/))
- Reaper or another VST3-compatible DAW
- (Optional) VST3 SDK for development

### Building a Project

```bash
# Navigate to project directory
cd naughty-and-tender

# Build in release mode (optimized)
cargo build --release

# Run tests
cargo test

# The plugin will be output to target/release/
```

### Installing to DAW

```bash
# Copy plugin to VST3 directory (adjust path for your system)
# Windows: C:\Program Files\Common Files\VST3\
# macOS: ~/Library/Audio/Plug-Ins/VST3/
# Linux: ~/.vst3/

cp target/release/naughty_and_tender.vst3 [VST3_PATH]
```

## Project Management

This repository uses GitHub Projects for task tracking and planning. See [project-management.md](./docs/project-management.md) for detailed workflow documentation.

**Key organizational concepts:**
- **Projects**: Individual plugins/tools (naughty-and-tender, future projects)
- **Categories**: Synthesis, Effects, Analysis, Infrastructure, etc.
- **Learning Focus**: Specific DSP concepts being explored

[→ View all issues](https://github.com/colcavanaugh/audio-experiments/issues)

**Note**: GitHub Project board setup in progress - follow [this guide](./docs/github-project-setup-guide.md) to configure custom fields and views

## Learning Resources

Key references for this journey:

**Books & Papers**
- "The Audio Programming Book" - Boulanger & Lazzarini
- "Designing Software Synthesizer Plug-Ins in C++" - Will Pirkle
- Julius O. Smith III's online books (CCRMA Stanford)
- "The Computer Music Tutorial" - Curtis Roads

**Online Resources**
- [musicdsp.org](https://www.musicdsp.org/) - Algorithm implementations
- [Katjaas](https://www.katjaas.nl/home/home.html) - DSP tutorials
- Synth Secrets series (Sound on Sound)
- [Rust Audio Discord](https://discord.gg/QPdhk2u) - Community support

**Rust Audio Ecosystem**
- [nih-plug](https://github.com/robbert-vdh/nih-plug) - Modern plugin framework
- [fundsp](https://github.com/SamiPerttu/fundsp) - DSP library
- [cpal](https://github.com/RustAudio/cpal) - Cross-platform audio I/O

## Roadmap

### Phase 1: Synthesis Foundations (Months 1-2)
- ✅ Project setup and documentation
- 🔄 First synthesizer (naughty-and-tender)
- 🔄 Establish development workflow
- ⏳ Extract reusable oscillator components

### Phase 2: Effects & Processing (Months 2-3)
- ⏳ 2-3 focused effect plugins
- ⏳ Shared filter implementations
- ⏳ Time-based effects exploration

### Phase 3: Creative Exploration (Month 4+)
- ⏳ Generative/algorithmic tools
- ⏳ Analysis and visualization plugins
- ⏳ Integration of synthesis + effects

## Contributing

This is a personal learning project, but insights, suggestions, and discussions are welcome! Feel free to:
- Open issues for bugs or questions
- Share resources or learning materials
- Discuss DSP approaches and algorithms

## License

TBD - Will be determined as projects mature

## Acknowledgments

Built with inspiration from the Rust audio community and the wealth of open DSP knowledge available online. Special thanks to the creators of educational resources that make this learning journey possible.

---

**"The goal is not to build the perfect synth, but to understand synthesis perfectly."**

---

*Last updated: October 2025*
