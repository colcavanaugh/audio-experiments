# Naughty and Tender

**First Audio Plugin - MIDI Synthesizer**

## Project Vision

**A MIDI-triggered synthesizer designed for parameter exploration and hands-on learning.**

This first plugin serves as both a foundation and a playground—establishing the development workflow while providing maximum flexibility to experiment with synthesis techniques. Rather than chasing preset perfection, this project prioritizes understanding how different oscillator types, modulation sources, and processing stages shape sound.

## Core Concept

A polyphonic VST synthesizer that transforms MIDI input into audio through a flexible signal chain. The focus is on **clarity and experimentation**: every parameter should be accessible, every change should be audible, and every component should teach something fundamental about synthesis.

## Technical Scope

### Essential Features

#### 1. Oscillators (Multiple Types)
**Core Waveforms**
- Sine wave (pure tone, foundation)
- Sawtooth (bright, harmonic-rich)
- Square wave (hollow, odd harmonics)
- Triangle wave (softer than square)

**Advanced Algorithms** (pick 2-3 to start)
- FM synthesis (frequency modulation for complex timbres)
- Wavetable synthesis (morphing through stored waveforms)
- Additive synthesis (building sounds from harmonics)

**Architecture**: Support for multiple oscillators per voice with mixing

#### 2. Modulation System
**Sources**
- Envelopes (ADSR for amplitude, filter, and custom targets)
- LFOs (multiple shapes, rates, sync options)
- Velocity sensitivity
- MIDI CC (mod wheel, aftertouch, etc.)

**Destinations**
- Flexible modulation matrix
- Any parameter should be modulatable
- Visual feedback for modulation routing

#### 3. Filtering
- Low-pass filter (with resonance)
- High-pass filter (with resonance)
- Filter envelope (dedicated ADSR)
- Potential for additional filter types (band-pass, notch)

#### 4. Effects Chain
**Start with ONE, expand later**
- Option A: Reverb (space and depth)
- Option B: Delay (temporal effects)
- Option C: Distortion/saturation (harmonic excitement)

**Rationale**: Better to have one excellent effect than three mediocre ones

#### 5. Voice Management
- Polyphonic (target: 8-16 voices)
- Voice stealing algorithm (oldest/quietest)
- Monophonic mode option
- Glide/portamento

### GUI Design

**Philosophy**: Form follows function—utilitarian and clear over beautiful and confusing

**Layout Concept**
```
+------------------+------------------+
|   OSCILLATORS    |   MODULATION     |
|  [Type] [Mix]    | LFO1 [shape][→]  |
|  Osc1: [params]  | LFO2 [shape][→]  |
|  Osc2: [params]  | ENV1 [ADSR]      |
+------------------+------------------+
|     FILTER       |     EFFECTS      |
|  [Type] [Cutoff] |  [Type] [Mix]    |
|  [Resonance]     |  [Params...]     |
+------------------+------------------+
|  MASTER  [Volume] [Voices: 8]      |
+-------------------------------------+
```

**Requirements**
- All parameters visible at once (no hidden menus)
- Clear labeling
- Visual feedback for modulation
- Parameter values displayed numerically
- Keyboard playable for testing

## Technology Stack

### Language & Framework
- **Rust** - Primary implementation language
- **VST3 SDK** - Plugin format (via `vst3-rs` or similar)
- Consider: `nih-plug` framework for Rust VST development

### Audio DSP
- Sample rate handling (44.1kHz, 48kHz, 96kHz support)
- Real-time safe audio processing
- Anti-aliasing for oscillators
- Efficient voice management

### GUI Options
- **iced** - Rust-native, good for audio UIs
- **egui** - Immediate mode, flexible
- **vizia** - Audio-focused Rust GUI framework
- **imgui-rs** - Proven in audio applications

### Development Tools
- **Cargo** - Rust build system
- **Reaper** - Primary testing DAW
- **pluginval** - VST validation tool
- Unit tests for DSP algorithms

## Implementation Phases

### Phase 1: Foundation (Week 1-2)
- [ ] Project setup (Cargo, VST framework)
- [ ] Basic plugin shell loading in Reaper
- [ ] MIDI input handling (note on/off, velocity)
- [ ] Single sine wave oscillator
- [ ] Basic amplitude envelope

### Phase 2: Core Synthesis (Week 2-4)
- [ ] Multiple oscillator types (saw, square, triangle)
- [ ] Voice management and polyphony
- [ ] Oscillator mixing
- [ ] Basic GUI with parameter controls

### Phase 3: Modulation (Week 4-6)
- [ ] LFO implementation (multiple shapes)
- [ ] Filter with envelope
- [ ] Modulation matrix/routing
- [ ] Parameter automation support

### Phase 4: Advanced Features (Week 6-8)
- [ ] Advanced oscillator types (FM/wavetable/additive)
- [ ] Effects chain (choose one to implement)
- [ ] GUI refinement and visual feedback
- [ ] Preset management (basic save/load)

### Phase 5: Polish & Learning (Week 8+)
- [ ] Performance optimization
- [ ] Bug fixing and stability
- [ ] Documentation of learnings
- [ ] Extract reusable components for future projects

## Success Criteria

This project will be successful when:

1. ✅ **It works in Reaper**: Loads reliably, responds to MIDI, produces sound
2. ✅ **Parameters are understandable**: Each control has a clear, audible effect
3. ✅ **Code is learning-friendly**: Well-organized, commented, ready to reuse
4. ✅ **Sounds are explorable**: Can create diverse timbres through parameter tweaking
5. ✅ **Foundation is solid**: Workflow and architecture ready for next projects

## Learning Goals

Through building this plugin, aim to understand:

- **Oscillator anti-aliasing**: Why and how to prevent aliasing artifacts
- **Real-time audio constraints**: What's safe in the audio callback vs. UI thread
- **Modulation architecture**: Flexible routing without spaghetti code
- **Voice management**: Efficient polyphony and resource management
- **Filter design**: Different topologies and their sonic characteristics
- **Rust audio patterns**: Idiomatic Rust for real-time DSP

## Resources & References

### Rust Audio Ecosystem
- [nih-plug](https://github.com/robbert-vdh/nih-plug) - Modern Rust plugin framework
- [rust-vst](https://github.com/rust-dsp/rust-vst) - VST 2.4 bindings
- [fundsp](https://github.com/SamiPerttu/fundsp) - DSP library for Rust

### DSP References
- musicdsp.org - Algorithm implementations
- Katjaas - DSP tutorials
- Julius O. Smith III's online books (CCRMA Stanford)

### Synthesis Theory
- "Designing Software Synthesizer Plug-Ins in C++" - Will Pirkle
- "The Computer Music Tutorial" - Curtis Roads
- Synth Secrets series (Sound on Sound)

## Repository

**GitHub**: [colcavanaugh/naughty-and-tender](https://github.com/colcavanaugh/naughty-and-tender)

---

*A playground for synthesis exploration, designed to teach as much as it creates.*
