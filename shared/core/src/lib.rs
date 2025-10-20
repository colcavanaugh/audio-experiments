//! Shared core utilities for audio DSP experiments
//!
//! This crate provides common utilities used across multiple plugin projects.

#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

/// Common audio constants
pub mod constants {
    /// Standard sample rates
    pub const SAMPLE_RATE_44100: f32 = 44100.0;
    pub const SAMPLE_RATE_48000: f32 = 48000.0;
    pub const SAMPLE_RATE_96000: f32 = 96000.0;

    /// MIDI constants
    pub const MIDI_NOTE_OFF: u8 = 0x80;
    pub const MIDI_NOTE_ON: u8 = 0x90;
    pub const MIDI_CC: u8 = 0xB0;

    /// MIDI note range
    pub const MIDI_MIN: u8 = 0;
    pub const MIDI_MAX: u8 = 127;
}

/// Utility functions for real-time safe operations
pub mod util {
    /// Convert MIDI note number to frequency in Hz
    /// Uses equal temperament tuning with A4 = 440 Hz
    #[inline]
    #[must_use]
    pub fn midi_note_to_freq(note: u8) -> f32 {
        // f = 440 * 2^((note - 69) / 12)
        const A4_FREQ: f32 = 440.0;
        const A4_NOTE: i32 = 69;

        A4_FREQ * 2.0_f32.powf((f32::from(note) - A4_NOTE as f32) / 12.0)
    }

    /// Clamp a value between min and max
    #[inline]
    #[must_use]
    pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_to_freq() {
        // A4 (MIDI 69) should be 440 Hz
        let freq = util::midi_note_to_freq(69);
        assert!((freq - 440.0).abs() < 0.01);

        // C4 (MIDI 60) should be ~261.63 Hz
        let freq = util::midi_note_to_freq(60);
        assert!((freq - 261.63).abs() < 0.1);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(util::clamp(5, 0, 10), 5);
        assert_eq!(util::clamp(-1, 0, 10), 0);
        assert_eq!(util::clamp(15, 0, 10), 10);
    }
}
