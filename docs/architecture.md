# Audio DSP Experiments - Project Architecture

> **Code Organization Guidelines for Audio Plugin Development**

This document defines the architectural principles and code organization rules for the Audio DSP Experiments project. Following these guidelines ensures clean separation of concerns, maximum code reuse, and maintainable plugin development.

---

## Table of Contents

1. [Workspace Structure](#workspace-structure)
2. [Code Organization Rules](#code-organization-rules)
3. [Integration Patterns](#integration-patterns)
4. [Development Workflow](#development-workflow)
5. [Testing Strategy](#testing-strategy)
6. [Examples](#examples)

---

## Workspace Structure

This project uses a **Cargo workspace** to manage shared DSP components and individual plugin projects.

### Directory Layout

```
Experiments/
├── Cargo.toml                      # Workspace definition
│
├── shared/                         # Shared DSP crates (reusable components)
│   ├── dsp-core/                  # Core DSP algorithms
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── oscillators/      # Oscillator implementations
│   │   │   ├── filters/          # Filter implementations
│   │   │   ├── envelopes/        # Envelope generators
│   │   │   └── utils/            # DSP utilities
│   │   ├── tests/                # Integration tests
│   │   ├── benches/              # Performance benchmarks
│   │   ├── Cargo.toml
│   │   └── README.md
│   │
│   ├── audio-utils/               # Cross-cutting audio utilities
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── smoothing.rs      # Parameter smoothing
│   │   │   ├── denormals.rs      # Denormal handling
│   │   │   ├── voice_pool.rs     # Voice management utilities
│   │   │   └── sample_rate.rs    # Sample rate utilities
│   │   └── Cargo.toml
│   │
│   └── modulation/                # Modulation routing system
│       ├── src/
│       │   ├── lib.rs
│       │   ├── lfo.rs            # LFO generators
│       │   ├── matrix.rs         # Modulation matrix
│       │   └── sources.rs        # Modulation sources
│       └── Cargo.toml
│
├── naughty-and-tender/            # First synthesizer plugin
│   ├── src/
│   │   ├── lib.rs                # Plugin entry point (nih-plug integration)
│   │   ├── gui/                  # Plugin-specific GUI code
│   │   │   ├── mod.rs
│   │   │   └── components.rs
│   │   ├── voice.rs              # Voice management (uses shared/dsp-core)
│   │   ├── params.rs             # Plugin parameter definitions
│   │   ├── processor.rs          # Audio processing (orchestrates shared crates)
│   │   └── presets.rs            # Preset management
│   ├── Cargo.toml                # Depends on shared crates
│   └── README.md
│
├── docs/                          # Project documentation
│   ├── architecture.md           # This file
│   ├── project-statement.md
│   └── project-management.md
│
└── library/                       # Research & knowledge base
    ├── sources/                   # Research materials
    ├── research-notes/            # DSP analysis & theory
    ├── educational-articles/      # Learning materials
    └── implementation-guides/     # Practical implementation guidance
```

### Root Cargo.toml (Workspace Definition)

```toml
[workspace]
members = [
    "shared/dsp-core",
    "shared/audio-utils",
    "shared/modulation",
    "naughty-and-tender",
    # Future plugins will be added here
]
resolver = "2"

[workspace.dependencies]
# Shared dependencies across all crates
nih-plug = "0.0.0"  # Replace with actual version
```

### Plugin Cargo.toml (Example: naughty-and-tender)

```toml
[package]
name = "naughty-and-tender"
version = "0.1.0"
edition = "2021"

[dependencies]
# Local workspace dependencies
dsp-core = { path = "../shared/dsp-core" }
audio-utils = { path = "../shared/audio-utils" }
modulation = { path = "../shared/modulation" }

# External dependencies
nih-plug = { workspace = true }

[lib]
crate-type = ["cdylib"]  # For VST3 plugin
```

---

## Code Organization Rules

### What Goes Where?

#### **shared/dsp-core/** - Pure DSP Algorithms

**Purpose**: Reusable DSP algorithms with no plugin, GUI, or DAW dependencies.

**Belongs Here**:
- ✅ Oscillator implementations (sine, saw, square, PolyBLEP, wavetable, FM)
- ✅ Filter algorithms (SVF, Moog ladder, biquad, etc.)
- ✅ Envelope generators (ADSR, AR, multi-stage)
- ✅ DSP utilities (phase accumulation, anti-aliasing, interpolation)
- ✅ Pure mathematical functions for audio processing

**Does NOT Belong Here**:
- ❌ GUI code
- ❌ Plugin framework integration (nih-plug, VST3 SDK)
- ❌ Parameter definitions
- ❌ DAW automation handling
- ❌ Preset management

**Criteria**:
- Zero dependencies on plugin frameworks
- Fully testable in isolation
- Generic over sample rates and buffer sizes
- Reusable across any plugin or standalone application

**Example Module** (`shared/dsp-core/src/oscillators/polyblep.rs`):

```rust
/// PolyBLEP antialiased sawtooth oscillator
/// Based on "Antialiasing Oscillators in Subtractive Synthesis" - Välimäki et al. (2007)
/// Reference: library/research-notes/oscillators/polyblep-antialiasing.md
pub struct PolyBLEPOscillator {
    phase: f32,
    sample_rate: f32,
}

impl PolyBLEPOscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self { phase: 0.0, sample_rate }
    }

    #[inline]
    pub fn process(&mut self, frequency: f32) -> f32 {
        // Pure DSP implementation - no plugin dependencies
        // ...
    }
}
```

---

#### **shared/audio-utils/** - Cross-Cutting Utilities

**Purpose**: Audio utilities that don't fit into specific DSP categories but are needed across plugins.

**Belongs Here**:
- ✅ Parameter smoothing (prevent clicks on parameter changes)
- ✅ Denormal handling (flush to zero, DAZ/FTZ)
- ✅ Voice stealing algorithms
- ✅ Sample rate conversion utilities
- ✅ Audio buffer management helpers
- ✅ MIDI utilities (note number to frequency, velocity scaling)

**Example** (`shared/audio-utils/src/smoothing.rs`):

```rust
/// Exponential parameter smoother to prevent audio clicks
/// Time constant controls smoothing speed
pub struct ParameterSmoother {
    current: f32,
    target: f32,
    coefficient: f32,
}

impl ParameterSmoother {
    pub fn new(sample_rate: f32, time_ms: f32) -> Self {
        // Implementation...
    }

    #[inline]
    pub fn set_target(&mut self, target: f32) {
        self.target = target;
    }

    #[inline]
    pub fn process(&mut self) -> f32 {
        // Smooth current value toward target
        // ...
    }
}
```

---

#### **shared/modulation/** - Modulation System

**Purpose**: Modulation sources and routing infrastructure.

**Belongs Here**:
- ✅ LFO generators (sine, triangle, saw, square, random)
- ✅ Modulation matrix/routing logic
- ✅ Modulation source abstractions
- ✅ Modulation destination handling
- ✅ Velocity/aftertouch processing

**Example** (`shared/modulation/src/lfo.rs`):

```rust
/// LFO generator with multiple waveform shapes
pub struct LFO {
    phase: f32,
    sample_rate: f32,
    shape: LFOShape,
}

pub enum LFOShape {
    Sine,
    Triangle,
    Sawtooth,
    Square,
    Random,
}

impl LFO {
    #[inline]
    pub fn process(&mut self, frequency: f32) -> f32 {
        // Generate modulation signal
        // ...
    }
}
```

---

#### **[plugin-name]/src/** - Plugin-Specific Code

**Purpose**: Plugin framework integration, GUI, parameters, and orchestration of shared components.

**Belongs Here**:
- ✅ Plugin entry point (`lib.rs` with nih-plug integration)
- ✅ Parameter definitions and automation handling
- ✅ GUI layout and controls (plugin-specific)
- ✅ Voice management (orchestrates `shared/dsp-core` components)
- ✅ Audio processor (combines shared DSP into plugin signal chain)
- ✅ Preset management and state serialization
- ✅ Plugin-specific configuration (voice count, routing, etc.)

**Example** (`naughty-and-tender/src/voice.rs`):

```rust
use dsp_core::oscillators::PolyBLEPOscillator;
use dsp_core::filters::SVFilter;
use dsp_core::envelopes::ADSREnvelope;

/// Single voice in the synthesizer
/// Orchestrates shared DSP components
pub struct Voice {
    oscillator: PolyBLEPOscillator,
    filter: SVFilter,
    amp_envelope: ADSREnvelope,
    // ... plugin-specific voice state
}

impl Voice {
    pub fn process(&mut self, params: &VoiceParams) -> f32 {
        // Combine shared DSP components into signal chain
        let osc_out = self.oscillator.process(params.frequency);
        let filtered = self.filter.process(osc_out, params.cutoff, params.resonance);
        let amp_env = self.amp_envelope.process();
        filtered * amp_env
    }
}
```

**Example** (`naughty-and-tender/src/lib.rs`):

```rust
use nih_plug::prelude::*;

// This is plugin-specific - integrates with nih-plug framework
struct NaughtyAndTender {
    params: Arc<NaughtyAndTenderParams>,
    voices: Vec<Voice>,
}

impl Plugin for NaughtyAndTender {
    // nih-plug integration code
    // This stays in plugin-specific directory
}
```

---

## Integration Patterns

### How Plugins Use Shared Components

#### Pattern 1: Direct Instantiation

```rust
// In plugin voice management
use dsp_core::oscillators::PolyBLEPOscillator;

pub struct Voice {
    osc: PolyBLEPOscillator,
}

impl Voice {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            osc: PolyBLEPOscillator::new(sample_rate),
        }
    }
}
```

#### Pattern 2: Trait-Based Polymorphism

```rust
// In shared/dsp-core
pub trait Oscillator {
    fn process(&mut self, frequency: f32) -> f32;
    fn reset(&mut self);
}

// Implement for various oscillator types
impl Oscillator for PolyBLEPOscillator { /* ... */ }
impl Oscillator for WavetableOscillator { /* ... */ }

// In plugin
pub struct Voice {
    osc: Box<dyn Oscillator>,  // Runtime polymorphism
}
```

#### Pattern 3: Generic Composition

```rust
// In plugin
pub struct Voice<O: Oscillator, F: Filter> {
    oscillator: O,
    filter: F,
}

// Allows compile-time specialization for performance
```

### Parameter Flow

```
DAW Automation
    ↓
Plugin Parameter (nih-plug)
    ↓
Parameter Smoother (shared/audio-utils)
    ↓
DSP Component (shared/dsp-core)
    ↓
Audio Output
```

---

## Development Workflow

### Adding a New DSP Component

#### Step 1: Research Phase
1. Research algorithm in `library/sources/`
2. Create analysis in `library/research-notes/`
3. Draft implementation guide in `library/implementation-guides/`

#### Step 2: Implement in `shared/dsp-core/`
```rust
// shared/dsp-core/src/filters/new_filter.rs

/// New filter implementation
/// Reference: library/implementation-guides/filters/new-filter-guide.md
pub struct NewFilter {
    // Pure DSP state
}

impl NewFilter {
    pub fn new(sample_rate: f32) -> Self {
        // Initialize
    }

    #[inline]
    pub fn process(&mut self, input: f32, params: &FilterParams) -> f32 {
        // Pure DSP processing
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_frequency_response() {
        // Test DSP accuracy in isolation
    }
}
```

#### Step 3: Integrate into Plugin
```rust
// naughty-and-tender/src/voice.rs
use dsp_core::filters::NewFilter;

pub struct Voice {
    filter: NewFilter,
}

impl Voice {
    pub fn process(&mut self, params: &VoiceParams) -> f32 {
        // Use shared component in plugin context
        self.filter.process(input, &params.filter_params)
    }
}
```

### Adding a New Plugin

1. **Add to workspace** (`Cargo.toml`):
   ```toml
   [workspace]
   members = [
       "shared/dsp-core",
       "shared/audio-utils",
       "shared/modulation",
       "naughty-and-tender",
       "new-plugin-name",  # Add here
   ]
   ```

2. **Create plugin directory**:
   ```
   new-plugin-name/
   ├── src/
   │   ├── lib.rs       # Plugin entry point
   │   ├── params.rs    # Parameter definitions
   │   └── processor.rs # Audio processing
   └── Cargo.toml       # Depends on shared crates
   ```

3. **Import shared components**:
   ```toml
   [dependencies]
   dsp-core = { path = "../shared/dsp-core" }
   audio-utils = { path = "../shared/audio-utils" }
   modulation = { path = "../shared/modulation" }
   ```

---

## Testing Strategy

### Unit Tests (Shared Crates)

Test DSP components in **isolation** within `shared/` crates:

```rust
// shared/dsp-core/tests/oscillator_tests.rs

#[test]
fn polyblep_frequency_accuracy() {
    let mut osc = PolyBLEPOscillator::new(44100.0);
    osc.set_frequency(440.0);

    // Generate one second
    let samples: Vec<f32> = (0..44100)
        .map(|_| osc.process())
        .collect();

    let zero_crossings = count_zero_crossings(&samples);
    assert!((zero_crossings - 880).abs() < 2); // 440 Hz * 2 crossings/cycle
}
```

### Integration Tests (Plugins)

Test plugin-specific behavior and DSP component orchestration:

```rust
// naughty-and-tender/tests/voice_tests.rs

#[test]
fn voice_envelope_timing() {
    let mut voice = Voice::new(44100.0);
    voice.note_on(60, 127);

    // Test attack phase duration
    // Test voice interaction with multiple components
}
```

### Manual Testing (DAW)

Follow test plans from `daw-test-coordinator` agent for Reaper validation.

---

## Examples

### Example 1: Implementing a New Oscillator

**Scenario**: Add a wavetable oscillator

**Step 1: Research** (handled by research team)
- `library/sources/wavetable/` - Gathered resources
- `library/research-notes/wavetable/` - Theoretical analysis
- `library/implementation-guides/wavetable/` - Implementation guidance

**Step 2: Implement in `shared/dsp-core/`**

```rust
// shared/dsp-core/src/oscillators/wavetable.rs

/// Wavetable oscillator with linear interpolation
/// Reference: library/implementation-guides/oscillators/wavetable-guide.md
pub struct WavetableOscillator {
    wavetable: Vec<f32>,
    phase: f32,
    sample_rate: f32,
}

impl WavetableOscillator {
    pub fn new(wavetable: Vec<f32>, sample_rate: f32) -> Self {
        Self { wavetable, phase: 0.0, sample_rate }
    }

    #[inline]
    pub fn process(&mut self, frequency: f32) -> f32 {
        // Wavetable lookup with interpolation
        let table_size = self.wavetable.len() as f32;
        let phase_increment = frequency / self.sample_rate;

        // Linear interpolation between table samples
        let index = self.phase * table_size;
        let index_floor = index.floor();
        let index_frac = index - index_floor;

        let i0 = index_floor as usize % self.wavetable.len();
        let i1 = (i0 + 1) % self.wavetable.len();

        let sample = self.wavetable[i0] * (1.0 - index_frac)
                   + self.wavetable[i1] * index_frac;

        // Advance phase
        self.phase += phase_increment;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }

        sample
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wavetable_playback() {
        // Create simple sine wavetable
        let wavetable: Vec<f32> = (0..1024)
            .map(|i| (2.0 * std::f32::consts::PI * i as f32 / 1024.0).sin())
            .collect();

        let mut osc = WavetableOscillator::new(wavetable, 44100.0);

        // Test frequency accuracy
        // ...
    }
}
```

**Step 3: Integrate into Plugin**

```rust
// naughty-and-tender/src/voice.rs
use dsp_core::oscillators::WavetableOscillator;

pub struct Voice {
    wavetable_osc: WavetableOscillator,
    // ... other components
}

impl Voice {
    pub fn new(sample_rate: f32) -> Self {
        // Load wavetable from plugin resources
        let wavetable = load_wavetable("preset_wave_1");

        Self {
            wavetable_osc: WavetableOscillator::new(wavetable, sample_rate),
        }
    }
}
```

---

### Example 2: Adding an Effect Processor

**Scenario**: Add a delay effect to `shared/dsp-core`

**Step 1: Implement in `shared/dsp-core/src/effects/delay.rs`**

```rust
/// Stereo delay with feedback and filtering
/// Reference: library/implementation-guides/effects/delay-guide.md
pub struct StereoDelay {
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    write_pos: usize,
    sample_rate: f32,
}

impl StereoDelay {
    pub fn new(max_delay_seconds: f32, sample_rate: f32) -> Self {
        let buffer_size = (max_delay_seconds * sample_rate) as usize;
        Self {
            buffer_l: vec![0.0; buffer_size],
            buffer_r: vec![0.0; buffer_size],
            write_pos: 0,
            sample_rate,
        }
    }

    #[inline]
    pub fn process(&mut self, input_l: f32, input_r: f32,
                   delay_time: f32, feedback: f32) -> (f32, f32) {
        // Pure DSP delay implementation
        // ...
    }
}
```

**Step 2: Use in Plugin**

```rust
// naughty-and-tender/src/processor.rs
use dsp_core::effects::StereoDelay;

pub struct Processor {
    delay: StereoDelay,
}

impl Processor {
    pub fn process_audio(&mut self, buffer: &mut AudioBuffer) {
        // Apply delay as part of plugin effect chain
        let (delayed_l, delayed_r) = self.delay.process(
            input_l,
            input_r,
            self.params.delay_time,
            self.params.feedback,
        );

        // Mix with dry signal based on plugin parameters
        output_l = input_l * (1.0 - mix) + delayed_l * mix;
        output_r = input_r * (1.0 - mix) + delayed_r * mix;
    }
}
```

---

## Best Practices

### Real-Time Safety

**In `shared/` crates**:
- ✅ NO allocations in `process()` functions
- ✅ Pre-allocate buffers in constructors
- ✅ Use fixed-size arrays or pre-sized `Vec`
- ✅ NO `panic!()`, `unwrap()`, or `expect()` in audio code
- ✅ NO mutex locks or other blocking operations

**Example**:
```rust
// ✅ GOOD: Pre-allocated buffer
pub struct Delay {
    buffer: Vec<f32>,  // Allocated once in new()
}

impl Delay {
    pub fn new(size: usize) -> Self {
        Self {
            buffer: vec![0.0; size],  // Allocation happens here
        }
    }

    #[inline]
    pub fn process(&mut self, input: f32) -> f32 {
        // No allocations here - just reads/writes
        self.buffer[self.pos] = input;
        // ...
    }
}

// ❌ BAD: Allocates in process()
#[inline]
pub fn process(&mut self) -> f32 {
    let temp = vec![0.0; 10];  // ALLOCATION IN AUDIO THREAD!
    // ...
}
```

### Documentation Standards

**All public DSP components must include**:
- Purpose and algorithm description
- Mathematical explanation (if non-trivial)
- Reference to research materials in `library/`
- Example usage
- Performance characteristics

```rust
/// State-variable filter (SVF) with simultaneous outputs
///
/// Implements the topology described in Hal Chamberlin's
/// "Musical Applications of Microprocessors" (1980).
///
/// **Transfer Function**: H(s) = ωc² / (s² + (ωc/Q)s + ωc²)
///
/// **Reference**: library/research-notes/filters/state-variable-filter.md
///
/// **Performance**: ~15 operations per sample (3 multiply, 2 add per output)
///
/// # Example
/// ```
/// let mut filter = SVFilter::new(44100.0);
/// let (lowpass, bandpass, highpass) = filter.process(input, 1000.0, 0.707);
/// ```
pub struct SVFilter {
    // ...
}
```

### Testing Requirements

**Critical DSP components require**:
- ✅ Frequency accuracy tests
- ✅ Edge case tests (zero, Nyquist, extreme parameters)
- ✅ Real-time safety tests (no allocations, no panics)
- ✅ Regression tests for bug fixes

---

## Summary

### Decision Tree: Where Does This Code Belong?

```
Is this code specific to a single plugin?
├─ YES → Plugin directory (naughty-and-tender/src/)
└─ NO → Continue...
    │
    Is this pure DSP algorithm (oscillator, filter, envelope)?
    ├─ YES → shared/dsp-core/
    └─ NO → Continue...
        │
        Is this audio utility (smoothing, denormals, etc.)?
        ├─ YES → shared/audio-utils/
        └─ NO → Continue...
            │
            Is this modulation-related (LFO, routing)?
            ├─ YES → shared/modulation/
            └─ NO → Discuss with team (may need new shared crate)
```

### Key Principles

1. **Shared code is pure DSP** - No plugin dependencies
2. **Plugin code orchestrates** - Combines shared components
3. **Test in isolation first** - Shared crates have comprehensive tests
4. **Real-time safety always** - No allocations in `process()` functions
5. **Document rigorously** - Reference research materials, explain math
6. **Organize by domain** - Oscillators, filters, effects clearly separated

---

**Questions or additions?** See [docs/project-management.md](./project-management.md) for workflow or open a GitHub issue for architectural discussions.
