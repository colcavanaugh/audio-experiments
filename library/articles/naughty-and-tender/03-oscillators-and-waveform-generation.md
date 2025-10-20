# Building Audio Plugins in Rust: Oscillators and Waveform Generation

**Part 3 of the Naughty and Tender Development Series**

---

## What You'll Learn

By the end of this article, you'll understand:

- ✅ How digital oscillators generate waveforms using phase accumulation
- ✅ Why f64 phase accumulators prevent numerical drift
- ✅ Implementing sine, sawtooth, square, and triangle waveforms
- ✅ MIDI note to frequency conversion
- ✅ Real-time safety in audio DSP code
- ✅ Testing audio code with zero-crossing analysis

**Prerequisites:**
- [x] Completed [Article 1: Project Setup](./01-project-setup-and-why-rust.md)
- [x] Completed [Article 2: Plugin Shell](./02-plugin-shell-architecture.md)
- [x] Understanding of basic trigonometry (sine waves)
- [x] Familiarity with audio concepts (frequency, amplitude)

**Time Estimate:** 60-75 minutes

---

## Introduction: The Heart of a Synthesizer

Every synthesizer begins with oscillators - components that generate periodic waveforms at specific frequencies. When you press middle C (261.63 Hz) on a keyboard, an oscillator generates 261.63 complete wave cycles every second.

### Why Oscillators Matter

Oscillators are the **sound source** in subtractive synthesis:
1. **Oscillators** generate rich harmonic content
2. **Filters** shape the frequency spectrum
3. **Envelopes** control amplitude over time
4. **Effects** add color and space

Different waveforms contain different harmonics:
- **Sine**: Pure tone (no harmonics)
- **Sawtooth**: Bright, buzzy (all harmonics)
- **Square**: Hollow, clarinet-like (odd harmonics only)
- **Triangle**: Softer square (odd harmonics, weaker than square)

