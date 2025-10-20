# Building Audio Plugins in Rust: ADSR Envelopes and Polyphonic Voice Management

**Part 4 of the Naughty and Tender Development Series**

---

## What You'll Learn

By the end of this article, you'll understand:

- ✅ How ADSR envelopes shape amplitude over time
- ✅ Implementing a state machine for envelope phases
- ✅ Building polyphonic voice management with 16-voice pools
- ✅ Voice stealing algorithms for handling polyphony limits
- ✅ Sample-accurate mixing of multiple voices
- ✅ Real-time memory management (pre-allocation)
- ✅ Integrating oscillators, envelopes, and MIDI events

**Prerequisites:**
- [x] Completed [Article 3: Oscillators](./03-oscillators-and-waveform-generation.md)
- [x] Understanding of oscillators and waveform generation
- [x] Familiarity with MIDI note on/off events
- [x] Basic understanding of musical envelope shapes

**Time Estimate:** 75-90 minutes

---

## Introduction: From Continuous Tones to Musical Notes

Our oscillators from Article 3 generate continuous waveforms - turn them on and they play forever. But musical instruments have **natural dynamics**:

- **Piano**: Loud attack when hammer strikes, gradual decay to silence
- **Violin**: Gradual swell when bow engages, sustain while bowing, fade when bow lifts
- **Trumpet**: Quick attack when air starts, sustain while blowing, quick decay when stopped

We need **envelopes** to sculpt amplitude over time, transforming continuous oscillations into expressive notes.

### What Is an ADSR Envelope?

ADSR is the classic envelope generator from analog synthesizers:

```
Amplitude
    ^
    |
    |        Attack    Decay
    |       /\        /
    |      /  \      /  Sustain
    |     /    \____/___________
    |    /                      \  Release
    |   /                        \
    +--+----------------------------+-----> Time
       ^                         ^
    Note On                  Note Off
```

**Four Phases**:

1. **Attack**: Rise from 0 to peak (press key)
2. **Decay**: Fall from peak to sustain level
3. **Sustain**: Hold at constant level (key held)
4. **Release**: Fall from current level to 0 (release key)

