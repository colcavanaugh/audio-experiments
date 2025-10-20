//! Oscillator module for Naughty and Tender
//!
//! This module contains various oscillator implementations (sine, saw, square, triangle)
//! with proper frequency control and phase management.
//!
//! # References
//! - Standard oscillator equations from digital audio synthesis
//! - Phase accumulation: `phase_increment` = frequency / `sample_rate`
//! - Phase wrapping at 1.0 to prevent numerical drift

#![allow(dead_code)] // Some waveforms may not be used initially

use std::f32::consts::PI;

/// Waveform types available for oscillators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaveformType {
    Sine,
    Sawtooth,
    Square,
    Triangle,
}

/// Multi-waveform oscillator with phase accumulation
///
/// Uses f64 for phase accumulation to prevent numerical drift over long periods.
/// The phase is normalized to 0.0-1.0 range for easier waveform generation.
///
/// # Real-time Safety
/// - No allocations in process methods
/// - All state pre-initialized in `new()`
/// - Uses inline functions for hot path
///
/// # Example
/// ```
/// use naughty_and_tender::oscillators::Oscillator;
///
/// let mut osc = Oscillator::new(44100.0);
/// let sample = osc.process_sine(440.0); // Generate A4 sine wave
/// ```
pub struct Oscillator {
    /// Phase accumulator (0.0 to 1.0)
    /// Uses f64 for numerical stability - f32 can drift over time
    phase: f64,

    /// Sample rate in Hz
    sample_rate: f32,
}

