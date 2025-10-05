# Initial Issues for naughty-and-tender

Copy these issues to GitHub to bootstrap the project. Each issue follows the template from [project-management.md](./project-management.md).

---

## Issue 1: Project Setup & Rust Cargo Initialization

**Labels**: `infrastructure`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Infrastructure
**Priority**: Critical
**Learning Focus**: Rust project structure, VST framework selection

### Description
Set up the basic Rust project structure with Cargo and select/configure a VST framework for plugin development.

### Tasks
- [ ] Initialize Cargo project in `naughty-and-tender/`
- [ ] Research VST frameworks: `nih-plug`, `vst3-rs`, or `rust-vst`
- [ ] Choose framework based on: documentation, maintenance, ease of use
- [ ] Add framework dependency to `Cargo.toml`
- [ ] Create basic project structure (lib.rs, modules)
- [ ] Add development dependencies (testing, benchmarking)
- [ ] Create `README.md` for the project

### Acceptance Criteria
- [ ] `cargo build` completes successfully
- [ ] VST framework dependency resolved and compiling
- [ ] Basic project structure in place
- [ ] README documents chosen framework and rationale

### Resources
- [nih-plug framework](https://github.com/robbert-vdh/nih-plug)
- [vst-rs documentation](https://docs.rs/vst/)
- [Rust VST tutorial](https://www.seventeencups.net/posts/making-vst-plugins-in-rust/)

---

## Issue 2: Minimal Plugin Shell - Load in Reaper

**Labels**: `infrastructure`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Infrastructure
**Priority**: Critical
**Learning Focus**: VST plugin lifecycle, DAW integration

### Description
Create the absolute minimal VST plugin that can be loaded in Reaper. This validates the development environment and build process before adding any audio functionality.

### Tasks
- [ ] Implement VST plugin traits/interfaces
- [ ] Define basic plugin metadata (name, version, vendor)
- [ ] Implement empty audio process callback
- [ ] Build as VST3 plugin (.vst3 bundle)
- [ ] Copy plugin to Reaper's VST3 directory
- [ ] Load plugin in Reaper and verify it appears
- [ ] Document build and installation process

### Acceptance Criteria
- [ ] Plugin builds without errors
- [ ] Plugin appears in Reaper's plugin list
- [ ] Plugin loads without crashing Reaper
- [ ] Plugin can be instantiated on a track
- [ ] Build/install process documented

### Resources
- VST3 plugin directory locations (Windows: `C:\Program Files\Common Files\VST3\`)
- Reaper plugin scanner and debug info

---

## Issue 3: MIDI Input Handling

**Labels**: `feature`, `midi`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Infrastructure
**Priority**: Critical
**Learning Focus**: MIDI event processing, note on/off handling

### Description
Implement MIDI input handling to receive and process note on/off events with velocity information. This is foundational for triggering synthesis.

### Tasks
- [ ] Parse MIDI events from VST event buffer
- [ ] Implement note on handler (pitch, velocity)
- [ ] Implement note off handler
- [ ] Handle MIDI channel filtering (or omni mode)
- [ ] Log MIDI events for debugging
- [ ] Create voice trigger structure
- [ ] Add unit tests for MIDI parsing

### Acceptance Criteria
- [ ] Plugin receives MIDI note events from Reaper
- [ ] Note on events captured with correct pitch and velocity
- [ ] Note off events matched to corresponding note on
- [ ] MIDI events logged/printable for debugging
- [ ] No MIDI events dropped or mishandled

### Resources
- VST3 MIDI event specification
- MIDI message format (status byte, data bytes)

---

## Issue 4: Single Sine Wave Oscillator

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Synthesis
**Priority**: Critical
**Learning Focus**: Oscillator fundamentals, phase accumulation

### Description
Implement a basic sine wave oscillator that generates audio at the pitch specified by MIDI note number. This is the foundation for all synthesis.

### Tasks
- [ ] Implement phase accumulation algorithm
- [ ] Convert MIDI note number to frequency (A440 tuning)
- [ ] Generate sine wave using `sin()` or lookup table
- [ ] Handle sample rate properly (44.1kHz, 48kHz support)
- [ ] Output mono signal to both channels
- [ ] Add basic amplitude scaling
- [ ] Add unit tests for frequency accuracy

### Acceptance Criteria
- [ ] Oscillator generates audible sine tone in Reaper
- [ ] Pitch matches MIDI note number (verify with tuner)
- [ ] No clicks or pops during sustained notes
- [ ] Works at 44.1kHz and 48kHz sample rates
- [ ] Clean sine wave visible in spectrum analyzer

### Resources
- MIDI note to frequency formula: `f = 440 * 2^((n-69)/12)`
- Phase increment calculation: `freq / sample_rate`

---

## Issue 5: Amplitude Envelope (ADSR)

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Synthesis
**Priority**: High
**Learning Focus**: Envelope generation, envelope stages

### Description
Implement a standard ADSR (Attack, Decay, Sustain, Release) envelope generator to control amplitude over time for each voice.

### Tasks
- [ ] Create ADSR struct with parameters (A, D, S, R in ms/level)
- [ ] Implement attack stage (ramp from 0 to 1)
- [ ] Implement decay stage (ramp from 1 to sustain level)
- [ ] Implement sustain stage (hold at level)
- [ ] Implement release stage (ramp from current to 0)
- [ ] Trigger envelope on note on
- [ ] Trigger release on note off
- [ ] Apply envelope to oscillator output
- [ ] Add visualization or logging of envelope stages

### Acceptance Criteria
- [ ] Notes fade in smoothly (attack works)
- [ ] Envelope reaches sustain level correctly
- [ ] Notes fade out on release
- [ ] No clicks at envelope stage transitions
- [ ] Envelope resets properly for note retriggering

### Resources
- ADSR envelope theory
- Linear vs exponential envelopes (start with linear)

---

## Issue 6: Multiple Oscillator Types (Saw, Square, Triangle)

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Synthesis
**Priority**: High
**Learning Focus**: Basic waveform generation, aliasing awareness

### Description
Expand oscillator to support sawtooth, square, and triangle waveforms in addition to sine. Include basic anti-aliasing considerations.

### Tasks
- [ ] Implement naive sawtooth generator
- [ ] Implement naive square wave generator
- [ ] Implement naive triangle generator
- [ ] Create waveform selector (enum or parameter)
- [ ] Research and implement basic anti-aliasing (PolyBLEP or similar)
- [ ] Add parameter to switch waveform types
- [ ] Compare waveforms in spectrum analyzer
- [ ] Document aliasing issues and mitigation approaches

### Acceptance Criteria
- [ ] All four waveform types (sine, saw, square, tri) generate sound
- [ ] Waveforms sound distinctly different
- [ ] Anti-aliasing reduces harsh high-frequency content
- [ ] Waveform switching works without clicks
- [ ] Spectral content matches expected harmonic structure

### Resources
- PolyBLEP anti-aliasing algorithm
- Wavetable synthesis as anti-aliasing approach
- [musicdsp.org oscillator algorithms](https://www.musicdsp.org/en/latest/Synthesis/)

---

## Issue 7: Voice Management & Polyphony

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Synthesis
**Priority**: High
**Learning Focus**: Voice allocation, polyphony, voice stealing

### Description
Implement polyphonic voice management to allow multiple notes to sound simultaneously (8-16 voice target).

### Tasks
- [ ] Create Voice struct (oscillator, envelope, pitch, velocity)
- [ ] Create VoiceManager to allocate voices
- [ ] Implement voice assignment on note on
- [ ] Implement voice release on note off
- [ ] Implement voice stealing (oldest-note or quietest-voice strategy)
- [ ] Mix multiple voice outputs
- [ ] Add voice count parameter/display
- [ ] Handle edge case: all voices in use

### Acceptance Criteria
- [ ] Can play 8+ simultaneous notes
- [ ] Voice stealing occurs smoothly when voice limit exceeded
- [ ] Note on/off matched correctly to voices
- [ ] No audio glitches during voice allocation
- [ ] CPU usage reasonable with all voices active

### Resources
- Voice allocation strategies
- Voice stealing algorithms
- Rust ownership considerations for voice pool

---

## Issue 8: Basic GUI - Parameter Controls

**Labels**: `feature`, `gui`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: GUI/UX
**Priority**: High
**Learning Focus**: GUI framework integration, parameter binding

### Description
Create a minimal GUI with controls for essential parameters: oscillator type, ADSR envelope, volume. Start simple and functional.

### Tasks
- [ ] Research Rust GUI frameworks (iced, egui, vizia)
- [ ] Choose framework based on VST integration
- [ ] Implement GUI window/canvas
- [ ] Add dropdown for oscillator waveform selection
- [ ] Add 4 knobs/sliders for ADSR (A, D, S, R)
- [ ] Add master volume control
- [ ] Bind GUI parameters to DSP engine
- [ ] Implement parameter change notifications
- [ ] Test parameter changes while audio is playing

### Acceptance Criteria
- [ ] GUI displays in plugin window
- [ ] All parameters controllable via GUI
- [ ] Parameter changes immediately affect sound
- [ ] GUI updates when parameters change (automation support)
- [ ] No crashes or threading issues

### Resources
- [iced framework](https://github.com/iced-rs/iced)
- [egui framework](https://github.com/emilk/egui)
- [vizia framework](https://github.com/vizia/vizia)
- VST GUI integration patterns

---

## Issue 9: LFO Implementation

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Synthesis
**Priority**: Medium
**Learning Focus**: Low-frequency modulation, modulation sources

### Description
Implement one or more LFOs (Low-Frequency Oscillators) for modulation. Start with basic waveforms and rate control.

### Tasks
- [ ] Create LFO oscillator (reuse or adapt audio oscillator)
- [ ] Support multiple waveforms (sine, triangle, saw, square)
- [ ] Add rate parameter (Hz or sync to tempo)
- [ ] Add depth/amount parameter
- [ ] Implement phase reset on note trigger (optional)
- [ ] Create modulation output (bipolar -1 to 1, or unipolar 0 to 1)
- [ ] Add GUI controls for LFO parameters

### Acceptance Criteria
- [ ] LFO generates low-frequency control signal
- [ ] Multiple waveform shapes available
- [ ] Rate adjustable from ~0.1 Hz to ~20 Hz
- [ ] Depth scales modulation amount
- [ ] Ready to route to destinations (next issue)

### Resources
- LFO vs audio oscillator (frequency range difference)
- Tempo sync calculations (BPM to Hz)

---

## Issue 10: Modulation Matrix/Routing

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Synthesis
**Priority**: Medium
**Learning Focus**: Modulation architecture, parameter mapping

### Description
Create a flexible system for routing modulation sources (LFOs, envelopes, velocity) to synthesis parameters (pitch, filter cutoff, etc.).

### Tasks
- [ ] Design modulation routing data structure
- [ ] Define modulation sources (LFO1, LFO2, Env1, Velocity, etc.)
- [ ] Define modulation destinations (oscillator pitch, filter cutoff, etc.)
- [ ] Implement modulation scaling/depth per route
- [ ] Create modulation application function (source → destination)
- [ ] Add GUI for modulation routing (can be simple text/dropdowns initially)
- [ ] Test multiple simultaneous modulation routes
- [ ] Document modulation routing system

### Acceptance Criteria
- [ ] Can route LFO to oscillator pitch (vibrato)
- [ ] Can route velocity to filter cutoff (velocity sensitivity)
- [ ] Multiple modulation routes work simultaneously
- [ ] Modulation depth/amount adjustable per route
- [ ] System extensible for future modulation sources/destinations

### Resources
- Modulation matrix designs (studied in various synths)
- Parameter normalization (0-1 range) for consistent modulation

---

## Issue 11: Filter Implementation (Low-pass with Resonance)

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Effects
**Priority**: High
**Learning Focus**: Filter design, cutoff and resonance

### Description
Implement a resonant low-pass filter (Moog-style or state-variable filter) with cutoff frequency and resonance controls.

### Tasks
- [ ] Research filter algorithms (Moog ladder, SVF, one-pole)
- [ ] Choose filter topology (recommend SVF for flexibility)
- [ ] Implement filter with cutoff and resonance parameters
- [ ] Add filter to voice signal chain (after oscillator, before envelope or after)
- [ ] Add GUI controls for cutoff and resonance
- [ ] Test filter sweep and resonance behavior
- [ ] Add filter envelope (dedicated ADSR for cutoff)
- [ ] Document filter characteristics

### Acceptance Criteria
- [ ] Filter attenuates high frequencies as expected
- [ ] Cutoff frequency control works (audible sweep)
- [ ] Resonance adds emphasis at cutoff frequency
- [ ] Filter stable (no runaway oscillation) at high resonance
- [ ] Filter sounds musical and smooth

### Resources
- [Cytomic SVF filter](https://cytomic.com/files/dsp/SvfLinearTrapOptimised2.pdf)
- Moog ladder filter algorithm
- [musicdsp.org filter collection](https://www.musicdsp.org/en/latest/Filters/)

---

## Issue 12: High-pass Filter Support

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Effects
**Priority**: Medium
**Learning Focus**: Filter types, multi-mode filters

### Description
Add high-pass filter mode to the existing filter implementation, allowing selection between low-pass and high-pass.

### Tasks
- [ ] Extend filter to support high-pass output (if using SVF, already available)
- [ ] Add filter type selector (low-pass, high-pass)
- [ ] Update GUI with filter type control
- [ ] Test high-pass filtering behavior
- [ ] Consider adding band-pass and notch modes if SVF is used

### Acceptance Criteria
- [ ] High-pass mode attenuates low frequencies
- [ ] Filter type selector switches cleanly
- [ ] Both filter types work with resonance
- [ ] No audio artifacts when switching filter types

---

## Issue 13: Effects - Choose and Implement ONE

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Effects
**Priority**: Medium
**Learning Focus**: Time-based effects OR distortion

### Description
Choose and implement ONE effect to complete the signal chain. Options:
- **Option A**: Simple delay (delay line + feedback)
- **Option B**: Reverb (Freeverb or simple Schroeder reverb)
- **Option C**: Distortion/saturation (waveshaping or soft clipping)

Pick one based on learning interest.

### Tasks
- [ ] Choose effect type
- [ ] Research algorithm for chosen effect
- [ ] Implement effect processing
- [ ] Add to signal chain (post-filter)
- [ ] Add GUI controls (wet/dry mix + effect-specific params)
- [ ] Test effect with various inputs
- [ ] Document effect parameters and behavior

### Acceptance Criteria
- [ ] Effect audibly processes the sound
- [ ] Parameters control effect characteristics
- [ ] Wet/dry mix allows blending
- [ ] Effect doesn't introduce artifacts or instability
- [ ] Effect enhances sonic possibilities

### Resources
- **Delay**: Basic delay line implementation
- **Reverb**: [Freeverb algorithm](https://ccrma.stanford.edu/~jos/pasp/Freeverb.html)
- **Distortion**: [Waveshaping functions](https://www.musicdsp.org/en/latest/Effects/)

---

## Issue 14: Advanced Oscillator - FM Synthesis

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Synthesis
**Priority**: Medium
**Learning Focus**: Frequency modulation synthesis

### Description
Implement basic 2-operator FM synthesis (one carrier, one modulator) as an advanced oscillator type. This creates complex, inharmonic timbres.

### Tasks
- [ ] Research FM synthesis theory (carrier/modulator, ratio, index)
- [ ] Implement modulator oscillator (sine wave)
- [ ] Implement carrier oscillator (modulated by modulator)
- [ ] Add parameters: modulator ratio, modulation index/depth
- [ ] Add FM as oscillator type option
- [ ] Add GUI controls for FM parameters
- [ ] Experiment with different ratios and indices
- [ ] Document FM parameter behavior and sweet spots

### Acceptance Criteria
- [ ] FM oscillator generates complex timbres
- [ ] Modulation index controls brightness/harmonics
- [ ] Modulator ratio changes timbre character
- [ ] Can create bell-like and metallic sounds
- [ ] FM integrates with existing envelope and filter

### Resources
- [FM synthesis tutorial](https://learningsynths.ableton.com/en/fm-synthesis/what-is-fm-synthesis)
- Yamaha DX7 algorithm basics
- [musicdsp FM implementation](https://www.musicdsp.org/en/latest/Synthesis/169-fm-synthesis.html)

---

## Issue 15: Advanced Oscillator - Wavetable Synthesis

**Labels**: `feature`, `dsp`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Synthesis
**Priority**: Low
**Learning Focus**: Wavetable synthesis, interpolation

### Description
Implement wavetable synthesis: precomputed waveforms stored in tables, with smooth morphing between them. This allows evolving, complex timbres.

### Tasks
- [ ] Create wavetable storage structure (multiple single-cycle waveforms)
- [ ] Implement wavetable playback with interpolation
- [ ] Add wavetable position parameter (morph control)
- [ ] Generate or import initial wavetable set
- [ ] Implement linear interpolation between wavetables
- [ ] Add wavetable synthesis as oscillator option
- [ ] Add GUI control for wavetable position
- [ ] Consider allowing wavetable loading from files

### Acceptance Criteria
- [ ] Wavetable oscillator generates sound
- [ ] Morphing between wavetables sounds smooth
- [ ] Position parameter sweeps through timbres
- [ ] No aliasing or clicking during playback
- [ ] Wavetable position modulatable by LFO/envelope

### Resources
- [Wavetable synthesis guide](https://www.perfectcircuit.com/signal/learning-synthesis-wavetable)
- Adventure Kid waveforms (free wavetable set)
- Serum/Vital wavetable formats for reference

---

## Issue 16: Preset Management (Save/Load)

**Labels**: `feature`, `infrastructure`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Infrastructure
**Priority**: Low
**Learning Focus**: State serialization, file I/O

### Description
Implement basic preset save/load functionality to store and recall parameter settings. Start with simple JSON or binary format.

### Tasks
- [ ] Define preset data structure (all parameter values)
- [ ] Implement serialize preset to file
- [ ] Implement deserialize preset from file
- [ ] Add save preset button/menu to GUI
- [ ] Add load preset button/menu to GUI
- [ ] Handle file dialogs for preset management
- [ ] Create default preset directory
- [ ] Test saving and loading presets across sessions

### Acceptance Criteria
- [ ] Presets save all current parameter values
- [ ] Loading preset restores all parameters correctly
- [ ] Preset files are human-readable (if JSON) or documented (if binary)
- [ ] File I/O errors handled gracefully
- [ ] Presets persist across plugin sessions

### Resources
- Rust `serde` crate for serialization
- VST preset formats (can use custom format initially)

---

## Issue 17: Performance Optimization

**Labels**: `enhancement`, `performance`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Infrastructure
**Priority**: Low
**Learning Focus**: Real-time optimization, profiling

### Description
Profile and optimize the DSP code for real-time performance. Ensure low CPU usage and no audio dropouts under load.

### Tasks
- [ ] Set up benchmarking for audio processing
- [ ] Profile CPU usage with max polyphony
- [ ] Identify performance bottlenecks
- [ ] Optimize hot paths (oscillators, filters, envelopes)
- [ ] Consider SIMD optimizations if applicable
- [ ] Test performance at different sample rates and buffer sizes
- [ ] Ensure real-time safety (no allocations in audio thread)
- [ ] Document performance characteristics

### Acceptance Criteria
- [ ] Plugin runs smoothly with 8-16 voices active
- [ ] No audio dropouts or glitches under load
- [ ] CPU usage acceptable (< 10% on reference system)
- [ ] Performance documented and understood
- [ ] No allocations or locks in audio callback

### Resources
- Rust profiling tools (cargo flamegraph, perf)
- Real-time audio programming best practices
- SIMD in Rust (consider libraries like `wide` or manual intrinsics)

---

## Issue 18: Testing & Validation

**Labels**: `testing`, `infrastructure`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Testing
**Priority**: Medium
**Learning Focus**: Audio plugin testing, validation

### Description
Comprehensive testing of the plugin: unit tests for DSP, integration tests, and validation with pluginval.

### Tasks
- [ ] Write unit tests for oscillators (frequency accuracy)
- [ ] Write unit tests for envelopes (stage transitions)
- [ ] Write unit tests for filters (frequency response)
- [ ] Create integration tests (full voice processing)
- [ ] Run pluginval on built plugin
- [ ] Fix any issues found by pluginval
- [ ] Test in Reaper with real musical usage
- [ ] Document testing approach and coverage

### Acceptance Criteria
- [ ] All unit tests pass
- [ ] Integration tests cover main functionality
- [ ] Plugin passes pluginval without errors
- [ ] No crashes during extended use in Reaper
- [ ] Test coverage documented

### Resources
- [pluginval tool](https://github.com/Tracktion/pluginval)
- Rust testing documentation
- VST3 validation requirements

---

## Issue 19: Documentation & Learnings Extraction

**Labels**: `documentation`, `naughty-and-tender`
**Project**: naughty-and-tender
**Category**: Documentation
**Priority**: Medium
**Learning Focus**: Knowledge capture, reflection

### Description
Document the project: user guide, developer notes, and extracted learnings for future projects.

### Tasks
- [ ] Write user guide (how to use the plugin)
- [ ] Document all parameters and their ranges
- [ ] Write developer README (build, architecture, code organization)
- [ ] Create learning summary (what was learned, what worked, what didn't)
- [ ] Document reusable components for extraction to `shared/`
- [ ] Add code comments to complex DSP sections
- [ ] Create example presets/patches
- [ ] Update `docs/naughty-and-tender.md` with final status

### Acceptance Criteria
- [ ] User can understand how to use plugin from docs
- [ ] Developer can build and modify plugin from docs
- [ ] Learnings captured for future reference
- [ ] Reusable components identified
- [ ] Code is well-commented

---

## Issue 20: Identify & Extract Reusable Components to `shared/`

**Labels**: `refactor`, `infrastructure`, `naughty-and-tender`
**Project**: shared-utilities
**Category**: Infrastructure
**Priority**: Low
**Learning Focus**: Code reusability, library extraction

### Description
Review the naughty-and-tender codebase and extract reusable DSP components to the `shared/` directory for use in future projects.

### Tasks
- [ ] Identify reusable oscillator code
- [ ] Identify reusable envelope code
- [ ] Identify reusable filter code
- [ ] Refactor components for generality
- [ ] Create separate crates in `shared/` for each module
- [ ] Add comprehensive tests to shared components
- [ ] Update naughty-and-tender to depend on shared crates
- [ ] Document shared component APIs

### Acceptance Criteria
- [ ] Shared components compile independently
- [ ] Shared components have thorough tests
- [ ] naughty-and-tender uses shared components successfully
- [ ] Components documented for reuse in future projects
- [ ] Shared modules follow Rust best practices

### Resources
- Rust workspace organization
- Cargo local dependencies
- API design for libraries

---

## Summary

**Total Issues**: 20

**By Priority**:
- Critical: 5 issues (foundation)
- High: 5 issues (core features)
- Medium: 7 issues (enhancements)
- Low: 3 issues (polish)

**Suggested Order**:
1. Issues 1-5 (foundation: project setup → basic synthesis)
2. Issues 6-8 (core features: waveforms, polyphony, GUI)
3. Issues 9-11 (modulation and filtering)
4. Issues 12-17 (advanced features and optimization)
5. Issues 18-20 (testing, documentation, extraction)

Create these issues in GitHub and use the Project board to track progress!