> **💡 Tooltip: Why ADSR?**
>
> ADSR was developed in the 1960s for analog synthesizers (Moog, ARP). It remains standard because it models natural instrument behavior and provides intuitive control. Alternatives include AHDSR (adds Hold), AR (simpler), and multi-stage envelopes. [Learn more](https://en.wikipedia.org/wiki/Envelope_(music))

> **📚 Further Reading:**
> - [Sound On Sound - Synth Secrets: ADSR](https://www.soundonsound.com/techniques/synthesizer-envelopes) - Deep dive into envelope design
> - [The Art of VA Filter Design](https://www.native-instruments.com/fileadmin/ni_media/downloads/pdf/VAFilterDesign_2.1.0.pdf) - Chapter on envelopes (page 45)
> - [Designing Sound by Andy Farnell](https://mitpress.mit.edu/books/designing-sound) - Chapter 12: Time and Envelopes

---

## The Envelope State Machine

Envelopes are perfect for **state machines** - each phase has distinct behavior and transition rules.

### State Diagram

```
        ┌──────┐
        │ Idle │ ←──────────────────┐
        └──┬───┘                     │
           │ note_on()               │
           ↓                         │
        ┌────────┐                   │
        │ Attack │ (ramp 0→1)        │
        └───┬────┘                   │
            │ peak reached           │
            ↓                        │
        ┌───────┐                    │
        │ Decay │ (ramp 1→sustain)   │
        └───┬───┘                    │
            │ sustain reached        │
            ↓                        │
        ┌─────────┐                  │
        │ Sustain │ (hold level)     │
        └────┬────┘                  │
             │ note_off()            │
             ↓                       │
        ┌─────────┐                  │
        │ Release │ (ramp→0)         │
        └─────┬───┘                  │
              │ zero reached         │
              └──────────────────────┘
```

**Transitions**:
- `note_on()` always goes to **Attack** (even from Release - retriggering)
- **Attack** → **Decay** when peak is reached
- **Decay** → **Sustain** when sustain level is reached
- **Sustain** stays forever until `note_off()`
- `note_off()` always goes to **Release** (from any active state)
- **Release** → **Idle** when amplitude reaches zero

### Edge Cases

**Instant phases**: If attack time is 0ms, we jump immediately from Idle to Attack to Decay in one sample. We need to **fall through** states in a single `process()` call.

**Release from any phase**: User might release the key during Attack or Decay. We must smoothly transition to Release from the current amplitude (not from sustain level).

---

## Implementing the Envelope Module

Create `naughty-and-tender/src/envelope.rs`:

### State Enum

```rust
/// Envelope state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}
```

This enum represents which phase the envelope is currently in.

### The ADSREnvelope Struct

```rust
/// ADSR Envelope generator
///
/// Generates amplitude envelopes with Attack, Decay, Sustain, and Release phases.
/// Uses linear ramps for all phases and maintains sample-accurate timing.
///
/// # Real-time Safety
/// - No allocations in `process()`
/// - All state pre-initialized
/// - Simple state machine with no branches in hot path
pub struct ADSREnvelope {
    /// Current envelope state
    state: EnvelopeState,

    /// Current envelope output value (0.0 to 1.0)
    current_value: f32,

    /// Sample rate in Hz
    sample_rate: f32,

    /// Attack time in samples
    attack_samples: f32,

    /// Decay time in samples
    decay_samples: f32,

    /// Sustain level (0.0 to 1.0)
    sustain_level: f32,

    /// Release time in samples
    release_samples: f32,

    /// Current sample position in current phase
    phase_sample: f32,

    /// Velocity scaling (0.0 to 1.0)
    velocity: f32,

    /// Value at start of release (for release from any level)
    release_start_value: f32,
}
```

**Code Analysis:**

```rust
/// Attack time in samples
attack_samples: f32,
```

> **Why store times in samples?**: Converting milliseconds to samples every process call would waste CPU. We convert once when the parameter changes and cache the sample count. At 44.1kHz, 100ms = 4410 samples.

```rust
/// Velocity scaling (0.0 to 1.0)
velocity: f32,
```

> **Velocity sensitivity**: MIDI velocity (how hard you press a key) should affect loudness. We scale the envelope peak by velocity. MIDI velocity is 0-127, normalized to 0.0-1.0.

```rust
/// Value at start of release (for release from any level)
release_start_value: f32,
```

> **Release from anywhere**: If you release during Attack (amplitude = 0.3), Release should ramp from 0.3 to 0, not from sustain level. We capture the current value when entering Release.

### Constructor and Parameter Setters

```rust
impl ADSREnvelope {
    /// Create a new ADSR envelope
    ///
    /// # Arguments
    /// * `sample_rate` - Sample rate in Hz
    ///
    /// # Default Settings
    /// - Attack: 10ms
    /// - Decay: 100ms
    /// - Sustain: 70%
    /// - Release: 100ms
    #[must_use]
    pub fn new(sample_rate: f32) -> Self {
        let mut env = Self {
            state: EnvelopeState::Idle,
            current_value: 0.0,
            sample_rate,
            attack_samples: 0.0,
            decay_samples: 0.0,
            sustain_level: 0.7,
            release_samples: 0.0,
            phase_sample: 0.0,
            velocity: 1.0,
            release_start_value: 0.0,
        };

        // Set default envelope times
        env.set_attack_ms(10.0);
        env.set_decay_ms(100.0);
        env.set_release_ms(100.0);

        env
    }

    /// Set attack time in milliseconds
    pub fn set_attack_ms(&mut self, attack_ms: f32) {
        self.attack_samples = (attack_ms / 1000.0) * self.sample_rate;
    }

    /// Set decay time in milliseconds
    pub fn set_decay_ms(&mut self, decay_ms: f32) {
        self.decay_samples = (decay_ms / 1000.0) * self.sample_rate;
    }

    /// Set sustain level (0.0 to 1.0)
    pub fn set_sustain_level(&mut self, sustain_level: f32) {
        self.sustain_level = sustain_level.clamp(0.0, 1.0);
    }

    /// Set release time in milliseconds
    pub fn set_release_ms(&mut self, release_ms: f32) {
        self.release_samples = (release_ms / 1000.0) * self.sample_rate;
    }
}
```

**Code Analysis:**

```rust
self.attack_samples = (attack_ms / 1000.0) * self.sample_rate;
```

> **Milliseconds to samples**:
> ```
> samples = (milliseconds / 1000) * sample_rate
>
> Example: 50ms at 44100 Hz
> samples = (50 / 1000) * 44100 = 2205 samples
> ```

```rust
self.sustain_level = sustain_level.clamp(0.0, 1.0);
```

> **Clamping**: Ensures value stays in valid range. If a user (or automation) passes 1.5, we clamp to 1.0. Prevents out-of-range amplitudes. [clamp documentation](https://doc.rust-lang.org/std/primitive.f32.html#method.clamp)

### Note On and Note Off

```rust
/// Trigger note on - start attack phase
///
/// # Arguments
/// * `velocity` - Note velocity (0.0 to 1.0)
pub fn note_on(&mut self, velocity: f32) {
    self.velocity = velocity.clamp(0.0, 1.0);
    self.state = EnvelopeState::Attack;
    self.phase_sample = 0.0;
    self.current_value = 0.0;
}

/// Trigger note off - start release phase
pub fn note_off(&mut self) {
    self.state = EnvelopeState::Release;
    self.phase_sample = 0.0;
    self.release_start_value = self.current_value;
}
```

**Code Analysis:**

```rust
self.current_value = 0.0;
```

> **Why reset to 0 on note_on?**: Even if retriggering (note_on during Attack), we start fresh. This creates a **hard restart** - you hear the attack again. Alternative: **legato** mode would continue from current value. We keep it simple.

```rust
self.release_start_value = self.current_value;
```

> **Smooth release**: Capturing the current value ensures Release doesn't cause a discontinuity (click). If we're at amplitude 0.4 when releasing, we ramp from 0.4 → 0.0, not from sustain_level → 0.0.

---

## The Process Method: State Machine Logic

This is the most complex function - it processes one sample and advances the state machine.

### The Loop Structure

We use a `loop` that can `continue` to handle instant phases:

```rust
/// Process one sample and return envelope value
///
/// Returns amplitude multiplier (0.0 to 1.0) scaled by velocity.
///
/// # Returns
/// Current envelope amplitude (0.0 to 1.0)
#[inline]
pub fn process(&mut self) -> f32 {
    // Process envelope state machine
    // Handle instant phases by falling through to next state
    loop {
        match self.state {
            EnvelopeState::Idle => {
                self.current_value = 0.0;
                break;
            }

            EnvelopeState::Attack => {
                // Attack logic (see below)
            }

            EnvelopeState::Decay => {
                // Decay logic (see below)
            }

            EnvelopeState::Sustain => {
                // Sustain logic (see below)
            }

            EnvelopeState::Release => {
                // Release logic (see below)
            }
        }
    }

    self.current_value
}
```

**Code Analysis:**

```rust
loop {
    match self.state {
        // ...
        if instant_phase {
            self.transition_to_next();
            continue; // Process next state in same call
        } else {
            break; // Done for this sample
        }
    }
}
```

> **Why a loop?**: If attack_samples is 0.0 (instant attack), we transition to Decay **in the same sample**. The `continue` jumps back to the top of the loop, processing Decay. This handles **multiple instant phases** correctly (instant attack + instant decay = jump straight to sustain).

### Attack Phase

```rust
EnvelopeState::Attack => {
    if self.attack_samples <= 0.0 {
        // Instant attack - fall through to decay
        self.current_value = self.velocity;
        self.transition_to_decay();
        continue; // Process decay in same call
    } else {
        // Linear ramp from 0 to velocity
        let progress = self.phase_sample / self.attack_samples;
        self.current_value = progress * self.velocity;

        self.phase_sample += 1.0;

        if self.phase_sample >= self.attack_samples {
            self.current_value = self.velocity;
            self.transition_to_decay();
        }
        break;
    }
}
```

**Code Analysis:**

```rust
let progress = self.phase_sample / self.attack_samples;
```

> **Linear interpolation**: `progress` goes from 0.0 to 1.0 over the attack duration:
> ```
> sample 0:    progress = 0 / 4410 = 0.0
> sample 2205: progress = 2205 / 4410 = 0.5
> sample 4410: progress = 4410 / 4410 = 1.0
> ```
> Then `current_value = progress * velocity` scales it.

```rust
if self.phase_sample >= self.attack_samples {
    self.current_value = self.velocity;
    self.transition_to_decay();
}
```

> **Exact peak value**: We explicitly set `current_value = velocity` at the end to avoid floating-point error. If we relied on the ramp calculation, we might get 0.999999 instead of 1.0.

### Decay Phase

```rust
EnvelopeState::Decay => {
    if self.decay_samples <= 0.0 {
        // Instant decay - fall through to sustain
        self.current_value = self.sustain_level * self.velocity;
        self.transition_to_sustain();
        break; // Sustain doesn't need processing, so we can stop
    } else {
        // Linear ramp from velocity to sustain_level * velocity
        let progress = self.phase_sample / self.decay_samples;
        let target = self.sustain_level * self.velocity;
        self.current_value = self.velocity + (target - self.velocity) * progress;

        self.phase_sample += 1.0;

        if self.phase_sample >= self.decay_samples {
            self.current_value = target;
            self.transition_to_sustain();
        }
        break;
    }
}
```

**Code Analysis:**

```rust
let target = self.sustain_level * self.velocity;
self.current_value = self.velocity + (target - self.velocity) * progress;
```

> **Lerp formula**: This is linear interpolation (lerp) from `velocity` to `target`:
> ```
> lerp(start, end, t) = start + (end - start) * t
>
> Example: Decay from 1.0 to 0.7
> progress = 0.0: value = 1.0 + (0.7 - 1.0) * 0.0 = 1.0
> progress = 0.5: value = 1.0 + (0.7 - 1.0) * 0.5 = 0.85
> progress = 1.0: value = 1.0 + (0.7 - 1.0) * 1.0 = 0.7
> ```

```rust
break; // Sustain doesn't need processing, so we can stop
```

> **Why break here?**: Sustain holds constant - no need to continue the loop. Instant decay sets the value and we're done for this sample.

### Sustain Phase

```rust
EnvelopeState::Sustain => {
    // Hold at sustain level
    self.current_value = self.sustain_level * self.velocity;
    break;
}
```

Simple! Sustain just holds the value. It stays here until `note_off()` is called.

### Release Phase

```rust
EnvelopeState::Release => {
    if self.release_samples <= 0.0 {
        // Instant release
        self.current_value = 0.0;
        self.transition_to_idle();
    } else {
        // Linear ramp from release_start_value to 0
        let progress = self.phase_sample / self.release_samples;
        self.current_value = self.release_start_value * (1.0 - progress);

        self.phase_sample += 1.0;

        if self.phase_sample >= self.release_samples {
            self.current_value = 0.0;
            self.transition_to_idle();
        }
    }
    break;
}
```

**Code Analysis:**

```rust
self.current_value = self.release_start_value * (1.0 - progress);
```

> **Decay to zero**: As `progress` goes 0.0 → 1.0, `(1.0 - progress)` goes 1.0 → 0.0:
> ```
> progress = 0.0: value = start_value * 1.0 = start_value
> progress = 0.5: value = start_value * 0.5 = half
> progress = 1.0: value = start_value * 0.0 = 0.0
> ```

### Utility Methods

```rust
/// Check if envelope is active (not idle)
#[must_use]
pub fn is_active(&self) -> bool {
    self.state != EnvelopeState::Idle
}

/// Get current envelope state
#[must_use]
pub fn get_state(&self) -> EnvelopeState {
    self.state
}

/// Reset envelope to idle state
pub fn reset(&mut self) {
    self.state = EnvelopeState::Idle;
    self.current_value = 0.0;
    self.phase_sample = 0.0;
}
```

`is_active()` is critical for voice management - we'll use it to know when voices are silent and can be reused.

---

## Voice Management: Polyphony

Now we combine oscillators and envelopes into **voices** - independent note generators.

### The Voice Struct

Create `naughty-and-tender/src/voice.rs`:

```rust
use crate::envelope::ADSREnvelope;
use crate::oscillators::{Oscillator, WaveformType};

/// Voice state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoiceState {
    Idle,
    Active,
    Releasing,
}

/// Single synthesizer voice
///
/// Each voice contains an oscillator and envelope, and tracks a MIDI note number.
///
/// # Real-time Safety
/// - All components pre-allocated
/// - No allocations in `process()`
pub struct Voice {
    /// Oscillator for generating waveforms
    oscillator: Oscillator,

    /// ADSR envelope for amplitude control
    envelope: ADSREnvelope,

    /// MIDI note number (0-127)
    note: u8,

    /// Current voice state
    state: VoiceState,

    /// Current waveform type
    waveform: WaveformType,

    /// Voice age (for voice stealing)
    age: u64,
}
```

**Code Analysis:**

```rust
/// Voice age (for voice stealing)
age: u64,
```

> **Age tracking**: When all voices are busy and we get a new note_on, we **steal** the oldest voice. We increment a global counter each note_on and assign it to the voice. The voice with the lowest age is oldest.

### Voice Implementation

```rust
impl Voice {
    /// Create a new voice
    #[must_use]
    pub fn new(sample_rate: f32) -> Self {
        Self {
            oscillator: Oscillator::new(sample_rate),
            envelope: ADSREnvelope::new(sample_rate),
            note: 0,
            state: VoiceState::Idle,
            waveform: WaveformType::Sine,
            age: 0,
        }
    }

    /// Trigger note on
    pub fn note_on(&mut self, note: u8, velocity: f32) {
        self.note = note;
        self.state = VoiceState::Active;
        self.envelope.note_on(velocity);
        self.oscillator.reset();
    }

    /// Trigger note off
    pub fn note_off(&mut self) {
        self.state = VoiceState::Releasing;
        self.envelope.note_off();
    }

    /// Process one sample
    ///
    /// Returns the output sample (audio * envelope).
    #[inline]
    pub fn process(&mut self) -> f32 {
        // Check if envelope completed release
        if !self.envelope.is_active() {
            self.state = VoiceState::Idle;
            return 0.0;
        }

        // Get frequency from MIDI note
        let frequency = midi_note_to_frequency(self.note);

        // Generate waveform
        let audio = match self.waveform {
            WaveformType::Sine => self.oscillator.process_sine(frequency),
            WaveformType::Sawtooth => self.oscillator.process_sawtooth(frequency),
            WaveformType::Square => self.oscillator.process_square(frequency),
            WaveformType::Triangle => self.oscillator.process_triangle(frequency),
        };

        // Apply envelope
        let envelope_value = self.envelope.process();

        audio * envelope_value
    }
}
```

**Code Analysis:**

```rust
if !self.envelope.is_active() {
    self.state = VoiceState::Idle;
    return 0.0;
}
```

> **Automatic voice deactivation**: When the envelope finishes Release, `is_active()` returns false. We set voice state to Idle and return silence. This voice is now available for reuse.

```rust
audio * envelope_value
```

> **Amplitude modulation**: The envelope (0.0 to 1.0) multiplies the oscillator output (-1.0 to 1.0). When envelope = 0.5, we get 50% amplitude. This is the fundamental synthesis technique: `output = oscillator * envelope`.

### MIDI Note to Frequency

```rust
/// Convert MIDI note number to frequency in Hz
///
/// Uses standard MIDI tuning: A4 (note 69) = 440 Hz
#[inline]
#[must_use]
pub fn midi_note_to_frequency(note: u8) -> f32 {
    440.0 * 2.0f32.powf((f32::from(note) - 69.0) / 12.0)
}
```

This is the same formula from Article 3, now used in the voice context.

---

## Voice Manager: Polyphony Pool

The `VoiceManager` maintains a **pre-allocated pool** of voices and assigns them to incoming notes.

### The VoiceManager Struct

```rust
/// Voice manager for polyphonic synthesis
///
/// Manages a fixed-size pool of voices with voice stealing when limit is reached.
///
/// # Real-time Safety
/// - Voices pre-allocated at construction
/// - No dynamic allocation in `note_on/note_off/process`
pub struct VoiceManager {
    /// Pre-allocated voice pool
    voices: Vec<Voice>,

    /// Maximum polyphony
    max_voices: usize,

    /// Global voice age counter
    voice_age_counter: u64,

    /// Sample rate
    sample_rate: f32,
}
```

**Code Analysis:**

```rust
voices: Vec<Voice>,
```

> **Vec in real-time code?**: `Vec` can allocate, which is forbidden in real-time. But here, we **pre-allocate** in the constructor and never grow the Vec. We use `Vec::with_capacity()` to reserve space upfront, then never push/pop. This is safe.

### Constructor

```rust
impl VoiceManager {
    /// Create a new voice manager
    ///
    /// # Arguments
    /// * `sample_rate` - Sample rate in Hz
    /// * `max_voices` - Maximum number of simultaneous voices
    #[must_use]
    pub fn new(sample_rate: f32, max_voices: usize) -> Self {
        let mut voices = Vec::with_capacity(max_voices);
        for _ in 0..max_voices {
            voices.push(Voice::new(sample_rate));
        }

        Self {
            voices,
            max_voices,
            voice_age_counter: 0,
            sample_rate,
        }
    }
}
```

**Code Analysis:**

```rust
let mut voices = Vec::with_capacity(max_voices);
for _ in 0..max_voices {
    voices.push(Voice::new(sample_rate));
}
```

> **Pre-allocation**: We allocate all voices upfront. At 16 voices, each voice is ~200 bytes (oscillator + envelope state), so ~3.2KB total. This happens once during initialization, not in the audio callback.

### Note On: Voice Allocation

```rust
/// Trigger note on
///
/// Allocates a voice, or steals one if all voices are in use.
///
/// # Arguments
/// * `note` - MIDI note number (0-127)
/// * `velocity` - Note velocity (0.0-1.0)
pub fn note_on(&mut self, note: u8, velocity: f32) {
    // First, check if this note is already playing and reuse it (retrigger)
    for voice in &mut self.voices {
        if voice.get_note() == note && voice.get_state() != VoiceState::Idle {
            voice.note_on(note, velocity);
            voice.set_age(self.voice_age_counter);
            self.voice_age_counter += 1;
            return;
        }
    }

    // Find an idle voice
    for voice in &mut self.voices {
        if voice.get_state() == VoiceState::Idle {
            voice.note_on(note, velocity);
            voice.set_age(self.voice_age_counter);
            self.voice_age_counter += 1;
            return;
        }
    }

    // No idle voice found - steal one
    self.steal_voice(note, velocity);
}
```

**Code Analysis:**

**Step 1: Retrigger check**
```rust
if voice.get_note() == note && voice.get_state() != VoiceState::Idle {
    voice.note_on(note, velocity);
    return;
}
```

> **Retriggering**: If you press C4, release it (enters Release), then press C4 again before Release finishes, we **reuse the same voice**. This prevents two voices playing the same note (which would double the volume unnaturally).

**Step 2: Find idle voice**
```rust
if voice.get_state() == VoiceState::Idle {
    voice.note_on(note, velocity);
    return;
}
```

> **Fast path**: Most of the time, we have idle voices. Linear search through 16 voices is fast (~16 comparisons).

**Step 3: Voice stealing**
```rust
self.steal_voice(note, velocity);
```

> **Polyphony limit**: If you play a 17-note chord with 16 voices, we must steal a voice. Which one? See below.

### Voice Stealing Algorithm

```rust
/// Steal a voice
///
/// Strategy:
/// 1. Prefer releasing voices over active voices
/// 2. Among releasing voices, steal oldest
/// 3. Among active voices, steal oldest
fn steal_voice(&mut self, note: u8, velocity: f32) {
    // Find releasing voice with oldest age
    let mut oldest_releasing: Option<usize> = None;
    let mut oldest_releasing_age = u64::MAX;

    for (i, voice) in self.voices.iter().enumerate() {
        if voice.get_state() == VoiceState::Releasing
            && (oldest_releasing.is_none() || voice.get_age() < oldest_releasing_age)
        {
            oldest_releasing = Some(i);
            oldest_releasing_age = voice.get_age();
        }
    }

    // If we found a releasing voice, steal it
    if let Some(index) = oldest_releasing {
        self.voices[index].note_on(note, velocity);
        self.voices[index].set_age(self.voice_age_counter);
        self.voice_age_counter += 1;
        return;
    }

    // No releasing voice - find oldest active voice
    let mut oldest_active_index = 0;
    let mut oldest_active_age = self.voices[0].get_age();

    for (i, voice) in self.voices.iter().enumerate() {
        if voice.get_age() < oldest_active_age {
            oldest_active_index = i;
            oldest_active_age = voice.get_age();
        }
    }

    // Steal oldest active voice
    self.voices[oldest_active_index].note_on(note, velocity);
    self.voices[oldest_active_index].set_age(self.voice_age_counter);
    self.voice_age_counter += 1;
}
```

**Code Analysis:**

**Priority 1: Steal releasing voices**
```rust
if voice.get_state() == VoiceState::Releasing { ... }
```

> **Why prefer releasing voices?**: A releasing voice is fading out - stealing it is less audible than cutting an active sustaining note. Better to truncate a tail than chop a held chord.

**Priority 2: Steal oldest active**
```rust
if voice.get_age() < oldest_active_age {
    oldest_active_index = i;
}
```

> **Oldest = lowest age**: If ages are [5, 12, 8, 3], the voice with age 3 is oldest (first played). This is a simple LRU (Least Recently Used) strategy. Alternatives: steal quietest, steal lowest frequency, round-robin.

### Note Off

```rust
/// Trigger note off
///
/// # Arguments
/// * `note` - MIDI note number to release
pub fn note_off(&mut self, note: u8) {
    for voice in &mut self.voices {
        if voice.get_note() == note && voice.get_state() == VoiceState::Active {
            voice.note_off();
        }
    }
}
```

Simple: find all voices playing this note and trigger their release phase.

---

## Critical Realization: Sample-by-Sample Mixing

This was a **key bug fix** during development. Let's understand the mistake and solution.

### The Wrong Approach (Voice-by-Voice)

```rust
// ❌ WRONG: Process each voice completely, then next voice
pub fn process_wrong(&mut self, buffer: &mut [f32]) {
    buffer.fill(0.0);

    for voice in &mut self.voices {
        if voice.get_state() != VoiceState::Idle {
            // Process entire buffer for this voice
            for sample in buffer.iter_mut() {
                *sample += voice.process();
            }
        }
    }
}
```

**Problem**: Voice 1 processes samples [0..127], then Voice 2 processes samples [0..127]. But envelopes are **stateful** - each `process()` call advances the envelope. Voice 1's envelope advances 128 samples, then Voice 2's envelope **also** advances 128 samples. They're out of sync!

### The Right Approach (Sample-by-Sample)

```rust
// ✅ CORRECT: Process sample-by-sample
pub fn process(&mut self, buffer: &mut [f32]) {
    // Clear buffer
    buffer.fill(0.0);

    // Mix all voices - process sample-by-sample for sample-accurate mixing
    // Each sample contains contributions from all voices at that exact time point
    for sample in buffer.iter_mut() {
        for voice in &mut self.voices {
            if voice.get_state() != VoiceState::Idle {
                *sample += voice.process();
            }
        }
    }
}
```

**Why this works**: For each sample index, we call `process()` on **all active voices** before moving to the next sample. All envelopes advance together, staying synchronized.

**Visualization**:

```
Sample 0: Voice1.process() + Voice2.process() + Voice3.process()
Sample 1: Voice1.process() + Voice2.process() + Voice3.process()
Sample 2: Voice1.process() + Voice2.process() + Voice3.process()
...
```

All voices experience the same timeline.

**Code Analysis:**

```rust
for sample in buffer.iter_mut() {
    for voice in &mut self.voices {
        *sample += voice.process();
    }
}
```

> **Nested loops**: Outer loop iterates samples (typically 64-512). Inner loop iterates voices (up to 16). Total: 64 * 16 = 1024 calls per buffer. At 48kHz with 64 samples: ~750,000 voice.process() calls per second. This must be fast!

---

## Integration with the Plugin

Update `naughty-and-tender/src/lib.rs`:

### Add Voice Manager Field

```rust
pub struct NaughtyAndTender {
    params: Arc<NaughtyAndTenderParams>,
    sample_rate: f32,
    voice_manager: Option<VoiceManager>,  // Added
}
```

### Initialize in `initialize()`

```rust
fn initialize(
    &mut self,
    _audio_io_layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>,
) -> bool {
    const NUM_VOICES: usize = 16;

    self.sample_rate = buffer_config.sample_rate;
    self.voice_manager = Some(VoiceManager::new(self.sample_rate, NUM_VOICES));

    true
}
```

### Process Audio and MIDI

```rust
fn process(
    &mut self,
    buffer: &mut Buffer,
    _aux: &mut AuxiliaryBuffers,
    context: &mut impl ProcessContext<Self>,
) -> ProcessStatus {
    let Some(voice_manager) = &mut self.voice_manager else {
        // Not initialized - output silence
        for channel_samples in buffer.as_slice() {
            channel_samples.fill(0.0);
        }
        return ProcessStatus::Normal;
    };

    // Get parameters
    let gain = self.params.gain.value();
    let waveform_int = self.params.waveform.value();
    let attack_ms = self.params.attack_ms.value();
    let decay_ms = self.params.decay_ms.value();
    let sustain_level = self.params.sustain_level.value();
    let release_ms = self.params.release_ms.value();

    // Convert waveform int to enum
    let waveform = match waveform_int {
        0 => WaveformType::Sine,
        1 => WaveformType::Sawtooth,
        2 => WaveformType::Square,
        3 => WaveformType::Triangle,
        _ => WaveformType::Sine,
    };

    // Update voice manager with current parameters
    voice_manager.set_waveform(waveform);
    voice_manager.set_attack_ms(attack_ms);
    voice_manager.set_decay_ms(decay_ms);
    voice_manager.set_sustain_level(sustain_level);
    voice_manager.set_release_ms(release_ms);

    // Process MIDI events
    let mut next_event = context.next_event();
    let num_samples = buffer.samples();

    // Process sample by sample (for sample-accurate MIDI)
    for sample_idx in 0..num_samples {
        // Handle MIDI events at this sample
        while let Some(event) = next_event {
            if event.timing() > sample_idx as u32 {
                break;
            }

            match event {
                NoteEvent::NoteOn { note, velocity, .. } => {
                    voice_manager.note_on(note, velocity);
                }
                NoteEvent::NoteOff { note, .. } => {
                    voice_manager.note_off(note);
                }
                _ => {}
            }

            next_event = context.next_event();
        }

        // Generate one sample from voice manager
        let mut mono_sample = [0.0f32];
        voice_manager.process(&mut mono_sample);

        // Apply master gain
        let output_sample = mono_sample[0] * gain;

        // Write to stereo output (duplicate mono to both channels)
        let output = buffer.as_slice();
        for channel_samples in output {
            channel_samples[sample_idx] = output_sample;
        }
    }

    ProcessStatus::Normal
}
```

**Code Analysis:**

```rust
while let Some(event) = next_event {
    if event.timing() > sample_idx as u32 {
        break;
    }
    // Process event
}
```

> **Sample-accurate MIDI**: Events have timing in samples. We process all events **before or at** the current sample before generating audio. This ensures note_on at sample 32 affects audio starting at sample 32, not sample 64.

```rust
let mut mono_sample = [0.0f32];
voice_manager.process(&mut mono_sample);
```

> **Single-sample buffer**: We process one sample at a time because we need to interleave MIDI event handling. Each sample might have a different set of active voices (if a note_on happened).

---

## Testing the Implementation

Run the comprehensive test suite:

```bash
cd naughty-and-tender
cargo test
```

### Key Tests

**Envelope state transitions**:
```rust
#[test]
fn test_envelope_state_transitions() {
    let mut env = ADSREnvelope::new(SAMPLE_RATE);

    assert_eq!(env.get_state(), EnvelopeState::Idle);

    env.note_on(1.0);
    assert_eq!(env.get_state(), EnvelopeState::Attack);

    // Process through attack...
    assert_eq!(env.get_state(), EnvelopeState::Decay);

    // Process through decay...
    assert_eq!(env.get_state(), EnvelopeState::Sustain);

    env.note_off();
    assert_eq!(env.get_state(), EnvelopeState::Release);

    // Process through release...
    assert_eq!(env.get_state(), EnvelopeState::Idle);
}
```

**Voice stealing**:
```rust
#[test]
fn test_voice_stealing_oldest_first() {
    let max_voices = 4;
    let mut vm = VoiceManager::new(SAMPLE_RATE, max_voices);

    // Fill all voices
    vm.note_on(60, 1.0); // Oldest
    vm.note_on(62, 1.0);
    vm.note_on(64, 1.0);
    vm.note_on(65, 1.0);

    // Add one more note - should steal oldest (60)
    vm.note_on(67, 1.0);

    let notes = vm.get_active_notes();
    assert!(!notes.contains(&60), "Note 60 should be stolen");
    assert!(notes.contains(&67), "Note 67 should be active");
}
```

**Polyphonic mixing**:
```rust
#[test]
fn test_polyphony_produces_mixed_output() {
    let mut vm = VoiceManager::new(SAMPLE_RATE, 16);

    // Play chord: C, E, G
    vm.note_on(60, 0.5);
    vm.note_on(64, 0.5);
    vm.note_on(67, 0.5);

    let mut buffer = vec![0.0; 512];
    vm.process(&mut buffer);

    let rms_chord = calculate_rms(&buffer);
    assert!(rms_chord > 0.01, "Chord should produce audible output");
}
```

---

## Building and Testing in a DAW

Let's hear our synthesizer!

```bash
cd naughty-and-tender
cargo build --release
```

**Copy to plugin folder**:

```bash
# macOS
cp target/release/libnaughty_and_tender.dylib ~/Library/Audio/Plug-Ins/VST3/

# Windows
copy target\release\naughty_and_tender.dll "C:\Program Files\Common Files\VST3\"

# Linux
cp target/release/libnaughty_and_tender.so ~/.vst3/
```

**Load in Reaper**:
1. Create MIDI track
2. Add "Naughty and Tender" as instrument
3. Create MIDI clip with notes
4. Adjust envelope parameters (Attack, Decay, Sustain, Release)
5. Try different waveforms

**Expected behavior**:
- ✅ Notes have attack (not instant)
- ✅ Notes sustain while held
- ✅ Notes fade out when released
- ✅ Polyphony works (play chords)
- ✅ No clicks or pops
- ✅ No dropouts or glitches

---

## Troubleshooting

### "Notes never turn off / infinite drone"

**Symptom**: Notes sustain forever even after note_off.

**Causes**:
1. **Release time too long**: Set release to 100-500ms
2. **Envelope not transitioning**: Check `note_off()` sets state to Release
3. **Voice not checking envelope**: Ensure `voice.process()` checks `envelope.is_active()`

**Debug**:
```rust
println!("Envelope state: {:?}", env.get_state());
println!("Envelope active: {}", env.is_active());
```

### "Clicks on note_on or note_off"

**Symptom**: Audible pops when triggering notes.

**Causes**:
1. **Oscillator not reset**: Call `oscillator.reset()` in `voice.note_on()`
2. **Discontinuous envelope**: Ensure attack starts at 0.0
3. **Release not smooth**: Capture `release_start_value` in `note_off()`

**Solution**: Verify phase resets and envelope continuity.

### "Only one note sounds at a time (no polyphony)"

**Symptom**: Playing a chord only sounds one note.

**Causes**:
1. **Not creating multiple voices**: Check `VoiceManager::new()` creates 16 voices
2. **Voice allocation broken**: Verify `note_on()` finds idle voices
3. **Mixing broken**: Ensure `process()` loops through all voices

**Debug**:
```rust
println!("Active voices: {}", vm.active_voice_count());
```

### "Massive volume spikes / clipping"

**Symptom**: Extremely loud output, distortion.

**Causes**:
1. **Additive mixing**: 16 voices at full volume = 16x amplitude
2. **No master gain**: Add gain control in plugin parameters
3. **Envelope scaling wrong**: Check velocity multiplication

**Solution**: Add `gain` parameter (see `params.rs`) and apply to final output:
```rust
let output_sample = mono_sample[0] * gain;
```

---

## Performance Notes

**CPU Usage**:
- 16 voices, 64 samples, 48kHz: ~0.1% CPU (modern processor)
- Sine waveform: Cheapest (one `sin()` call)
- Sawtooth/Square/Triangle: Cheaper (arithmetic only)
- Most CPU time: Envelope state machine (branching)

**Optimization Opportunities**:
1. **SIMD oscillators**: Process 4 samples at once
2. **Branchless envelopes**: Lookup tables instead of state machine
3. **Voice culling**: Skip voices below audibility threshold
4. **Lazy parameter updates**: Only update changed parameters

For now, performance is excellent - no optimization needed.

---

## Key Takeaways

**ADSR Envelopes**:
- ✅ State machine: Idle → Attack → Decay → Sustain → Release → Idle
- ✅ Linear ramps for each phase (could be exponential)
- ✅ Velocity scales peak amplitude
- ✅ Release from any phase (smooth transitions)
- ✅ Handle instant phases with loop + continue

**Voice Management**:
- ✅ Pre-allocate fixed voice pool (real-time safe)
- ✅ Voice states: Idle, Active, Releasing
- ✅ Voice stealing: Prefer releasing, then oldest active
- ✅ Age tracking with monotonic counter
- ✅ Sample-by-sample mixing (critical for envelope sync!)

**Integration**:
- ✅ Oscillator + Envelope = Voice
- ✅ VoiceManager handles polyphony
- ✅ Plugin routes MIDI to VoiceManager
- ✅ Sample-accurate MIDI event handling

---

## What's Next?

**Phase 3: Filtering and Modulation** (Future):
- Low-pass filter with resonance
- Filter envelopes
- LFO (Low-Frequency Oscillator)
- Modulation matrix

**Phase 4: Effects** (Future):
- Reverb
- Delay
- Chorus

**Phase 5: Polish** (Future):
- Better GUI (waveform display, envelope visualization)
- Preset system
- PolyBLEP anti-aliasing
- Optimization (SIMD, voice culling)

---

## Complete Code Reference

**Files implemented in this article**:
- `naughty-and-tender/src/envelope.rs` (835 lines)
- `naughty-and-tender/src/voice.rs` (929 lines)
- `naughty-and-tender/src/lib.rs` (integration, 222 lines)

**Key sections**:

**envelope.rs**:
- Lines 14-21: `EnvelopeState` enum
- Lines 46-76: `ADSREnvelope` struct
- Lines 89-129: Parameter setters
- Lines 135-147: note_on/note_off
- Lines 155-238: `process()` method (state machine)

**voice.rs**:
- Lines 16-21: `VoiceState` enum
- Lines 30-48: `Voice` struct
- Lines 52-156: Voice implementation
- Lines 165-384: `VoiceManager` implementation
- Lines 341-383: `steal_voice()` algorithm
- Lines 396-398: `midi_note_to_frequency()`

**Integration tests**:
- `naughty-and-tender/tests/integration_tests.rs` - Full system tests

---

## Further Reading

**ADSR Envelopes**:
- [Sound on Sound: Synth Secrets Part 3](https://www.soundonsound.com/techniques/synthesizer-envelopes) - Envelope design in depth
- [Will Pirkle - Designing Software Synthesizer Plug-Ins](https://www.willpirkle.com/synth-book/) - Chapter 4: Envelopes
- [Envelope Generators Explained](https://www.perfectcircuit.com/signal/learning-synthesis-envelopes) - Visual guide

**Voice Management**:
- [Voice Allocation Strategies](http://www.martin-finke.de/blog/articles/audio-plugins-013-voice-stealing/) - Deep dive
- [Polyphonic Synthesizers](https://www.soundonsound.com/techniques/practical-polyphony) - Historical context
- [JUCE Forum: Voice Stealing](https://forum.juce.com/t/voice-stealing-algorithm/18967) - Alternative approaches

**State Machines**:
- [Rust Enums as State Machines](https://hoverbear.org/blog/rust-state-machine-pattern/) - Idiomatic Rust patterns
- [State Machine Design in Audio](https://www.youtube.com/watch?v=I-hZkUa9mIs) - ADC talk

**Real-time Audio**:
- [Real-Time Audio Programming](http://www.rossbencina.com/code/real-time-audio-programming-101-time-waits-for-nothing) - Ross Bencina's guide
- [Lock-Free Programming](https://www.youtube.com/watch?v=c1gO9aB9nbs) - Timur Doumler's CppCon talk
- [Audio Thread Safety](https://github.com/free-audio/clap/blob/main/include/clap/thread-check.h) - CLAP guidelines

---

**Next**: Phase 3: Filtering and Modulation (Coming Soon)

**Previous**: [Article 3: Oscillators and Waveform Generation](./03-oscillators-and-waveform-generation.md)

**Series Home**: [Naughty and Tender Development Series](./README.md)
