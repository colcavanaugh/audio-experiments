//! Voice management module for Naughty and Tender
//!
//! This module handles polyphonic voice allocation, voice stealing, and state management.
//! Each voice contains its own oscillator and envelope for independent note playback.
//!
//! # References
//! - Voice stealing: Steal oldest active voice or releasing voice first
//! - MIDI note to frequency: f = 440 * 2^((note - 69) / 12)

#![allow(dead_code)] // Some methods may not be used initially

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

impl Voice {
    /// Create a new voice
    #[must_use] pub fn new(sample_rate: f32) -> Self {
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

    /// Get voice state
    #[must_use] pub fn get_state(&self) -> VoiceState {
        self.state
    }

    /// Get MIDI note number
    #[must_use] pub fn get_note(&self) -> u8 {
        self.note
    }

    /// Get voice age
    #[must_use] pub fn get_age(&self) -> u64 {
        self.age
    }

    /// Set voice age (for voice stealing)
    pub fn set_age(&mut self, age: u64) {
        self.age = age;
    }

    /// Set waveform type
    pub fn set_waveform(&mut self, waveform: WaveformType) {
        self.waveform = waveform;
    }

    /// Set envelope attack time
    pub fn set_envelope_attack_ms(&mut self, attack_ms: f32) {
        self.envelope.set_attack_ms(attack_ms);
    }

    /// Set envelope decay time
    pub fn set_envelope_decay_ms(&mut self, decay_ms: f32) {
        self.envelope.set_decay_ms(decay_ms);
    }

    /// Set envelope sustain level
    pub fn set_envelope_sustain_level(&mut self, sustain_level: f32) {
        self.envelope.set_sustain_level(sustain_level);
    }

    /// Set envelope release time
    pub fn set_envelope_release_ms(&mut self, release_ms: f32) {
        self.envelope.set_release_ms(release_ms);
    }

    /// Reset voice to idle state
    pub fn reset(&mut self) {
        self.state = VoiceState::Idle;
        self.envelope.reset();
        self.oscillator.reset();
    }
}

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

impl VoiceManager {
    /// Create a new voice manager
    ///
    /// # Arguments
    /// * `sample_rate` - Sample rate in Hz
    /// * `max_voices` - Maximum number of simultaneous voices
    #[must_use] pub fn new(sample_rate: f32, max_voices: usize) -> Self {
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

    /// Process audio for all voices and fill buffer
    ///
    /// Mixes all active voices into the output buffer.
    ///
    /// # Arguments
    /// * `buffer` - Output buffer to fill (mono)
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

    /// Get number of active voices (not idle)
    #[must_use] pub fn active_voice_count(&self) -> usize {
        self.voices
            .iter()
            .filter(|v| v.get_state() != VoiceState::Idle)
            .count()
    }

    /// Get number of releasing voices
    #[must_use] pub fn releasing_voice_count(&self) -> usize {
        self.voices
            .iter()
            .filter(|v| v.get_state() == VoiceState::Releasing)
            .count()
    }

    /// Get list of active note numbers
    #[must_use] pub fn get_active_notes(&self) -> Vec<u8> {
        self.voices
            .iter()
            .filter(|v| v.get_state() == VoiceState::Active)
            .map(Voice::get_note)
            .collect()
    }

    /// Get voice states (for testing)
    #[must_use] pub fn get_voice_states(&self) -> Vec<VoiceState> {
        self.voices.iter().map(Voice::get_state).collect()
    }

    /// Get maximum voice count
    #[must_use] pub fn max_voice_count(&self) -> usize {
        self.max_voices
    }

    /// Reset all voices
    pub fn reset(&mut self) {
        for voice in &mut self.voices {
            voice.reset();
        }
    }

    /// Update waveform type for all voices
    pub fn set_waveform(&mut self, waveform: WaveformType) {
        for voice in &mut self.voices {
            voice.set_waveform(waveform);
        }
    }

    /// Update attack time for all voices
    pub fn set_attack_ms(&mut self, attack_ms: f32) {
        for voice in &mut self.voices {
            voice.set_envelope_attack_ms(attack_ms);
        }
    }

    /// Update decay time for all voices
    pub fn set_decay_ms(&mut self, decay_ms: f32) {
        for voice in &mut self.voices {
            voice.set_envelope_decay_ms(decay_ms);
        }
    }

    /// Update sustain level for all voices
    pub fn set_sustain_level(&mut self, sustain_level: f32) {
        for voice in &mut self.voices {
            voice.set_envelope_sustain_level(sustain_level);
        }
    }

    /// Update release time for all voices
    pub fn set_release_ms(&mut self, release_ms: f32) {
        for voice in &mut self.voices {
            voice.set_envelope_release_ms(release_ms);
        }
    }

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
                && (oldest_releasing.is_none() || voice.get_age() < oldest_releasing_age) {
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
}

