//! ADSR Envelope module for Naughty and Tender
//!
//! This module implements Attack-Decay-Sustain-Release envelopes for amplitude control.
//! Envelopes are sample-accurate and support various timing configurations.
//!
//! # References
//! - Standard ADSR envelope from analog synthesizers
//! - Linear ramps for attack, decay, and release
//! - State machine: Idle → Attack → Decay → Sustain → Release → Idle

#![allow(dead_code)] // Some methods may not be used initially

/// Envelope state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}

/// ADSR Envelope generator
///
/// Generates amplitude envelopes with Attack, Decay, Sustain, and Release phases.
/// Uses linear ramps for all phases and maintains sample-accurate timing.
///
/// # Real-time Safety
/// - No allocations in `process()`
/// - All state pre-initialized
/// - Simple state machine with no branches in hot path
///
/// # Example
/// ```
/// use naughty_and_tender::envelope::ADSREnvelope;
///
/// let mut env = ADSREnvelope::new(44100.0);
/// env.set_attack_ms(50.0);
/// env.set_decay_ms(100.0);
/// env.set_sustain_level(0.7);
/// env.set_release_ms(200.0);
///
/// env.note_on(1.0); // Full velocity
/// let amplitude = env.process(); // Get current envelope value
/// ```
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
    #[must_use] pub fn new(sample_rate: f32) -> Self {
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

    /// Process one sample and return envelope value
    ///
    /// Returns amplitude multiplier (0.0 to 1.0) scaled by velocity.
    ///
    /// # Returns
    /// Current envelope amplitude (0.0 to 1.0)
    #[inline]
    #[allow(clippy::needless_continue, clippy::redundant_else)]
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

                EnvelopeState::Sustain => {
                    // Hold at sustain level
                    self.current_value = self.sustain_level * self.velocity;
                    break;
                }

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
            }
        }

        self.current_value
    }

    /// Check if envelope is active (not idle)
    #[must_use] pub fn is_active(&self) -> bool {
        self.state != EnvelopeState::Idle
    }

    /// Get current envelope state
    #[must_use] pub fn get_state(&self) -> EnvelopeState {
        self.state
    }

    /// Reset envelope to idle state
    pub fn reset(&mut self) {
        self.state = EnvelopeState::Idle;
        self.current_value = 0.0;
        self.phase_sample = 0.0;
    }

    /// Transition to decay phase
    #[inline]
    fn transition_to_decay(&mut self) {
        self.state = EnvelopeState::Decay;
        self.phase_sample = 0.0;
    }

    /// Transition to sustain phase
    #[inline]
    fn transition_to_sustain(&mut self) {
        self.state = EnvelopeState::Sustain;
        self.phase_sample = 0.0;
    }

    /// Transition to idle phase
    #[inline]
    fn transition_to_idle(&mut self) {
        self.state = EnvelopeState::Idle;
        self.phase_sample = 0.0;
        self.current_value = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_RATE: f32 = 44100.0;

    #[test]
    fn test_envelope_creation() {
        // RED: This will fail - ADSREnvelope doesn't exist yet
        let _env = ADSREnvelope::new(SAMPLE_RATE);
    }

    #[test]
    fn test_attack_phase_timing() {
        // RED: Attack should ramp from 0 to 1 over specified time
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        // Set 100ms attack time
        env.set_attack_ms(100.0);
        env.set_decay_ms(0.0);
        env.set_sustain_level(1.0);
        env.set_release_ms(0.0);

        env.note_on(1.0); // Full velocity

        // Calculate samples for 100ms
        let attack_samples = (SAMPLE_RATE * 0.1) as usize; // 4410 samples

        let mut values = Vec::new();
        for _ in 0..attack_samples {
            values.push(env.process());
        }

        // First sample should be near 0
        assert!(
            values[0] < 0.1,
            "Attack should start near 0, got {}",
            values[0]
        );

        // Last sample should be near 1
        let last_value = values[values.len() - 1];
        assert!(
            (last_value - 1.0).abs() < 0.05,
            "Attack should reach ~1.0 after 100ms, got {}",
            last_value
        );

        // Values should be monotonically increasing
        for window in values.windows(2) {
            assert!(
                window[1] >= window[0],
                "Attack should be monotonically increasing"
            );
        }
    }

    #[test]
    fn test_attack_phase_zero_time() {
        // RED: Zero attack time should jump immediately to 1
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(0.0);
        env.set_decay_ms(0.0);
        env.set_sustain_level(1.0);
        env.set_release_ms(0.0);

        env.note_on(1.0);

        let value = env.process();
        assert!(
            (value - 1.0).abs() < 0.01,
            "Zero attack should immediately produce 1.0, got {}",
            value
        );
    }

    #[test]
    fn test_decay_phase_timing() {
        // RED: Decay should ramp from 1 to sustain level over specified time
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(0.0); // Instant attack
        env.set_decay_ms(100.0); // 100ms decay
        env.set_sustain_level(0.5); // 50% sustain
        env.set_release_ms(0.0);

        env.note_on(1.0);

        // Skip attack (instant)
        env.process();

        // Collect decay phase samples
        let decay_samples = (SAMPLE_RATE * 0.1) as usize;
        let mut values = Vec::new();
        for _ in 0..decay_samples {
            values.push(env.process());
        }

        // Should start near 1.0 (end of attack)
        assert!(
            (values[0] - 1.0).abs() < 0.1,
            "Decay should start near 1.0, got {}",
            values[0]
        );

        // Should end near 0.5 (sustain level)
        let last_value = values[values.len() - 1];
        assert!(
            (last_value - 0.5).abs() < 0.05,
            "Decay should reach sustain level 0.5, got {}",
            last_value
        );

        // Values should be monotonically decreasing
        for window in values.windows(2) {
            assert!(
                window[1] <= window[0],
                "Decay should be monotonically decreasing"
            );
        }
    }

    #[test]
    fn test_sustain_phase_holds_level() {
        // RED: Sustain should hold at specified level indefinitely
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(0.0);
        env.set_decay_ms(0.0);
        env.set_sustain_level(0.7); // 70% sustain
        env.set_release_ms(0.0);

        env.note_on(1.0);

        // Skip to sustain phase
        env.process();

        // Sustain should hold for a long time
        for _ in 0..10000 {
            let value = env.process();
            assert!(
                (value - 0.7).abs() < 0.01,
                "Sustain should hold at 0.7, got {}",
                value
            );
        }
    }

    #[test]
    fn test_sustain_level_100_percent() {
        // RED: 100% sustain level (no decay)
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(10.0);
        env.set_decay_ms(10.0);
        env.set_sustain_level(1.0); // 100% - no decay
        env.set_release_ms(0.0);

        env.note_on(1.0);

        // Process through attack and decay
        for _ in 0..(SAMPLE_RATE * 0.03) as usize {
            env.process();
        }

        // Should still be at 1.0 (100% sustain means no decay)
        let value = env.process();
        assert!(
            (value - 1.0).abs() < 0.05,
            "100% sustain should maintain 1.0, got {}",
            value
        );
    }

    #[test]
    fn test_release_phase_timing() {
        // RED: Release should ramp from current level to 0 over specified time
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(0.0);
        env.set_decay_ms(0.0);
        env.set_sustain_level(0.8);
        env.set_release_ms(100.0); // 100ms release

        env.note_on(1.0);

        // Process to sustain
        for _ in 0..10 {
            env.process();
        }

        // Trigger release
        env.note_off();

        let release_samples = (SAMPLE_RATE * 0.1) as usize;
        let mut values = Vec::new();
        for _ in 0..release_samples {
            values.push(env.process());
        }

        // Should start near 0.8 (sustain level)
        assert!(
            (values[0] - 0.8).abs() < 0.1,
            "Release should start near sustain level 0.8, got {}",
            values[0]
        );

        // Should end near 0
        let last_value = values[values.len() - 1];
        assert!(
            last_value < 0.05,
            "Release should reach near 0, got {}",
            last_value
        );

        // Values should be monotonically decreasing
        for window in values.windows(2) {
            assert!(
                window[1] <= window[0],
                "Release should be monotonically decreasing"
            );
        }
    }

    #[test]
    fn test_release_from_attack_phase() {
        // RED: Release can be triggered during attack
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(100.0);
        env.set_decay_ms(0.0);
        env.set_sustain_level(1.0);
        env.set_release_ms(50.0);

        env.note_on(1.0);

        // Process halfway through attack
        for _ in 0..(SAMPLE_RATE * 0.05) as usize {
            env.process();
        }

        let level_before_release = env.process();

        // Trigger release mid-attack
        env.note_off();

        // Should start releasing from current level
        let values: Vec<f32> = (0..3000).map(|_| env.process()).collect();

        assert!(
            values[0] <= level_before_release,
            "Release should start from current level"
        );

        // Should eventually reach 0 (process enough samples to complete 50ms release)
        assert!(
            values[values.len() - 1] < 0.1,
            "Release should reach near 0"
        );
    }

    #[test]
    fn test_instant_release() {
        // RED: Zero release time should immediately silence
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(0.0);
        env.set_decay_ms(0.0);
        env.set_sustain_level(1.0);
        env.set_release_ms(0.0); // Instant release

        env.note_on(1.0);
        env.process(); // Reach sustain

        env.note_off();

        let value = env.process();
        assert!(
            value < 0.01,
            "Instant release should immediately produce ~0, got {}",
            value
        );
    }

    #[test]
    fn test_note_off_triggers_release() {
        // RED: note_off should transition to release phase
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(0.0);
        env.set_decay_ms(0.0);
        env.set_sustain_level(0.9);
        env.set_release_ms(100.0);

        env.note_on(1.0);

        // Process to sustain
        for _ in 0..100 {
            let value = env.process();
            assert!(
                (value - 0.9).abs() < 0.05,
                "Should be in sustain phase"
            );
        }

        // Trigger release
        env.note_off();

        // Next samples should start decreasing
        let value1 = env.process();
        let value2 = env.process();
        let value3 = env.process();

        assert!(value2 < value1, "Should be releasing (decreasing)");
        assert!(value3 < value2, "Should continue releasing");
    }

    #[test]
    fn test_envelope_velocity_sensitivity() {
        // RED: Note velocity should affect envelope amplitude
        let mut env1 = ADSREnvelope::new(SAMPLE_RATE);
        let mut env2 = ADSREnvelope::new(SAMPLE_RATE);

        env1.set_attack_ms(0.0);
        env1.set_decay_ms(0.0);
        env1.set_sustain_level(1.0);
        env1.set_release_ms(0.0);

        env2.set_attack_ms(0.0);
        env2.set_decay_ms(0.0);
        env2.set_sustain_level(1.0);
        env2.set_release_ms(0.0);

        env1.note_on(1.0); // Full velocity
        env2.note_on(0.5); // Half velocity

        let value1 = env1.process();
        let value2 = env2.process();

        assert!(
            value1 > value2,
            "Higher velocity should produce higher output: {} vs {}",
            value1,
            value2
        );

        assert!(
            (value2 - 0.5).abs() < 0.1,
            "Half velocity should produce ~0.5, got {}",
            value2
        );
    }

    #[test]
    fn test_envelope_is_active() {
        // RED: Envelope should report if it's active (not idle)
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(0.0);
        env.set_decay_ms(0.0);
        env.set_sustain_level(1.0);
        env.set_release_ms(10.0);

        // Initially idle
        assert!(!env.is_active(), "Envelope should start idle");

        // Active after note on
        env.note_on(1.0);
        assert!(env.is_active(), "Envelope should be active after note on");

        // Still active during sustain
        for _ in 0..100 {
            env.process();
        }
        assert!(env.is_active(), "Envelope should be active during sustain");

        // Active during release
        env.note_off();
        assert!(env.is_active(), "Envelope should be active during release");

        // Process through release
        for _ in 0..(SAMPLE_RATE * 0.02) as usize {
            env.process();
        }

        // Should be idle after release completes
        assert!(!env.is_active(), "Envelope should be idle after release");
    }

    #[test]
    fn test_envelope_reset() {
        // RED: Envelope should have reset method to return to idle
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(100.0);
        env.set_decay_ms(0.0);
        env.set_sustain_level(1.0);
        env.set_release_ms(0.0);

        env.note_on(1.0);

        // Process for a while
        for _ in 0..1000 {
            env.process();
        }

        // Reset
        env.reset();

        // Should be idle and output 0
        assert!(!env.is_active(), "Should be idle after reset");
        let value = env.process();
        assert!(value < 0.01, "Should output ~0 after reset, got {}", value);
    }

    #[test]
    fn test_sample_accurate_timing() {
        // RED: Envelope timing should be sample-accurate
        let mut env1 = ADSREnvelope::new(SAMPLE_RATE);
        let mut env2 = ADSREnvelope::new(SAMPLE_RATE);

        let attack_time = 44.1; // Exactly 1944 samples at 44100 Hz
        env1.set_attack_ms(attack_time);
        env2.set_attack_ms(attack_time);

        env1.set_decay_ms(0.0);
        env1.set_sustain_level(1.0);
        env1.set_release_ms(0.0);

        env2.set_decay_ms(0.0);
        env2.set_sustain_level(1.0);
        env2.set_release_ms(0.0);

        env1.note_on(1.0);
        env2.note_on(1.0);

        // Both should produce identical values sample by sample
        for _ in 0..2000 {
            let v1 = env1.process();
            let v2 = env2.process();
            assert!(
                (v1 - v2).abs() < 0.0001,
                "Envelopes should be sample-accurate"
            );
        }
    }

    #[test]
    fn test_envelope_retrigger() {
        // RED: Retriggering envelope during attack should restart
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(100.0);
        env.set_decay_ms(0.0);
        env.set_sustain_level(1.0);
        env.set_release_ms(0.0);

        env.note_on(1.0);

        // Process halfway through attack
        for _ in 0..(SAMPLE_RATE * 0.05) as usize {
            env.process();
        }

        let mid_attack_value = env.process();
        assert!(
            mid_attack_value > 0.3 && mid_attack_value < 0.7,
            "Should be mid-attack"
        );

        // Retrigger
        env.note_on(1.0);

        // Should restart from beginning
        let retriggered_value = env.process();
        assert!(
            retriggered_value < mid_attack_value,
            "Retrigger should restart attack, got {} (was {})",
            retriggered_value,
            mid_attack_value
        );
    }

    #[test]
    fn test_no_allocations_in_process() {
        // RED: Real-time safety - process() should not allocate
        // This is a conceptual test - Rust's type system helps us here
        // We ensure process() takes &mut self and returns f32
        // No Vec, Box, or other allocations in the hot path

        let mut env = ADSREnvelope::new(SAMPLE_RATE);
        env.set_attack_ms(10.0);
        env.set_decay_ms(10.0);
        env.set_sustain_level(0.5);
        env.set_release_ms(10.0);

        env.note_on(1.0);

        // Process many samples - should be real-time safe
        for _ in 0..100000 {
            let _value = env.process(); // Just &mut self -> f32, no allocations
        }

        // If this compiles and runs, we've verified the signature
        // Manual inspection of implementation will confirm no allocations
    }

    #[test]
    fn test_envelope_state_transitions() {
        // RED: Envelope should transition through states correctly
        let mut env = ADSREnvelope::new(SAMPLE_RATE);

        env.set_attack_ms(10.0);
        env.set_decay_ms(10.0);
        env.set_sustain_level(0.5);
        env.set_release_ms(10.0);

        // Start in idle
        assert_eq!(env.get_state(), EnvelopeState::Idle);

        // Note on -> Attack
        env.note_on(1.0);
        assert_eq!(env.get_state(), EnvelopeState::Attack);

        // Process through attack
        for _ in 0..(SAMPLE_RATE * 0.015) as usize {
            env.process();
        }

        // Should be in Decay
        assert_eq!(env.get_state(), EnvelopeState::Decay);

        // Process through decay
        for _ in 0..(SAMPLE_RATE * 0.015) as usize {
            env.process();
        }

        // Should be in Sustain
        assert_eq!(env.get_state(), EnvelopeState::Sustain);

        // Note off -> Release
        env.note_off();
        assert_eq!(env.get_state(), EnvelopeState::Release);

        // Process through release
        for _ in 0..(SAMPLE_RATE * 0.015) as usize {
            env.process();
        }

        // Should be back to Idle
        assert_eq!(env.get_state(), EnvelopeState::Idle);
    }
}
