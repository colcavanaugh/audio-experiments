# Building Audio Plugins in Rust: The Plugin Shell

**Part 2 of the Naughty and Tender Development Series**

---

## What You'll Learn

By the end of this article, you'll understand:

- ✅ The architecture of VST3 and CLAP plugins
- ✅ How to implement the nih-plug `Plugin` trait
- ✅ Audio processing callbacks and real-time safety
- ✅ Parameter systems and automation
- ✅ GUI integration with egui
- ✅ Building and loading your plugin in a DAW

**Prerequisites:**
- [x] Completed [Article 1: Project Setup](./01-project-setup-and-why-rust.md)
- [x] Working Rust development environment
- [x] Workspace structure created
- [x] Basic understanding of audio concepts (sample rate, buffers)

**Time Estimate:** 60-90 minutes

---

## Introduction: What Is a Plugin, Really?

Before we write code, let's understand what we're building.

### The Plugin Architecture

Audio plugins are **shared libraries** (`.dll` on Windows, `.so` on Linux, `.dylib` on macOS) that export specific functions. The DAW (host) loads these libraries and calls the functions to:

1. **Initialize** the plugin
2. **Process audio** in real-time (the audio callback)
3. **Handle MIDI** events
4. **Expose parameters** for automation
5. **Display a GUI** for user interaction

> **💡 Tooltip: What's a Shared Library?**
>
> A shared library is compiled code that multiple programs can use. Unlike a standalone program, it doesn't have a `main()` function - the host program loads it dynamically and calls its exported functions. [Learn more](https://en.wikipedia.org/wiki/Library_(computing))

### Plugin Formats: VST3 vs. CLAP

**VST3** (Virtual Studio Technology 3):
- Industry standard by Steinberg
- Complex C++ API
- Supports advanced features (sample-accurate automation, sidechain routing)
- Required licensing (free but requires agreement)

**CLAP** (Clever Audio Plugin):
- Newer open-source standard
- Designed for modern plugin needs
- Free and open (no licensing)
- Growing adoption

nih-plug supports both formats from the same codebase - we'll export both!