/// Convert MIDI note number to frequency in Hz
///
/// Uses standard MIDI tuning: A4 (note 69) = 440 Hz
///
/// # Arguments
/// * `note` - MIDI note number (0-127)
///
/// # Returns
/// Frequency in Hz
#[inline]
#[must_use] pub fn midi_note_to_frequency(note: u8) -> f32 {
    440.0 * 2.0f32.powf((f32::from(note) - 69.0) / 12.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: f32 = 44100.0;
    const MAX_VOICES: usize = 16;

    #[test]
    fn test_voice_creation() {
        // RED: This will fail - Voice doesn't exist yet
        let _voice = Voice::new(SAMPLE_RATE);
    }

    #[test]
    fn test_voice_manager_creation() {
        // RED: VoiceManager with configurable polyphony
        let _voice_manager = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);
    }

    #[test]
    fn test_voice_allocation_on_note_on() {
        // RED: note_on should allocate a voice
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        let note = 60; // C4
        let velocity = 1.0;

        vm.note_on(note, velocity);

        // Should have one active voice
        assert_eq!(vm.active_voice_count(), 1, "Should have 1 active voice");
    }

    #[test]
    fn test_voice_deallocation_on_note_off() {
        // RED: note_off should trigger release, eventually deallocating
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0);
        assert_eq!(vm.active_voice_count(), 1);

        vm.note_off(60);

        // Voice should be in releasing state
        let releasing_count = vm.releasing_voice_count();
        assert_eq!(releasing_count, 1, "Should have 1 releasing voice");

        // Process audio until envelope completes (assuming short release)
        for _ in 0..(SAMPLE_RATE * 0.5) as usize {
            let mut buffer = vec![0.0; 128];
            vm.process(&mut buffer);
        }

        // After release completes, voice should be idle
        assert_eq!(
            vm.active_voice_count(),
            0,
            "Voice should be idle after release"
        );
    }

    #[test]
    fn test_polyphony_multiple_notes() {
        // RED: Multiple simultaneous notes should work
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        // Play a chord: C, E, G
        vm.note_on(60, 1.0); // C
        vm.note_on(64, 1.0); // E
        vm.note_on(67, 1.0); // G

        assert_eq!(vm.active_voice_count(), 3, "Should have 3 active voices");

        // Each voice should track its own note
        let notes = vm.get_active_notes();
        assert!(notes.contains(&60), "Should have note 60");
        assert!(notes.contains(&64), "Should have note 64");
        assert!(notes.contains(&67), "Should have note 67");
    }

    #[test]
    fn test_polyphony_limit() {
        // RED: Should enforce max voice limit
        let max_voices = 8;
        let mut vm = VoiceManager::new(SAMPLE_RATE, max_voices);

        // Try to allocate more voices than the limit
        for note in 60..80 {
            vm.note_on(note, 1.0);
        }

        // Should not exceed max voices
        assert!(
            vm.active_voice_count() <= max_voices,
            "Should not exceed max voices: {} <= {}",
            vm.active_voice_count(),
            max_voices
        );
    }

    #[test]
    fn test_voice_stealing_oldest_first() {
        // RED: When limit reached, steal oldest voice
        let max_voices = 4;
        let mut vm = VoiceManager::new(SAMPLE_RATE, max_voices);

        // Fill all voices
        vm.note_on(60, 1.0); // Oldest
        vm.note_on(62, 1.0);
        vm.note_on(64, 1.0);
        vm.note_on(65, 1.0);

        assert_eq!(vm.active_voice_count(), 4);

        // Add one more note - should steal oldest (60)
        vm.note_on(67, 1.0);

        assert_eq!(vm.active_voice_count(), 4, "Should still have 4 voices");

        let notes = vm.get_active_notes();
        assert!(!notes.contains(&60), "Note 60 should be stolen");
        assert!(notes.contains(&67), "Note 67 should be active");
        assert!(notes.contains(&62), "Note 62 should still be active");
    }

    #[test]
    fn test_voice_stealing_releasing_voice_first() {
        // RED: Prefer stealing releasing voices over active ones
        let max_voices = 4;
        let mut vm = VoiceManager::new(SAMPLE_RATE, max_voices);

        // Fill all voices
        vm.note_on(60, 1.0);
        vm.note_on(62, 1.0);
        vm.note_on(64, 1.0);
        vm.note_on(65, 1.0);

        // Release one voice
        vm.note_off(62);

        // Add new note - should steal the releasing voice (62) not oldest (60)
        vm.note_on(67, 1.0);

        let notes = vm.get_active_notes();
        assert!(notes.contains(&60), "Note 60 should still be active");
        assert!(!notes.contains(&62), "Note 62 (releasing) should be stolen");
        assert!(notes.contains(&67), "Note 67 should be active");
    }

    #[test]
    fn test_each_voice_tracks_own_note() {
        // RED: Each voice should track its MIDI note number
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0);
        vm.note_on(64, 1.0);
        vm.note_on(67, 1.0);

        let notes = vm.get_active_notes();
        assert_eq!(notes.len(), 3);
        assert!(notes.contains(&60));
        assert!(notes.contains(&64));
        assert!(notes.contains(&67));
    }

    #[test]
    fn test_each_voice_has_own_envelope_state() {
        // RED: Each voice has independent envelope
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0);
        vm.note_on(64, 1.0);

        // Release one note
        vm.note_off(60);

        // Voice 60 should be releasing, 64 should still be sustaining
        let states = vm.get_voice_states();

        // We expect one releasing, one active
        let releasing_count = states
            .iter()
            .filter(|s| matches!(s, VoiceState::Releasing))
            .count();
        let active_count = states
            .iter()
            .filter(|s| matches!(s, VoiceState::Active))
            .count();

        assert_eq!(releasing_count, 1, "Should have 1 releasing voice");
        assert_eq!(active_count, 1, "Should have 1 active voice");
    }

    #[test]
    fn test_each_voice_has_own_oscillator_phase() {
        // RED: Each voice maintains independent phase
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0);

        // Process some samples
        let mut buffer1 = vec![0.0; 100];
        vm.process(&mut buffer1);

        // Add another note
        vm.note_on(64, 1.0);

        // Process more samples
        let mut buffer2 = vec![0.0; 100];
        vm.process(&mut buffer2);

        // Both voices should be producing output
        // (This is a basic test - more detailed tests in integration)
        assert!(
            buffer2.iter().any(|&s| s.abs() > 0.01),
            "Should be producing audio"
        );
    }

    #[test]
    fn test_voice_state_machine() {
        // RED: Voice should transition Idle -> Active -> Releasing -> Idle
        let mut voice = Voice::new(SAMPLE_RATE);

        // Start idle
        assert_eq!(voice.get_state(), VoiceState::Idle);

        // Trigger note
        voice.note_on(60, 1.0);
        assert_eq!(voice.get_state(), VoiceState::Active);

        // Process some samples
        for _ in 0..1000 {
            voice.process();
        }
        assert_eq!(
            voice.get_state(),
            VoiceState::Active,
            "Should still be active"
        );

        // Release note
        voice.note_off();
        assert_eq!(voice.get_state(), VoiceState::Releasing);

        // Process through release (assuming short release time)
        for _ in 0..(SAMPLE_RATE * 0.2) as usize {
            voice.process();
        }

        // Should return to idle
        assert_eq!(voice.get_state(), VoiceState::Idle);
    }

    #[test]
    fn test_voice_generates_correct_frequency() {
        // RED: Voice should generate correct frequency for MIDI note
        let mut voice = Voice::new(SAMPLE_RATE);

        voice.note_on(69, 1.0); // A4 = 440 Hz

        // Generate 1 second of audio
        let samples: Vec<f32> = (0..44100).map(|_| voice.process()).collect();

        // Count zero crossings to verify frequency
        let zero_crossings = samples
            .windows(2)
            .filter(|w| (w[0] < 0.0 && w[1] >= 0.0) || (w[0] >= 0.0 && w[1] < 0.0))
            .count();

        // For 440 Hz, expect ~880 zero crossings (2 per cycle)
        assert!(
            (zero_crossings as i32 - 880).abs() < 10,
            "Expected ~880 zero crossings for A4, got {}",
            zero_crossings
        );
    }

    #[test]
    fn test_voice_respects_velocity() {
        // RED: Higher velocity should produce louder output
        let mut voice1 = Voice::new(SAMPLE_RATE);
        let mut voice2 = Voice::new(SAMPLE_RATE);

        voice1.note_on(60, 1.0); // Full velocity
        voice2.note_on(60, 0.5); // Half velocity

        // Process through attack to stable level
        for _ in 0..1000 {
            voice1.process();
            voice2.process();
        }

        let sample1 = voice1.process();
        let sample2 = voice2.process();

        assert!(
            sample1.abs() > sample2.abs(),
            "Higher velocity should be louder: {} vs {}",
            sample1,
            sample2
        );
    }

    #[test]
    fn test_voice_manager_process_produces_audio() {
        // RED: process() should fill buffer with audio
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0);

        let mut buffer = vec![0.0; 128];
        vm.process(&mut buffer);

        // Should have non-zero audio (after envelope attack)
        let max_amplitude = buffer.iter().map(|s| s.abs()).fold(0.0f32, f32::max);

        assert!(
            max_amplitude > 0.01,
            "Should produce audible output, got max {}",
            max_amplitude
        );
    }

    #[test]
    fn test_voice_manager_process_is_additive() {
        // RED: Multiple voices should mix additively
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0);
        vm.note_on(64, 1.0);

        let mut buffer = vec![0.0; 128];
        vm.process(&mut buffer);

        // Two voices should be louder than one
        // (Actual mixing test - voices should add)
        let rms: f32 = buffer.iter().map(|s| s * s).sum::<f32>() / buffer.len() as f32;
        assert!(rms > 0.001, "Two voices should produce audible mix");
    }

    #[test]
    fn test_voice_manager_silence_when_no_notes() {
        // RED: No active voices should produce silence
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        let mut buffer = vec![0.0; 128];
        vm.process(&mut buffer);

        // Should be silent
        let max_amplitude = buffer.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!(
            max_amplitude < 0.0001,
            "Should be silent with no notes, got {}",
            max_amplitude
        );
    }

    #[test]
    fn test_voice_manager_returns_to_silence() {
        // RED: After all notes released, should return to silence
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0);
        vm.note_off(60);

        // Process through release
        for _ in 0..100 {
            let mut buffer = vec![0.0; 128];
            vm.process(&mut buffer);
        }

        // Should be silent now
        let mut buffer = vec![0.0; 128];
        vm.process(&mut buffer);

        let max_amplitude = buffer.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!(
            max_amplitude < 0.001,
            "Should be silent after release, got {}",
            max_amplitude
        );
    }

    #[test]
    fn test_voice_reset() {
        // RED: Voice should have reset method
        let mut voice = Voice::new(SAMPLE_RATE);

        voice.note_on(60, 1.0);

        // Process some samples
        for _ in 0..1000 {
            voice.process();
        }

        // Reset
        voice.reset();

        // Should be idle and silent
        assert_eq!(voice.get_state(), VoiceState::Idle);
        let sample = voice.process();
        assert!(sample.abs() < 0.001, "Should be silent after reset");
    }

    #[test]
    fn test_voice_manager_reset_all_voices() {
        // RED: VoiceManager should reset all voices
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0);
        vm.note_on(64, 1.0);
        vm.note_on(67, 1.0);

        vm.reset();

        assert_eq!(vm.active_voice_count(), 0, "All voices should be idle");

        let mut buffer = vec![0.0; 128];
        vm.process(&mut buffer);

        let max_amplitude = buffer.iter().map(|s| s.abs()).fold(0.0f32, f32::max);
        assert!(max_amplitude < 0.001, "Should be silent after reset");
    }

    #[test]
    fn test_voice_pre_allocation_no_runtime_allocation() {
        // RED: Real-time safety - voices should be pre-allocated
        let vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        // Voices should be pre-allocated (fixed-size array)
        // This is verified by the signature and implementation
        // VoiceManager should use: Vec::with_capacity or fixed array

        assert_eq!(
            vm.max_voice_count(),
            MAX_VOICES,
            "Should pre-allocate max voices"
        );
    }

    #[test]
    fn test_process_no_allocations() {
        // RED: process() should not allocate in audio callback
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0);

        // Process many buffers - should be real-time safe
        for _ in 0..1000 {
            let mut buffer = vec![0.0; 128];
            vm.process(&mut buffer); // Should not allocate
        }

        // If this runs without performance issues, real-time safety is likely good
        // Manual code inspection will confirm no allocations in hot path
    }

    #[test]
    fn test_note_on_off_same_note_multiple_times() {
        // RED: Pressing same note multiple times should retrigger
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0);
        assert_eq!(vm.active_voice_count(), 1);

        vm.note_off(60);
        assert_eq!(vm.releasing_voice_count(), 1);

        // Press again before release completes
        vm.note_on(60, 1.0);

        // Should either reuse the releasing voice or allocate new one
        // Either way, we should have an active voice for note 60
        let notes = vm.get_active_notes();
        assert!(notes.contains(&60), "Note 60 should be active again");
    }

    #[test]
    fn test_voice_manager_handles_rapid_note_events() {
        // RED: Handle rapid MIDI events without issues
        let mut vm = VoiceManager::new(SAMPLE_RATE, 8);

        // Rapid note on/off events
        for i in 0..100 {
            let note = 60 + (i % 12) as u8;
            vm.note_on(note, 1.0);

            if i % 2 == 0 {
                vm.note_off(note);
            }
        }

        // Should not crash or exceed voice limit
        assert!(vm.active_voice_count() <= 8);

        // Should still produce audio
        let mut buffer = vec![0.0; 128];
        vm.process(&mut buffer);

        // Some voices should be active
        assert!(
            buffer.iter().any(|&s| s.abs() > 0.001),
            "Should have some active voices"
        );
    }

    #[test]
    fn test_polyphonic_note_off_releases_correct_voice() {
        // RED: note_off should release only the specified note
        let mut vm = VoiceManager::new(SAMPLE_RATE, MAX_VOICES);

        vm.note_on(60, 1.0); // C
        vm.note_on(64, 1.0); // E
        vm.note_on(67, 1.0); // G

        // Release E
        vm.note_off(64);

        let notes = vm.get_active_notes();

        // C and G should still be active (not releasing)
        assert!(notes.contains(&60), "C should still be active");
        assert!(notes.contains(&67), "G should still be active");

        // E should be releasing (not in active notes)
        assert!(!notes.contains(&64), "E should be releasing");
    }
}