> **💡 Tooltip: What Are Harmonics?**
>
> Harmonics are integer multiples of the fundamental frequency. If you play A4 (440 Hz), its harmonics are 880 Hz, 1320 Hz, 1760 Hz, etc. Different waveforms contain different harmonics, giving each its characteristic timbre. [Learn more](https://en.wikipedia.org/wiki/Harmonic)

### Digital vs. Analog Oscillators

**Analog oscillators** use voltage-controlled circuits. A capacitor charges and discharges to create waveforms.

**Digital oscillators** use mathematics to generate sample values. We'll use **phase accumulation** - the standard digital synthesis technique.

> **📚 Further Reading:**
> - [Digital Signal Processing - Smith](https://www.dspguide.com/) - Comprehensive DSP fundamentals
> - [The Audio Programming Book](https://mitpress.mit.edu/9780262014465/the-audio-programming-book/) - Chapter on oscillators
> - [Music DSP Archive](https://www.musicdsp.org/) - Oscillator implementations

---

## Phase Accumulation: The Core Concept

Phase accumulation is elegant: maintain a **phase value** that wraps from 0 to 1, then map it to a waveform.

### The Algorithm

```rust
// Phase ranges from 0.0 to 1.0 (one complete cycle)
let mut phase: f64 = 0.0;

loop {
    // 1. Generate waveform value from current phase
    let output = generate_waveform(phase);

    // 2. Calculate phase increment (how much to advance per sample)
    let phase_increment = frequency / sample_rate;

    // 3. Advance phase
    phase += phase_increment;

    // 4. Wrap phase at 1.0
    if phase >= 1.0 {
        phase -= 1.0;
    }
}
```

**Example**: At 440 Hz and 44100 Hz sample rate:
- `phase_increment = 440 / 44100 = 0.00997732...`
- After 100 samples: `phase = 0.997732` (almost one complete cycle)
- After 101 samples: `phase = 0.00748` (wrapped back to start)

> **💡 Tooltip: Why Use 0-1 Instead of 0-2π?**
>
> Using 0-1 for phase is a convention in digital audio because:
> - Division is simpler than modulo 2π
> - Easier to visualize (50% through cycle = 0.5)
> - Conversion to radians happens once: `angle = phase * 2π`

### Why f64 for Phase?

This is crucial for long-running oscillators:

```rust
// ❌ BAD: f32 accumulator
let mut phase_f32: f32 = 0.0;
for _ in 0..1_000_000_000 {
    phase_f32 += 0.00997732; // 440 Hz at 44.1kHz
    if phase_f32 >= 1.0 { phase_f32 -= 1.0; }
}
// After 5 minutes, rounding errors accumulate and pitch drifts!

// ✅ GOOD: f64 accumulator
let mut phase_f64: f64 = 0.0;
for _ in 0..1_000_000_000 {
    phase_f64 += 0.00997732; // 440 Hz at 44.1kHz
    if phase_f64 >= 1.0 { phase_f64 -= 1.0; }
}
// Stable for hours of continuous playback
```

**f32** has ~7 decimal digits of precision. Small phase increments lose precision when added repeatedly.

**f64** has ~15 decimal digits - enough to maintain stable pitch indefinitely.

We use f64 for phase, but output f32 samples (audio standard).

---

## Implementing the Oscillator Module

Let's build our oscillator from scratch. We'll use test-driven development to verify correctness.

### Module Structure

Create `naughty-and-tender/src/oscillators.rs`:

```rust
//! Oscillator module for Naughty and Tender
//!
//! This module contains various oscillator implementations (sine, saw, square, triangle)
//! with proper frequency control and phase management.

use std::f32::consts::PI;

/// Waveform types available for oscillators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveformType {
    Sine,
    Sawtooth,
    Square,
    Triangle,
}
```

**Code Analysis:**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
```

> **Derive macros explained**:
> - `Debug`: Enables printing with `{:?}` for debugging
> - `Clone`: Allows copying with `.clone()`
> - `Copy`: Enables cheap bitwise copying (enum is just an integer)
> - `PartialEq, Eq`: Enables equality comparison with `==`
>
> [Trait derivation documentation](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

### The Oscillator Struct

```rust
/// Multi-waveform oscillator with phase accumulation
///
/// Uses f64 for phase accumulation to prevent numerical drift over long periods.
/// The phase is normalized to 0.0-1.0 range for easier waveform generation.
///
/// # Real-time Safety
/// - No allocations in process methods
/// - All state pre-initialized in `new()`
/// - Uses inline functions for hot path
pub struct Oscillator {
    /// Phase accumulator (0.0 to 1.0)
    /// Uses f64 for numerical stability - f32 can drift over time
    phase: f64,

    /// Sample rate in Hz
    sample_rate: f32,
}
```

**Code Analysis:**

```rust
/// # Real-time Safety
```

> **Documentation sections**: We use special headers in doc comments to highlight critical information. The `#` creates a section in generated documentation. [Doc comment conventions](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)

### Constructor and Reset

```rust
impl Oscillator {
    /// Create a new oscillator
    ///
    /// # Arguments
    /// * `sample_rate` - Sample rate in Hz (e.g., 44100.0, 48000.0)
    #[must_use]
    pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            sample_rate,
        }
    }

    /// Reset phase to zero (for synced oscillators or voice reset)
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }
}
```

**Code Analysis:**

```rust
#[must_use]
```

> **Lint attribute**: This warns if someone calls `Oscillator::new()` without using the result. Constructors should always return a value that's used. [Lint attributes](https://doc.rust-lang.org/reference/attributes/diagnostics.html#lint-check-attributes)

---

## Implementing Waveforms

Now for the fun part - generating actual audio!

### Sine Wave

The purest waveform - a single frequency with no harmonics.

```rust
/// Process one sample of sine waveform
///
/// Uses standard sine formula: sin(2π * phase)
///
/// # Arguments
/// * `frequency` - Frequency in Hz (20-20000 typical range)
///
/// # Returns
/// Sine wave sample (-1.0 to 1.0)
#[inline]
#[allow(clippy::cast_possible_truncation)] // f64 phase -> f32 output is intentional
pub fn process_sine(&mut self, frequency: f32) -> f32 {
    // Calculate sine value at current phase
    let output = (self.phase as f32 * 2.0 * PI).sin();

    // Advance phase
    self.advance_phase(frequency);

    output
}
```

**Code Analysis:**

```rust
#[inline]
```

> **Inlining**: Suggests the compiler insert this function's code directly at call sites, avoiding function call overhead. Critical for per-sample processing. The compiler may ignore this hint if the function is complex. [Inline documentation](https://doc.rust-lang.org/reference/attributes/codegen.html#the-inline-attribute)

```rust
let output = (self.phase as f32 * 2.0 * PI).sin();
```

> **Phase to angle conversion**:
> - Phase: 0.0 to 1.0 (normalized)
> - Angle: 0.0 to 2π radians
> - Conversion: `angle = phase * 2π`
> - Then: `sin(angle)` produces -1.0 to 1.0

**Mathematics**:
- When `phase = 0.0`: `sin(0) = 0.0`
- When `phase = 0.25`: `sin(π/2) = 1.0` (peak)
- When `phase = 0.5`: `sin(π) = 0.0`
- When `phase = 0.75`: `sin(3π/2) = -1.0` (trough)
- When `phase = 1.0`: `sin(2π) = 0.0` (back to start)

### Sawtooth Wave

A linear ramp from -1 to +1, then discontinuous jump back to -1.

```rust
/// Process one sample of sawtooth waveform
///
/// Rising sawtooth from -1 to almost +1, then wraps.
/// Note: This is a naive implementation that will alias at high frequencies.
/// Future enhancement: Use PolyBLEP for anti-aliasing.
///
/// # Arguments
/// * `frequency` - Frequency in Hz
///
/// # Returns
/// Sawtooth sample (-1.0 to ~1.0)
#[inline]
#[allow(clippy::cast_possible_truncation)] // f64 phase -> f32 output is intentional
pub fn process_sawtooth(&mut self, frequency: f32) -> f32 {
    // Standard sawtooth: linear ramp from -1 to +1
    // This creates 2 zero crossings per cycle: one during the ramp (at phase ~0.5)
    // and one at the discontinuity (from +1 wrapping back to -1)
    let output = (2.0 * self.phase as f32) - 1.0;

    // Advance phase
    self.advance_phase(frequency);

    output
}
```

**Code Analysis:**

```rust
let output = (2.0 * self.phase as f32) - 1.0;
```

> **Linear mapping**:
> - When `phase = 0.0`: `output = -1.0`
> - When `phase = 0.5`: `output = 0.0`
> - When `phase = 1.0`: `output = 1.0`
> - Then phase wraps to 0.0, creating the discontinuity

> **⚠️ Aliasing Warning**
>
> This naive sawtooth will **alias** at high frequencies. Aliasing occurs when frequencies above Nyquist (sample_rate / 2) fold back into the audible spectrum, creating harsh digital artifacts.
>
> **Solutions**:
> - **PolyBLEP** (Polynomial Band-Limited Step): Add correction at discontinuities
> - **Wavetables**: Pre-compute band-limited waveforms
> - **Additive synthesis**: Sum band-limited harmonics
>
> For Phase 2, we accept aliasing. Phase 3 will add PolyBLEP.
>
> [Learn more about aliasing](https://en.wikipedia.org/wiki/Aliasing)

### Square Wave

Alternates between -1 and +1 based on phase.

```rust
/// Process one sample of square waveform
///
/// Output is -1 or +1 based on phase being below or above 0.5 (50% duty cycle).
/// Note: Naive implementation will alias. Future: PolyBLEP.
///
/// # Arguments
/// * `frequency` - Frequency in Hz
///
/// # Returns
/// Square wave sample (-1.0 or 1.0)
#[inline]
pub fn process_square(&mut self, frequency: f32) -> f32 {
    // Square wave: -1 for first half of cycle, +1 for second half
    let output = if self.phase < 0.5 { -1.0 } else { 1.0 };

    // Advance phase
    self.advance_phase(frequency);

    output
}
```

**Code Analysis:**

```rust
let output = if self.phase < 0.5 { -1.0 } else { 1.0 };
```

> **Duty cycle**: The percentage of time the wave is "high". Our square wave is 50% duty cycle (half the time at -1, half at +1). Variable duty cycle (pulse width modulation) is possible by changing the threshold from 0.5.

### Triangle Wave

Linear ramp up, then linear ramp down.

```rust
/// Process one sample of triangle waveform
///
/// Tent function: rises from -1 to +1 in first half, falls from +1 to -1 in second half.
///
/// # Arguments
/// * `frequency` - Frequency in Hz
///
/// # Returns
/// Triangle wave sample (-1.0 to 1.0)
#[inline]
#[allow(clippy::cast_possible_truncation)] // f64 phase -> f32 output is intentional
pub fn process_triangle(&mut self, frequency: f32) -> f32 {
    // Triangle wave: linear interpolation up then down
    let output = if self.phase < 0.5 {
        // Rising: -1 to +1 (phase 0.0 to 0.5)
        -1.0 + (4.0 * self.phase as f32)
    } else {
        // Falling: +1 to -1 (phase 0.5 to 1.0)
        3.0 - (4.0 * self.phase as f32)
    };

    // Advance phase
    self.advance_phase(frequency);

    output
}
```

**Code Analysis:**

**Rising segment** (phase 0.0 to 0.5):
```
output = -1.0 + (4.0 * phase)

phase = 0.0:  output = -1.0 + 0.0 = -1.0
phase = 0.25: output = -1.0 + 1.0 = 0.0
phase = 0.5:  output = -1.0 + 2.0 = 1.0
```

**Falling segment** (phase 0.5 to 1.0):
```
output = 3.0 - (4.0 * phase)

phase = 0.5:  output = 3.0 - 2.0 = 1.0
phase = 0.75: output = 3.0 - 3.0 = 0.0
phase = 1.0:  output = 3.0 - 4.0 = -1.0
```

### Phase Advancement

The shared logic for all waveforms:

```rust
/// Advance the phase accumulator and wrap at 1.0
///
/// Phase increment = frequency / sample_rate
/// This gives the fraction of a cycle completed per sample.
///
/// # Arguments
/// * `frequency` - Frequency in Hz
#[inline]
fn advance_phase(&mut self, frequency: f32) {
    // Calculate phase increment per sample
    let phase_inc = f64::from(frequency / self.sample_rate);

    // Advance phase
    self.phase += phase_inc;

    // Wrap phase at 1.0 to prevent drift
    // Using while loop handles edge case of very high frequencies
    while self.phase >= 1.0 {
        self.phase -= 1.0;
    }

    // Handle negative frequencies (reverse direction)
    while self.phase < 0.0 {
        self.phase += 1.0;
    }
}
```

**Code Analysis:**

```rust
let phase_inc = f64::from(frequency / self.sample_rate);
```

> **Type conversion**: `f64::from()` converts f32 to f64 losslessly (f32 fits in f64). This is safer than `as f64` which doesn't check for overflow. [Type conversion documentation](https://doc.rust-lang.org/std/convert/trait.From.html)

```rust
while self.phase >= 1.0 {
    self.phase -= 1.0;
}
```

> **Why while instead of if?**: For extremely high frequencies (above Nyquist), `phase_inc` might exceed 1.0. The while loop handles this edge case. In practice, audio frequencies won't trigger this, but defensive coding prevents bugs.

---

## MIDI to Frequency Conversion

MIDI notes are integers (0-127). We need to convert them to frequencies.

### The Formula

MIDI uses **equal temperament tuning** where each semitone is exactly `2^(1/12)` times the previous:

```
frequency = 440 * 2^((note - 69) / 12)
```

Where:
- `440` Hz is A4 (MIDI note 69)
- `note - 69` is semitones from A4
- Division by 12 converts semitones to octaves
- `2^octaves` multiplies frequency

**Examples**:
- MIDI 69 (A4): `440 * 2^0 = 440 Hz`
- MIDI 81 (A5): `440 * 2^1 = 880 Hz` (octave up)
- MIDI 57 (A3): `440 * 2^(-1) = 220 Hz` (octave down)
- MIDI 60 (C4): `440 * 2^(-9/12) = 261.63 Hz`

### Implementation

While we'll implement this in the voice module (next article), here's the conversion function:

```rust
/// Convert MIDI note number to frequency in Hz
///
/// Uses standard MIDI tuning: A4 (note 69) = 440 Hz
pub fn midi_note_to_frequency(note: u8) -> f32 {
    440.0 * 2.0f32.powf((f32::from(note) - 69.0) / 12.0)
}
```

> **💡 Tooltip: Why A4 = 440 Hz?**
>
> This is **concert pitch** - the standard tuning reference agreed upon internationally in 1955. Historically, A4 varied from 415-466 Hz. Some orchestras use 442 Hz for brighter tone. [Learn more](https://en.wikipedia.org/wiki/A440_(pitch_standard))

---

## Testing Oscillators

Audio code is tricky to test - we can't just check if samples equal specific values (floating-point precision, phase offset). Instead, we test **properties**:

### Zero-Crossing Analysis

Zero crossings (signal crossing 0.0) reveal frequency:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to count zero crossings in a waveform
    fn count_zero_crossings(samples: &[f32]) -> usize {
        samples
            .windows(2)
            .filter(|window| {
                (window[0] < 0.0 && window[1] >= 0.0) ||
                (window[0] >= 0.0 && window[1] < 0.0)
            })
            .count()
    }

    #[test]
    fn test_sine_wave_frequency_accuracy() {
        let sample_rate = 44100.0;
        let frequency = 440.0; // A4
        let mut osc = Oscillator::new(sample_rate);

        // Generate 1 second of audio
        let samples: Vec<f32> = (0..44100)
            .map(|_| osc.process_sine(frequency))
            .collect();

        // For 440 Hz, we expect 880 zero crossings in 1 second (2 per cycle)
        let zero_crossings = count_zero_crossings(&samples);
        assert!(
            (zero_crossings as i32 - 880).abs() < 4,
            "Expected ~880 zero crossings for 440 Hz, got {}",
            zero_crossings
        );
    }
}
```

**Code Analysis:**

```rust
.windows(2)
```

> **Sliding window iterator**: Produces overlapping pairs of elements: `[a, b, c, d]` becomes `[[a,b], [b,c], [c,d]]`. Perfect for comparing adjacent samples. [Windows documentation](https://doc.rust-lang.org/std/primitive.slice.html#method.windows)

**Why allow ±4 zero crossings?**
- Floating-point rounding in phase accumulation
- Phase offset (where we start in the cycle)
- Edge effects at buffer boundaries

Exact match would be fragile; ±4 is tolerance for real-world conditions.

### RMS Amplitude Testing

Root Mean Square measures average power:

```rust
// Helper to calculate RMS of a signal
fn calculate_rms(samples: &[f32]) -> f32 {
    let sum_squares: f32 = samples.iter().map(|s| s * s).sum();
    (sum_squares / samples.len() as f32).sqrt()
}

#[test]
fn test_sine_wave_amplitude() {
    let mut osc = Oscillator::new(44100.0);

    let samples: Vec<f32> = (0..1000)
        .map(|_| osc.process_sine(440.0))
        .collect();

    let max_amplitude = samples.iter().map(|s| s.abs()).fold(0.0f32, f32::max);

    // Should be close to 1.0
    assert!(
        (max_amplitude - 1.0).abs() < 0.01,
        "Expected max amplitude ~1.0, got {}",
        max_amplitude
    );

    // RMS should be approximately 1/sqrt(2) = 0.707 for sine wave
    let rms = calculate_rms(&samples);
    assert!(
        (rms - 0.707).abs() < 0.05,
        "Expected RMS ~0.707, got {}",
        rms
    );
}
```

**Code Analysis:**

> **Why is sine RMS 0.707?**
>
> For a sine wave with amplitude 1:
> ```
> RMS = √(mean(sin²(x))) = √(1/2) = 0.707
> ```
> This is because `sin²(x)` averages to 0.5 over a complete cycle.
>
> Square waves have RMS = 1.0 (always ±1).
> Sawtooth/triangle have different RMS values.

### Edge Case Testing

Real-time audio encounters weird situations:

```rust
#[test]
fn test_zero_frequency_edge_case() {
    let mut osc = Oscillator::new(44100.0);

    for _ in 0..100 {
        let sample = osc.process_sine(0.0);
        assert!(sample.is_finite(), "Zero frequency should produce finite output");
    }
}

#[test]
fn test_nyquist_frequency_edge_case() {
    // Frequency at Nyquist (sample_rate / 2) is the edge of valid range
    let sample_rate = 44100.0;
    let mut osc = Oscillator::new(sample_rate);

    let samples: Vec<f32> = (0..1000)
        .map(|_| osc.process_sine(sample_rate / 2.0))
        .collect();

    // Should produce alternating +1, -1 (or close to it)
    for sample in samples {
        assert!(sample.is_finite(), "Nyquist frequency should be stable");
        assert!(sample.abs() <= 1.1, "Amplitude should be reasonable");
    }
}

#[test]
fn test_phase_accumulation_wraps_correctly() {
    let mut osc = Oscillator::new(44100.0);

    // Run for a long time at high frequency
    for _ in 0..100000 {
        let sample = osc.process_sine(10000.0);
        assert!(
            sample.is_finite(),
            "Sample should be finite (phase wrapping working)"
        );
    }
}
```

**Code Analysis:**

```rust
assert!(sample.is_finite(), ...);
```

> **Detecting NaN and infinity**: `is_finite()` returns false for NaN (Not a Number) or ±infinity. These values indicate bugs (division by zero, overflow). Real-time audio should never produce NaN/infinity - they cause silence or loud noise. [is_finite documentation](https://doc.rust-lang.org/std/primitive.f32.html#method.is_finite)

---

## Integration and Performance

### Module Declaration

Add to `naughty-and-tender/src/lib.rs`:

```rust
// Phase 2 modules
pub mod oscillators;
```

The `pub` makes the module accessible to external code (tests, other plugins).

### Using Oscillators

Here's how voices (next article) will use oscillators:

```rust
use crate::oscillators::{Oscillator, WaveformType};

// In voice struct
struct Voice {
    oscillator: Oscillator,
    waveform: WaveformType,
    // ...
}

// In process method
let frequency = midi_note_to_frequency(self.note);
let sample = match self.waveform {
    WaveformType::Sine => self.oscillator.process_sine(frequency),
    WaveformType::Sawtooth => self.oscillator.process_sawtooth(frequency),
    WaveformType::Square => self.oscillator.process_square(frequency),
    WaveformType::Triangle => self.oscillator.process_triangle(frequency),
};
```

### Performance Considerations

**Inline functions**: All process methods are marked `#[inline]`. At 48kHz with 64-sample buffers, these functions are called ~750,000 times per second. Inlining eliminates call overhead.

**No branches in hot path**: Waveform selection happens outside the sample loop (in voice struct). Within each waveform function, logic is minimal.

**SIMD potential**: Future optimization could vectorize oscillator processing with SIMD (Single Instruction Multiple Data). Rust's `std::simd` (nightly) or libraries like `wide` enable this.

---

## Running the Tests

Let's verify our implementation:

```bash
cd naughty-and-tender
cargo test oscillators
```

Expected output:
```
running 15 tests
test oscillators::tests::test_oscillator_creation ... ok
test oscillators::tests::test_sine_wave_frequency_accuracy ... ok
test oscillators::tests::test_sine_wave_amplitude ... ok
test oscillators::tests::test_sawtooth_wave_frequency_accuracy ... ok
test oscillators::tests::test_square_wave_frequency_accuracy ... ok
test oscillators::tests::test_triangle_wave_frequency_accuracy ... ok
test oscillators::tests::test_zero_frequency_edge_case ... ok
test oscillators::tests::test_nyquist_frequency_edge_case ... ok
test oscillators::tests::test_phase_accumulation_wraps_correctly ... ok
...

test result: ok. 15 passed; 0 failed
```

All tests should pass! If you see failures, check:
- Phase wrapping logic
- Type conversions (f32 vs f64)
- Waveform formulas

---

## Troubleshooting

### "Pitch drifts over time"

**Symptom**: Oscillator starts at correct frequency but slowly drifts sharp or flat.

**Cause**: Using f32 for phase accumulation.

**Solution**: Ensure `phase` is `f64`:
```rust
pub struct Oscillator {
    phase: f64,  // ✅ Not f32!
    // ...
}
```

### "Frequency is double what I expect"

**Symptom**: A4 (440 Hz) sounds like A5 (880 Hz).

**Cause**: Counting both positive and negative zero crossings.

**Solution**: This is actually correct! Sine waves have 2 zero crossings per cycle (upward and downward). If you want fundamental frequency, divide zero crossings by 2.

### "Sawtooth sounds harsh and digital"

**Symptom**: Sawtooth has nasty high-frequency buzz, especially at high notes.

**Cause**: Aliasing from the discontinuity.

**Solution**: This is expected with naive waveforms. Phase 3 will add PolyBLEP anti-aliasing. For now, test with lower frequencies (<1000 Hz).

### "Tests fail with 'assertion failed: zero_crossings ~880'"

**Symptom**: Zero crossing count is way off (like 440 or 1760).

**Causes**:
1. **Phase increment wrong**: Check `frequency / sample_rate` calculation
2. **Phase not wrapping**: Ensure `while phase >= 1.0 { phase -= 1.0; }`
3. **Waveform formula wrong**: Double-check sine/saw/square/triangle math

**Debug**:
```rust
// Print first few samples
for i in 0..10 {
    let sample = osc.process_sine(440.0);
    println!("Sample {}: {}", i, sample);
}
```

---

## Key Takeaways

**Phase Accumulation**:
- ✅ Use 0-1 normalized phase for simplicity
- ✅ f64 prevents numerical drift
- ✅ `phase_increment = frequency / sample_rate`
- ✅ Wrap at 1.0 every sample

**Waveforms**:
- ✅ Sine: `sin(2π * phase)` - pure tone
- ✅ Sawtooth: `2*phase - 1` - bright, buzzy
- ✅ Square: `phase < 0.5 ? -1 : 1` - hollow
- ✅ Triangle: Piecewise linear - softer square

**Real-time Safety**:
- ✅ No allocations in process methods
- ✅ Inline hot path functions
- ✅ Pre-allocate all state in constructor

**Testing**:
- ✅ Zero-crossing analysis verifies frequency
- ✅ RMS analysis verifies amplitude
- ✅ Edge cases prevent runtime panics

---

## What's Next?

In **Article 4: ADSR Envelopes and Voice Management**, we'll:
- Implement Attack-Decay-Sustain-Release envelopes
- Create a state machine for envelope phases
- Build polyphonic voice management
- Integrate oscillators and envelopes
- Handle MIDI note on/off events
- Implement voice stealing for polyphony limits

Our oscillators generate continuous tones. Envelopes will shape them into musical notes with natural attack and decay!

---

## Complete Code Reference

**File: `naughty-and-tender/src/oscillators.rs`** (498 lines)

Key sections:
- Lines 16-22: `WaveformType` enum
- Lines 41-60: `Oscillator` struct and constructor
- Lines 76-86: `process_sine()`
- Lines 104-142: `process_sawtooth()`
- Lines 154-163: `process_square()`
- Lines 175-190: `process_triangle()`
- Lines 199-218: `advance_phase()` (shared logic)
- Lines 220-593: Comprehensive test suite

The full implementation is in the repository at:
`C:\Users\colca\OneDrive\Desktop\Audio\Experiments\naughty-and-tender\src\oscillators.rs`

---

## Further Reading

**Digital Oscillators**:
- [Julius O. Smith - Spectral Audio Signal Processing](https://ccrma.stanford.edu/~jos/sasp/) - Academic treatment
- [The Audio Programmer YouTube](https://www.youtube.com/c/TheAudioProgrammer) - Practical DSP tutorials
- [Designing Sound by Andy Farnell](https://mitpress.mit.edu/books/designing-sound) - Synthesis fundamentals

**Anti-aliasing**:
- [PolyBLEP Paper](http://www.martin-finke.de/blog/articles/audio-plugins-018-polyblep-oscillator/) - Anti-aliasing technique
- [Music DSP - Oscillators](https://www.musicdsp.org/en/latest/Synthesis/) - Community algorithms

**Rust Audio**:
- [RustAudio GitHub](https://github.com/RustAudio) - Audio ecosystem
- [dasp crate](https://docs.rs/dasp/) - Digital audio signal processing primitives
- [fundsp crate](https://docs.rs/fundsp/) - Functional DSP library

**Test-Driven DSP**:
- [Testing Audio Code](https://www.katjaas.nl/home/home.html) - Practical testing strategies
- [Real-time Audio Testing](https://github.com/free-audio/clap/blob/main/include/clap/ext/draft/check-for-update.h) - CLAP validation approach

---

**Next**: [Article 4: ADSR Envelopes and Voice Management](./04-envelopes-and-voice-management.md)

**Previous**: [Article 2: The Plugin Shell](./02-plugin-shell-architecture.md)

**Series Home**: [Naughty and Tender Development Series](./README.md)