> **📚 Further Reading:**
> - [VST3 Documentation](https://steinbergmedia.github.io/vst3_doc/)
> - [CLAP Specification](https://github.com/free-audio/clap)

### The Audio Thread: Real-Time Constraints

The most critical concept in plugin development is **real-time safety**.

When your DAW plays audio, the operating system calls your plugin's `process()` function 100+ times per second (depends on buffer size). This function must:

✅ **DO:**
- Process audio buffers
- Read parameter values
- Perform DSP calculations
- Use pre-allocated memory

❌ **DON'T:**
- Allocate memory (`Vec::new()`, `Box::new()`)
- Acquire locks that might block
- Perform I/O (file, network)
- Call system APIs that might sleep

**Why?** If `process()` takes too long, you'll hear glitches, pops, or dropouts. Real-time audio is unforgiving.

> **💡 Tooltip: What's a Buffer Size?**
>
> Audio is processed in chunks called buffers. At 48kHz sample rate with 512-sample buffers, `process()` is called every 10.67ms (512 ÷ 48000 = 0.01067s). Your code must finish in that time. [Learn more](https://support.native-instruments.com/hc/en-us/articles/209571729-What-is-a-Buffer-Size-and-How-Do-I-Change-It-)

Rust's ownership system helps enforce real-time safety, but **you** must still avoid allocations and blocking calls in the audio callback.

---

## Creating the Plugin Package

Let's create the naughty-and-tender plugin.

**Step 1: Create directory structure**

```bash
mkdir -p naughty-and-tender/src
```

**Step 2: Create `naughty-and-tender/Cargo.toml`**

```toml
# naughty-and-tender/Cargo.toml

[package]
name = "naughty-and-tender"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true

[lib]
crate-type = ["cdylib"]

[dependencies]
nih_plug = { workspace = true }
nih_plug_egui = { git = "https://github.com/robbert-vdh/nih-plug.git" }
shared-core = { workspace = true }
```

**Code Analysis:**

```toml
[lib]
crate-type = ["cdylib"]
```

> **`cdylib` explained**: A C-compatible dynamic library. This is what DAWs load. Rust has several crate types:
> - `lib`: Rust library (for other Rust code)
> - `cdylib`: C-compatible shared library (for plugins, FFI)
> - `bin`: Executable program
>
> Plugins must use `cdylib`. [Crate types documentation](https://doc.rust-lang.org/reference/linkage.html)

```toml
nih_plug_egui = { git = "https://github.com/robbert-vdh/nih-plug.git" }
```

> **GUI framework**: We're using **egui** (Emmediate mode GUI) via nih-plug's integration. Alternatives include `vizia` or custom OpenGL. egui is great for rapid development and is immediate-mode (no state management complexity).

---

## Implementing the Plugin Trait

The heart of any nih-plug plugin is the `Plugin` trait. Let's build it step by step.

**Create `naughty-and-tender/src/lib.rs`:**

```rust
//! Naughty and Tender - A MIDI Synthesizer Plugin
//!
//! First audio plugin - designed for parameter exploration and hands-on learning.

#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use nih_plug::prelude::*;
use std::sync::Arc;

mod editor;
mod params;

use params::NaughtyAndTenderParams;

/// The main plugin struct
pub struct NaughtyAndTender {
    params: Arc<NaughtyAndTenderParams>,
    sample_rate: f32,
}

impl Default for NaughtyAndTender {
    fn default() -> Self {
        Self {
            params: Arc::new(NaughtyAndTenderParams::default()),
            sample_rate: 44100.0,
        }
    }
}
```

**Code Analysis:**

```rust
mod editor;
mod params;
```

> **Module organization**: Rust encourages splitting code into modules. These declarations tell Rust to look for `editor.rs` and `params.rs` in the same directory as `lib.rs`. [Module system documentation](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

```rust
pub struct NaughtyAndTender {
    params: Arc<NaughtyAndTenderParams>,
    sample_rate: f32,
}
```

> **`Arc` explained**: Atomic Reference Counted pointer - shared ownership with thread-safe reference counting. We need `Arc` because both the audio thread and GUI thread access parameters. Regular `Rc` isn't thread-safe. [Arc documentation](https://doc.rust-lang.org/std/sync/struct.Arc.html)

Now let's implement the `Plugin` trait:

```rust
impl Plugin for NaughtyAndTender {
    const NAME: &'static str = "Naughty and Tender";
    const VENDOR: &'static str = "Col Cavanaugh";
    const URL: &'static str = "https://github.com/colcavanaugh/audio-experiments";
    const EMAIL: &'static str = "colcavanaugh@users.noreply.github.com";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    // Audio I/O configuration: stereo output, no input
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    // This is a synthesizer that responds to MIDI
    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    // ... (continued below)
}
```

**Code Analysis:**

```rust
const NAME: &'static str = "Naughty and Tender";
```

> **Plugin metadata**: These constants are compiled into the plugin. DAWs read them to display the plugin name, vendor, etc. in their plugin browsers.

```rust
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
```

> **`env!` macro**: Compile-time environment variable access. `CARGO_PKG_VERSION` is set by Cargo from `Cargo.toml`. This ensures the version is always in sync. [env! documentation](https://doc.rust-lang.org/std/macro.env.html)

```rust
main_input_channels: None,
main_output_channels: NonZeroU32::new(2),
```

> **Audio routing**: This plugin is a synthesizer - it has **no audio input** (it generates sound from MIDI) and **stereo output** (2 channels). Effects plugins would have both input and output. `NonZeroU32` is a type that guarantees the value isn't zero at compile time.

```rust
const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
```

> **MIDI configuration**: `Basic` means we receive note on/off, CC, pitch bend. `None` would disable MIDI. `MidiCc` would send MIDI parameters as CC messages.

```rust
const SAMPLE_ACCURATE_AUTOMATION: bool = true;
```

> **Sample-accurate automation**: When `true`, parameter changes are timestamped to specific samples within a buffer. This eliminates zipper noise and enables precise automation. VST3 and CLAP support this; older formats don't.

```rust
type SysExMessage = ();
type BackgroundTask = ();
```

> **Associated types**: nih-plug allows custom MIDI SysEx messages and background tasks (for async operations). We don't need them yet, so we use `()` (unit type - Rust's "nothing" type).

---

## The Initialize Method

Before processing audio, the DAW calls `initialize()`:

```rust
    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;

        nih_log!("Naughty and Tender initialized");
        nih_log!("Sample rate: {}", self.sample_rate);
        nih_log!("Max buffer size: {}", buffer_config.max_buffer_size);

        true
    }
```

**Code Analysis:**

```rust
&mut self,
```

> **Mutable reference**: `&mut` means this method can modify the plugin's state. We're storing the sample rate, so we need mutability.

```rust
_audio_io_layout: &AudioIOLayout,
```

> **Underscore prefix**: The `_` tells Rust "I know this parameter is unused, don't warn me." We don't need the I/O layout since we already defined it in `AUDIO_IO_LAYOUTS`, but the trait requires this parameter.

```rust
buffer_config: &BufferConfig,
```

> **BufferConfig**: Contains `sample_rate` (e.g., 48000.0) and `max_buffer_size` (e.g., 512). These are set by the DAW and vary based on user settings.

```rust
nih_log!("Sample rate: {}", self.sample_rate);
```

> **Debug logging**: `nih_log!` is nih-plug's logging macro. On Windows, use DebugView to see these messages. On Linux, they go to journalctl. Never use `println!` in real-time code (it allocates)!

```rust
true
```

> **Return value**: `true` means initialization succeeded. Returning `false` would tell the DAW the plugin failed to load.

---

## The Process Method: The Audio Callback

This is where the magic happens. The DAW calls `process()` continuously while audio is playing:

```rust
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Process MIDI events
        let mut next_event = context.next_event();
        let num_samples = buffer.samples();

        while let Some(event) = next_event {
            if event.timing() >= num_samples as u32 {
                break;
            }

            match event {
                NoteEvent::NoteOn {
                    timing: _,
                    voice_id: _,
                    channel: _,
                    note,
                    velocity,
                } => {
                    nih_log!("Note On: {} velocity: {}", note, velocity);
                    // TODO: Trigger voice with this note and velocity
                }
                NoteEvent::NoteOff {
                    timing: _,
                    voice_id: _,
                    channel: _,
                    note,
                    velocity: _,
                } => {
                    nih_log!("Note Off: {}", note);
                    // TODO: Release voice with this note
                }
                _ => {}
            }

            next_event = context.next_event();
        }

        // Get output buffer channels
        let output = buffer.as_slice();

        // For now, just output silence
        // In the next phase, we'll render oscillators here
        for channel_samples in output {
            channel_samples.fill(0.0);
        }

        ProcessStatus::Normal
    }
```

**Code Analysis:**

```rust
buffer: &mut Buffer,
```

> **The audio buffer**: This contains the audio samples to process. For synthesizers, it starts empty (or with previous audio). We fill it with our generated sound. For effects, it contains input audio to modify.

```rust
context: &mut impl ProcessContext<Self>,
```

> **Process context**: Provides access to:
> - MIDI events (`next_event()`)
> - Transport info (tempo, playhead position)
> - Parameter values
> - Timing information

```rust
let mut next_event = context.next_event();
```

> **MIDI event iteration**: nih-plug provides MIDI events in a stream. We pull one at a time with `next_event()`, which returns `Option<NoteEvent>` - `Some(event)` if there's an event, `None` when done.

```rust
if event.timing() >= num_samples as u32 {
    break;
}
```

> **Sample-accurate timing**: Each event has a `timing()` - the sample offset within this buffer where it occurs. If timing >= buffer size, it belongs to the next buffer. This ensures precise MIDI timing.

```rust
match event {
    NoteEvent::NoteOn { note, velocity, ... } => {
```

> **Pattern matching**: Rust's `match` is like a supercharged switch statement. We destructure the `NoteOn` variant to extract `note` (MIDI note number 0-127) and `velocity` (0.0-1.0 normalized).

```rust
channel_samples.fill(0.0);
```

> **Silence for now**: Phase 1 outputs silence. In Phase 2, we'll replace this with oscillator code. The `fill()` method sets all samples to 0.0 (digital silence).

```rust
ProcessStatus::Normal
```

> **Return status**: Tells the DAW processing succeeded. Other options:
> - `ProcessStatus::Error`: Critical failure
> - `ProcessStatus::Tail(samples)`: Plugin has a reverb tail
> - `ProcessStatus::KeepAlive`: Keep processing even when silent

### Real-Time Safety Analysis

Let's verify this code is real-time safe:

✅ **No allocations**: `fill()` uses pre-allocated buffers
✅ **No locks**: No `Mutex` or `RwLock` usage
✅ **No I/O**: Just processing samples
✅ **No blocking calls**: Everything is deterministic

This code is safe for the audio thread!

---

## The Parameter System

Parameters are how users control your plugin. They can be:
- Automated by the DAW
- Controlled by MIDI CC
- Adjusted via GUI

**Create `naughty-and-tender/src/params.rs`:**

```rust
//! Plugin parameters for Naughty and Tender

use nih_plug::prelude::*;
use nih_plug_egui::EguiState;
use std::sync::Arc;

/// All plugin parameters
#[derive(Params)]
pub struct NaughtyAndTenderParams {
    /// Editor state for saving/restoring GUI position and size
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,

    /// Master gain control (in dB)
    #[id = "gain"]
    pub gain: FloatParam,

    /// Number of active voices (read-only display parameter)
    #[id = "voices"]
    pub voice_count: IntParam,
}

impl Default for NaughtyAndTenderParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(500, 400),

            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(6.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 6.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            voice_count: IntParam::new("Voices", 0, IntRange::Linear { min: 0, max: 16 })
                .with_value_to_string(Arc::new(|value| format!("{value}")))
                .non_automatable(),
        }
    }
}
```

**Code Analysis:**

```rust
#[derive(Params)]
```

> **Derive macro**: This generates the boilerplate code to implement nih-plug's `Params` trait. Derive macros are Rust's way of automatic code generation. [Derive documentation](https://doc.rust-lang.org/book/ch05-02-example-structs.html#adding-useful-functionality-with-derived-traits)

```rust
#[persist = "editor-state"]
pub editor_state: Arc<EguiState>,
```

> **Persistence**: The `#[persist]` attribute tells nih-plug to save this field to the plugin state. When you close and reopen the DAW, your GUI size/position is restored. The string `"editor-state"` is a unique ID.

```rust
#[id = "gain"]
pub gain: FloatParam,
```

> **Parameter ID**: Each parameter needs a unique ID. The DAW uses these to save automation. **Never change these IDs** after release, or you'll break users' projects!

```rust
FloatRange::Skewed {
    min: util::db_to_gain(-30.0),
    max: util::db_to_gain(6.0),
    factor: FloatRange::gain_skew_factor(-30.0, 6.0),
}
```

> **Skewed range**: Gain controls feel better with a logarithmic scale. `-30 dB to +6 dB` is a typical range. The skew factor makes the slider allocate more resolution to lower values (where human hearing is more sensitive).

```rust
.with_smoother(SmoothingStyle::Logarithmic(50.0))
```

> **Parameter smoothing**: When a parameter changes, smoothing interpolates from old to new value over 50ms. This prevents clicks and zipper noise. Logarithmic smoothing is ideal for gain controls.

```rust
.with_value_to_string(formatters::v2s_f32_gain_to_db(2))
.with_string_to_value(formatters::s2v_f32_gain_to_db())
```

> **Display formatters**: Internally, gain is stored linear (0.0-1.0 range). These formatters convert to/from dB for display. `v2s` = value to string, `s2v` = string to value. The `2` parameter means 2 decimal places.

```rust
.non_automatable()
```

> **Non-automatable parameters**: The voice count is just a display - users can't automate it. This hides it from the DAW's automation menus.

---

## The GUI: Egui Integration

Let's create a simple GUI to display status and control parameters.

**Create `naughty-and-tender/src/editor.rs`:**

```rust
//! Editor/GUI for Naughty and Tender

use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, egui, widgets, EguiState};
use std::sync::Arc;

use crate::params::NaughtyAndTenderParams;

/// Create the plugin editor
pub(crate) fn create(
    params: Arc<NaughtyAndTenderParams>,
    editor_state: Arc<EguiState>,
) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        editor_state,
        (),
        |_, _| {},
        move |egui_ctx, setter, _state| {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.heading("Naughty and Tender");
                ui.add_space(10.0);

                ui.label("MIDI Synthesizer - Phase 1: Plugin Shell");
                ui.add_space(20.0);

                // Master section
                ui.group(|ui| {
                    ui.label("Master");
                    ui.add_space(5.0);

                    // Gain parameter
                    ui.label("Gain");
                    ui.add(widgets::ParamSlider::for_param(&params.gain, setter));

                    ui.add_space(5.0);

                    // Voice count display
                    ui.label("Active Voices");
                    ui.add(widgets::ParamSlider::for_param(
                        &params.voice_count,
                        setter,
                    ));
                });

                ui.add_space(20.0);

                // Status information
                ui.group(|ui| {
                    ui.label("Status");
                    ui.add_space(5.0);

                    ui.label("✅ Plugin loaded successfully");
                    ui.label("✅ MIDI input configured");
                    ui.label("✅ Audio callback active");
                    ui.label("⏳ Voice management (coming in Phase 2)");
                    ui.label("⏳ Oscillators (coming in Phase 2)");
                });

                ui.add_space(20.0);

                // Instructions
                ui.group(|ui| {
                    ui.label("Instructions");
                    ui.add_space(5.0);

                    ui.label("This is Phase 1: Plugin Shell");
                    ui.label("• Plugin loads in your DAW");
                    ui.label("• MIDI events are logged (check console)");
                    ui.label("• Audio pass-through is active (currently silence)");
                    ui.label("• Parameters can be automated");
                    ui.add_space(5.0);
                    ui.label("Next: Phase 2 will add oscillators and synthesis!");
                });
            });
        },
    )
}
```

**Code Analysis:**

```rust
pub(crate) fn create(
```

> **`pub(crate)` visibility**: Public within this crate, but not exported to other crates. The GUI creation function is internal implementation detail.

```rust
create_egui_editor(
    editor_state,
    (),
    |_, _| {},
    move |egui_ctx, setter, _state| {
```

> **Editor factory**: `create_egui_editor` is nih-plug's helper for egui integration. Parameters:
> 1. `editor_state`: Window size/position
> 2. `()`: User state (we don't need any)
> 3. `|_, _| {}`: Update closure (runs before rendering)
> 4. `move |...| { ... }`: Render closure (draws the GUI)

```rust
move |egui_ctx, setter, _state| {
```

> **`move` closure**: Captures `params` by moving it into the closure. This is necessary because the closure outlives the function call. [Closures documentation](https://doc.rust-lang.org/book/ch13-01-closures.html)

```rust
egui::CentralPanel::default().show(egui_ctx, |ui| {
```

> **egui layout**: `CentralPanel` is the main GUI area. egui uses immediate mode - you define the UI in code every frame, no separate state management. [egui documentation](https://docs.rs/egui/)

```rust
ui.add(widgets::ParamSlider::for_param(&params.gain, setter));
```

> **Parameter widgets**: `ParamSlider` is a nih-plug widget that connects to parameter automation. The `setter` handles automation recording. Moving this slider records automation in your DAW!

---

## Exporting the Plugin

Finally, we need to tell nih-plug to export VST3 and CLAP formats.

**Add to the end of `naughty-and-tender/src/lib.rs`:**

```rust
impl ClapPlugin for NaughtyAndTender {
    const CLAP_ID: &'static str = "com.colcavanaugh.naughty-and-tender";
    const CLAP_DESCRIPTION: Option<&'static str> =
        Some("A MIDI synthesizer for parameter exploration and learning");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for NaughtyAndTender {
    const VST3_CLASS_ID: [u8; 16] = *b"NaughtyAndTender";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(NaughtyAndTender);
nih_export_vst3!(NaughtyAndTender);
```

**Code Analysis:**

```rust
const CLAP_ID: &'static str = "com.colcavanaugh.naughty-and-tender";
```

> **CLAP ID**: Reverse-DNS style unique identifier. Use your domain (or GitHub username). This prevents ID collisions with other plugins.

```rust
const CLAP_FEATURES: &'static [ClapFeature] = &[
    ClapFeature::Instrument,
    ClapFeature::Synthesizer,
    ClapFeature::Stereo,
];
```

> **Feature tags**: Help DAWs categorize your plugin. `Instrument` = generates audio, `Synthesizer` = specific type, `Stereo` = stereo output.

```rust
const VST3_CLASS_ID: [u8; 16] = *b"NaughtyAndTender";
```

> **VST3 Class ID**: A 16-byte unique identifier. The `b"..."` syntax creates a byte string. **NEVER CHANGE THIS** after release - it's how DAWs identify your plugin!

```rust
nih_export_clap!(NaughtyAndTender);
nih_export_vst3!(NaughtyAndTender);
```

> **Export macros**: These generate the C ABI functions that DAWs call to load the plugin. They're the bridge between Rust and the plugin APIs.

---

## Building the Plugin

Time to compile!

**Step 1: Verify it compiles**

```bash
cargo check --package naughty-and-tender
```

You should see:
```
Checking naughty-and-tender v0.1.0
Finished dev profile [unoptimized + debuginfo] target(s) in 1.01s
```

**Step 2: Build the plugin bundle**

```bash
cd xtask
cargo run --release -- bundle naughty-and-tender --release
```

This will:
1. Compile naughty-and-tender in release mode (~1 minute)
2. Create bundles in `target/bundled/`:
   - `naughty-and-tender.vst3`
   - `naughty-and-tender.clap`

You should see:
```
Created a CLAP bundle at 'target\bundled\naughty-and-tender.clap'
Created a VST3 bundle at 'target\bundled\naughty-and-tender.vst3'
```

🎉 **You just built an audio plugin in Rust!**

---

## Testing in Reaper

Let's load the plugin and verify it works.

### Step 1: Install the Plugin

**Windows:**
```bash
copy target\bundled\naughty-and-tender.vst3 "C:\Program Files\Common Files\VST3\"
```

**Linux:**
```bash
cp -r target/bundled/naughty-and-tender.vst3 ~/.vst3/
```

**macOS:**
```bash
cp -r target/bundled/naughty-and-tender.vst3 ~/Library/Audio/Plug-Ins/VST3/
```

### Step 2: Load in Reaper

1. **Open Reaper**
2. **Rescan plugins**: Options → Preferences → Plug-ins → VST → Re-scan
3. **Create a MIDI track**: Track → Insert new track
4. **Add the plugin**: Track → Insert virtual instrument on new track
5. **Search "Naughty"** and add it

**You should see:**
- The plugin GUI appears
- Status shows "✅ Plugin loaded successfully"
- No error messages in Reaper's console

### Step 3: Test MIDI Input

1. **Enable MIDI recording** on the track
2. **Add a MIDI item** or play your MIDI keyboard
3. **Check for logs**:
   - **Windows**: Download [DebugView](https://learn.microsoft.com/en-us/sysinternals/downloads/debugview)
   - Run DebugView, play MIDI notes
   - You should see: `Note On: 60 velocity: 0.8`, `Note Off: 60`

**Current behavior:** The plugin receives MIDI but outputs silence (expected in Phase 1).

### Step 4: Test Parameters

1. **Move the Gain slider** in the plugin GUI
2. **Right-click the track's volume fader** → Show track envelope
3. **Add an automation point** while moving the Gain slider
4. **Play the project** - the slider should move with your automation

✅ **Automation works!** nih-plug handles all the complexity for you.

---

## Architectural Decisions: Why We Built It This Way

Let's reflect on our design choices.

### Decision 1: Workspace Structure

**Why separate `shared/core/`?**

- **Reusability**: MIDI-to-frequency conversion is needed in every synthesizer
- **Testing**: Shared code has its own tests, separate from plugin logic
- **Future-proofing**: When we build plugin #2, it can reuse this code

**Alternative:** Put everything in the plugin package. Works for one plugin, but doesn't scale.

### Decision 2: Arc for Parameters

**Why `Arc<NaughtyAndTenderParams>`?**

Parameters are accessed from:
1. **Audio thread** (reading values during `process()`)
2. **GUI thread** (updating sliders)
3. **Automation thread** (recording parameter changes)

`Arc` provides thread-safe shared ownership. Each thread has a reference-counted pointer.

> **💡 Tooltip: Why Not Just Pass References?**
>
> References (`&T`) are tied to lifetimes - they can't outlive the owner. The audio thread runs for the plugin's lifetime, but we create it in `initialize()`. `Arc` lets us share ownership without lifetime complexity.

### Decision 3: Silence in Phase 1

**Why not implement oscillators immediately?**

Audio plugin development has many layers:
1. **Project setup** (Cargo, workspace)
2. **Plugin shell** (trait implementation, callbacks)
3. **DSP** (oscillators, filters)
4. **Voice management** (polyphony)
5. **Modulation** (envelopes, LFOs)

By separating concerns, we:
- **Verify each layer works** before adding complexity
- **Debug more easily** (fewer moving parts)
- **Learn progressively** (master fundamentals first)

Phase 1 proves: Plugin loads, MIDI works, parameters automate, GUI renders. That's a solid foundation.

### Decision 4: egui for GUI

**Why egui instead of vizia or custom OpenGL?**

| Framework | Pros | Cons |
|-----------|------|------|
| **egui** | Simple, immediate-mode, minimal boilerplate | Less "native" look |
| **vizia** | Audio-focused, reactive | Steeper learning curve |
| **Custom OpenGL** | Total control | Huge time investment |

For a learning project, egui's simplicity wins. We can always migrate later.

### Decision 5: Both VST3 and CLAP

**Why support two formats?**

- **Cost:** Zero (nih-plug does the work)
- **Compatibility:** VST3 has wider DAW support (Reaper, Cubase, Ableton, etc.)
- **Future-proofing:** CLAP is growing (Bitwig, Reaper support it)

No reason not to export both!

---

## Troubleshooting

### Build Errors

**Error: "cannot find type `EguiState`"**

**Solution:** Add import to `params.rs`:
```rust
use nih_plug_egui::EguiState;
```

This happened to us during development - we forgot to import egui types!

**Error: "unused variable: `output`"**

**Solution:** Remove `mut` from:
```rust
let output = buffer.as_slice();  // Not: let mut output = ...
```

We don't mutate the slice variable itself, just the data it points to.

### Runtime Issues

**Plugin doesn't appear in Reaper**

1. **Verify bundle exists:** Check `target/bundled/` has `.vst3` file
2. **Correct installation path:** Windows: `C:\Program Files\Common Files\VST3\`
3. **Rescan plugins:** Reaper → Preferences → Plug-ins → Re-scan
4. **Check Reaper's log:** View → Monitoring FX/Track output

**GUI doesn't open**

1. **Check build output:** Should see "Finished `release` profile"
2. **Rebuild with debug symbols:**
   ```bash
   cargo run --release -- bundle naughty-and-tender
   ```
3. **Look for errors in DebugView** (Windows) or terminal

**No MIDI events logged**

1. **Enable MIDI track:** Track must be record-armed
2. **MIDI source configured:** Options → Preferences → MIDI Devices
3. **DebugView running** (Windows) to see `nih_log!` output

---

## What's Next?

We now have a fully functional plugin shell:
- ✅ Loads in Reaper
- ✅ Processes MIDI events
- ✅ Renders GUI
- ✅ Supports parameter automation
- ✅ Real-time safe audio callback

**In Article 3** (Phase 2), we'll add:
- Oscillators (sine, saw, square, triangle)
- Voice management (polyphony)
- Basic ADSR envelope
- Actual sound output!

The hard foundational work is done. From here, it's DSP and creativity.

---

## Key Takeaways

**VST3/CLAP Architecture:**
- Plugins are shared libraries with exported C functions
- DAWs call `initialize()`, `process()`, `reset()`, etc.
- Real-time safety is critical in `process()`

**nih-plug Design:**
- `Plugin` trait encapsulates all plugin logic
- `Params` trait handles automation and persistence
- `Editor` trait provides GUI integration
- `nih_export_*!` macros generate plugin exports

**Real-Time Safety Rules:**
- ❌ No allocations in `process()`
- ❌ No locks that might block
- ❌ No I/O or system calls
- ✅ Use pre-allocated buffers
- ✅ Keep computation deterministic

**Rust Advantages:**
- Memory safety prevents use-after-free bugs
- Ownership prevents data races at compile-time
- `Arc` provides thread-safe shared state
- `cargo` and `clippy` catch errors early

---

## Quick Reference

**Project Structure:**

```
naughty-and-tender/
├── Cargo.toml          # Package manifest
└── src/
    ├── lib.rs          # Plugin trait implementation
    ├── params.rs       # Parameter definitions
    └── editor.rs       # GUI code
```

**Build Commands:**

```bash
# Check compilation
cargo check --package naughty-and-tender

# Build plugin bundles
cd xtask
cargo run --release -- bundle naughty-and-tender --release

# Run tests
cargo test --package naughty-and-tender
```

**Key Files to Reference:**

- `lib.rs`: Main plugin logic, `Plugin` trait
- `params.rs`: `Params` derive, parameter ranges
- `editor.rs`: egui GUI creation

---

## Further Reading

**nih-plug Specifics:**
- [nih-plug API docs](https://nih-plug.robbertvanderhelm.nl/)
- [Example plugins](https://github.com/robbert-vdh/nih-plug/tree/master/plugins)
- [nih-plug Discord](https://discord.gg/tNX2XZXF)

**Plugin Development:**
- [VST3 SDK Documentation](https://steinbergmedia.github.io/vst3_doc/)
- [CLAP Specification](https://github.com/free-audio/clap)
- [Real-Time Audio Programming 101](http://www.rossbencina.com/code/real-time-audio-programming-101-time-waits-for-nothing)

**Rust for Audio:**
- [Rust Audio Discourse](https://rust-audio.discourse.group/)
- [RustConf: Writing Audio Software in Rust](https://www.youtube.com/watch?v=Yom9E-67bdI)
- [dasp - Digital Audio Signal Processing](https://github.com/RustAudio/dasp)

**GUI with egui:**
- [egui Documentation](https://docs.rs/egui/)
- [egui Demo App](https://www.egui.rs/#demo)
- [Immediate Mode GUIs Explained](https://www.youtube.com/watch?v=Z1qyvQsjK5Y)

---

**Next Article:** Building Audio Plugins in Rust: Oscillators and Synthesis (Phase 2)

**Previous Article:** [Building Audio Plugins in Rust: Project Setup](./01-project-setup-and-why-rust.md)

---

*Written as part of the Naughty and Tender development series - a journey building a MIDI synthesizer plugin in Rust. Follow along as we explore DSP, real-time programming, and the Rust audio ecosystem.*