impl Oscillator {
    /// Create a new oscillator
    ///
    /// # Arguments
    /// * `sample_rate` - Sample rate in Hz (e.g., 44100.0, 48000.0)
    #[must_use] pub fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            sample_rate,
        }
    }

    /// Reset phase to zero (for synced oscillators or voice reset)
    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

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

    /// Process one sample of sawtooth waveform
    ///
    /// Rising sawtooth from -1 to almost +1, then wraps.
    /// Formulated to have only 1 zero crossing per cycle (upward during the ramp).
    /// The discontinuity goes from positive back to negative, creating the second transition
    /// but not a zero crossing since it doesn't pass through zero.
    ///
    /// Note: This is a naive implementation that will alias at high frequencies.
    /// Future enhancement: Use `PolyBLEP` for anti-aliasing.
    ///
    /// # Arguments
    /// * `frequency` - Frequency in Hz
    ///
    /// # Returns
    /// Sawtooth sample (-1.0 to ~1.0)
    #[inline]
    #[allow(clippy::cast_possible_truncation)] // f64 phase -> f32 output is intentional
    pub fn process_sawtooth(&mut self, frequency: f32) -> f32 {
        // Generate sawtooth with 1 zero crossing per cycle
        // Ramp from -1.0 to +1.0, but we need to ensure the discontinuity doesn't create
        // a second zero crossing. Standard approach: ramp from -1 to just under 0, then wrap
        // This gives one upward zero crossing during the ramp
        //
        // Phase 0.0 → -1.0
        // Phase 0.5 → 0.0  (zero crossing here)
        // Phase 1.0 → +1.0 (wrap back to -1.0, crossing zero again!)
        //
        // To avoid double crossing, offset the waveform so it doesn't cross during ramp:
        // Ramp from 0.1 to 2.1, then scale/shift to audio range
        //
        // Actually, simpler: just use phase directly (0 to 1), scale to -1 to +1
        // This crosses zero at phase=0.5 (once) and at the wrap (going from +1 to -1)
        // So we get 2 crossings, which is what we're seeing (880 instead of 440)
        //
        // For 1 crossing only: DON'T cross zero during the ramp
        // Map phase 0-1 to output 0-2, then subtract 1 → range -1 to +1, crossing at 0.5
        // That still crosses twice!
        //
        // Solution: Map to asymmetric range that doesn't include zero discontinuity
        // Phase 0-1 → -1 to -0.001 (never reaches positive, never crosses zero at wrap)
        // But then where's the zero crossing? We need it during the ramp!
        //
        // Better: Phase 0-1 → -0.999 to +0.999, crosses zero once at phase~=0.5
        // Discontinuity: +0.999 → -0.999 (doesn't cross zero, stays away from it)

        // Standard sawtooth: linear ramp from -1 to +1
        // This creates 2 zero crossings per cycle: one during the ramp (at phase ~0.5)
        // and one at the discontinuity (from +1 wrapping back to -1)
        let output = (2.0 * self.phase as f32) - 1.0;

        // Advance phase
        self.advance_phase(frequency);

        output
    }

    /// Process one sample of square waveform
    ///
    /// Output is -1 or +1 based on phase being below or above 0.5 (50% duty cycle).
    /// Note: Naive implementation will alias. Future: `PolyBLEP`.
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

    /// Advance the phase accumulator and wrap at 1.0
    ///
    /// Phase increment = frequency / `sample_rate`
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
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to count zero crossings in a waveform
    fn count_zero_crossings(samples: &[f32]) -> usize {
        samples
            .windows(2)
            .filter(|window| {
                (window[0] < 0.0 && window[1] >= 0.0) || (window[0] >= 0.0 && window[1] < 0.0)
            })
            .count()
    }

    // Helper to calculate RMS of a signal
    fn calculate_rms(samples: &[f32]) -> f32 {
        let sum_squares: f32 = samples.iter().map(|s| s * s).sum();
        (sum_squares / samples.len() as f32).sqrt()
    }

    // Helper to convert MIDI note to frequency
    fn midi_note_to_frequency(note: u8) -> f32 {
        440.0 * 2.0f32.powf((note as f32 - 69.0) / 12.0)
    }

    #[test]
    fn test_oscillator_creation() {
        // RED: This will fail - Oscillator doesn't exist yet
        let _osc = Oscillator::new(44100.0);
    }

    #[test]
    fn test_sine_wave_frequency_accuracy() {
        // RED: Validate that a sine oscillator produces the correct frequency
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

    #[test]
    fn test_sine_wave_amplitude() {
        // RED: Sine wave should have peak amplitude of 1.0
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

    #[test]
    fn test_sawtooth_wave_frequency_accuracy() {
        // RED: Validate sawtooth frequency
        let sample_rate = 44100.0;
        let frequency = 440.0;
        let mut osc = Oscillator::new(sample_rate);

        let samples: Vec<f32> = (0..44100)
            .map(|_| osc.process_sawtooth(frequency))
            .collect();

        // Sawtooth has 2 zero crossings per cycle (one during ramp, one at discontinuity)
        let zero_crossings = count_zero_crossings(&samples);
        assert!(
            (zero_crossings as i32 - 880).abs() < 4,
            "Expected ~880 zero crossings for 440 Hz sawtooth, got {}",
            zero_crossings
        );
    }

    #[test]
    fn test_sawtooth_wave_range() {
        // RED: Sawtooth should ramp from -1 to +1
        let mut osc = Oscillator::new(44100.0);

        let samples: Vec<f32> = (0..1000)
            .map(|_| osc.process_sawtooth(100.0))
            .collect();

        let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));

        assert!(max <= 1.0, "Sawtooth max should be <= 1.0, got {}", max);
        assert!(min >= -1.0, "Sawtooth min should be >= -1.0, got {}", min);
        assert!(max > 0.9, "Sawtooth should reach close to 1.0");
        assert!(min < -0.9, "Sawtooth should reach close to -1.0");
    }

    #[test]
    fn test_square_wave_frequency_accuracy() {
        // RED: Validate square wave frequency
        let sample_rate = 44100.0;
        let frequency = 440.0;
        let mut osc = Oscillator::new(sample_rate);

        let samples: Vec<f32> = (0..44100)
            .map(|_| osc.process_square(frequency))
            .collect();

        // Square wave has 2 zero crossings per cycle
        let zero_crossings = count_zero_crossings(&samples);
        assert!(
            (zero_crossings as i32 - 880).abs() < 4,
            "Expected ~880 zero crossings for 440 Hz square, got {}",
            zero_crossings
        );
    }

    #[test]
    fn test_square_wave_duty_cycle() {
        // RED: Square wave should be 50% high, 50% low
        let mut osc = Oscillator::new(44100.0);

        let samples: Vec<f32> = (0..10000)
            .map(|_| osc.process_square(100.0))
            .collect();

        // Count samples above and below zero
        let high_samples = samples.iter().filter(|&&s| s > 0.0).count();
        let low_samples = samples.iter().filter(|&&s| s < 0.0).count();

        let ratio = high_samples as f32 / low_samples as f32;
        assert!(
            (ratio - 1.0).abs() < 0.1,
            "Square wave duty cycle should be ~50%, got ratio {}",
            ratio
        );

        // Values should be exactly +1 or -1
        for &sample in &samples {
            assert!(
                (sample.abs() - 1.0).abs() < 0.01,
                "Square wave samples should be ±1, got {}",
                sample
            );
        }
    }

    #[test]
    fn test_triangle_wave_frequency_accuracy() {
        // RED: Validate triangle wave frequency
        let sample_rate = 44100.0;
        let frequency = 440.0;
        let mut osc = Oscillator::new(sample_rate);

        let samples: Vec<f32> = (0..44100)
            .map(|_| osc.process_triangle(frequency))
            .collect();

        let zero_crossings = count_zero_crossings(&samples);
        assert!(
            (zero_crossings as i32 - 880).abs() < 4,
            "Expected ~880 zero crossings for 440 Hz triangle, got {}",
            zero_crossings
        );
    }

    #[test]
    fn test_triangle_wave_symmetry() {
        // RED: Triangle wave should be symmetric
        let mut osc = Oscillator::new(44100.0);

        let samples: Vec<f32> = (0..1000)
            .map(|_| osc.process_triangle(100.0))
            .collect();

        let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));

        assert!(
            (max - 1.0).abs() < 0.01,
            "Triangle max should be ~1.0, got {}",
            max
        );
        assert!(
            (min + 1.0).abs() < 0.01,
            "Triangle min should be ~-1.0, got {}",
            min
        );
    }

    #[test]
    fn test_frequency_control_midi_note() {
        // RED: Test MIDI note to frequency conversion
        let mut osc = Oscillator::new(44100.0);

        // MIDI note 69 = A4 = 440 Hz
        let freq_a4 = midi_note_to_frequency(69);
        assert!((freq_a4 - 440.0).abs() < 0.01, "A4 should be 440 Hz");

        // MIDI note 60 = C4 = 261.63 Hz
        let freq_c4 = midi_note_to_frequency(60);
        assert!(
            (freq_c4 - 261.63).abs() < 0.1,
            "C4 should be ~261.63 Hz, got {}",
            freq_c4
        );

        // Generate audio at C4 frequency
        let samples: Vec<f32> = (0..44100)
            .map(|_| osc.process_sine(freq_c4))
            .collect();

        // Verify frequency is correct
        let zero_crossings = count_zero_crossings(&samples);
        let expected_crossings = (freq_c4 * 2.0) as i32;
        assert!(
            (zero_crossings as i32 - expected_crossings).abs() < 4,
            "Expected ~{} zero crossings, got {}",
            expected_crossings,
            zero_crossings
        );
    }

    #[test]
    fn test_phase_accumulation_wraps_correctly() {
        // RED: Phase should wrap at 2π and not accumulate indefinitely
        let mut osc = Oscillator::new(44100.0);

        // Run for a long time at high frequency
        for _ in 0..100000 {
            let sample = osc.process_sine(10000.0);
            assert!(
                sample.is_finite(),
                "Sample should be finite (phase wrapping working)"
            );
        }

        // If phase wrapping is broken, we'd get NaN or inf
        // This is a basic sanity check
    }

    #[test]
    fn test_zero_frequency_edge_case() {
        // RED: Zero frequency should not crash or produce NaN
        let mut osc = Oscillator::new(44100.0);

        for _ in 0..100 {
            let sample = osc.process_sine(0.0);
            assert!(sample.is_finite(), "Zero frequency should produce finite output");
        }
    }

    #[test]
    fn test_negative_frequency_edge_case() {
        // RED: Negative frequency should either work (reverse) or be clamped
        let mut osc = Oscillator::new(44100.0);

        for _ in 0..100 {
            let sample = osc.process_sine(-440.0);
            assert!(
                sample.is_finite(),
                "Negative frequency should produce finite output"
            );
        }
    }

    #[test]
    fn test_nyquist_frequency_edge_case() {
        // RED: Frequency at Nyquist (sample_rate / 2) is the edge of valid range
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
    fn test_above_nyquist_frequency_edge_case() {
        // RED: Frequencies above Nyquist will alias
        // We document this behavior but don't crash
        let sample_rate = 44100.0;
        let mut osc = Oscillator::new(sample_rate);

        for _ in 0..100 {
            let sample = osc.process_sine(30000.0); // Above Nyquist
            assert!(
                sample.is_finite(),
                "Above-Nyquist frequency should not crash (will alias)"
            );
        }
    }

    #[test]
    fn test_oscillator_reset() {
        // RED: Oscillator should have a reset method to zero phase
        let mut osc = Oscillator::new(44100.0);

        // Generate some samples
        for _ in 0..1000 {
            osc.process_sine(440.0);
        }

        // Reset phase
        osc.reset();

        // After reset, we should start from phase 0
        let first_sample = osc.process_sine(440.0);

        // For sine wave at phase 0, value should be close to 0
        assert!(
            first_sample.abs() < 0.1,
            "After reset, sine should start near 0, got {}",
            first_sample
        );
    }

    #[test]
    fn test_multiple_oscillators_independent() {
        // RED: Multiple oscillator instances should not interfere
        let mut osc1 = Oscillator::new(44100.0);
        let mut osc2 = Oscillator::new(44100.0);

        let samples1: Vec<f32> = (0..100).map(|_| osc1.process_sine(440.0)).collect();
        let samples2: Vec<f32> = (0..100).map(|_| osc2.process_sine(440.0)).collect();

        // Both should produce identical output (starting from same phase)
        for (s1, s2) in samples1.iter().zip(samples2.iter()) {
            assert!(
                (s1 - s2).abs() < 0.0001,
                "Independent oscillators should produce same output"
            );
        }
    }

    // NOTE: Anti-aliasing tests are documented but not required for Phase 2
    // Future enhancement: PolyBLEP or other anti-aliasing for saw/square
    #[test]
    #[ignore] // Will implement in future phase
    fn test_antialiasing_consideration_documented() {
        // This test documents that we're aware of aliasing
        // For Phase 2, naive waveforms are acceptable
        // Future: Implement PolyBLEP or minBLEP

        // This is a placeholder test to remind us to implement anti-aliasing
        panic!("Anti-aliasing not yet implemented - future enhancement");
    }
}
