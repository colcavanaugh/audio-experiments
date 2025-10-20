//! Integration tests for Naughty and Tender Phase 2
//!
//! These tests verify that oscillators, envelopes, and voices work together correctly
//! to produce actual synthesizer behavior.

#[cfg(test)]
mod integration_tests {
    // These tests require all Phase 2 components to be implemented
    // Currently they will fail because modules don't exist yet

    const SAMPLE_RATE: f32 = 44100.0;

    #[test]
    fn test_midi_note_on_produces_audio() {
        // RED: MIDI note on should trigger voice and produce audio
        use naughty_and_tender::voice::VoiceManager;

        let mut vm = VoiceManager::new(SAMPLE_RATE, 16);

        // Trigger note
        vm.note_on(60, 1.0); // Middle C

        // Process a buffer
        let mut buffer = vec![0.0; 256];
        vm.process(&mut buffer);

        // Should produce audio (after attack phase)
        let has_audio = buffer.iter().any(|&s| s.abs() > 0.01);
        assert!(has_audio, "MIDI note on should produce audio");
    }

    #[test]
    fn test_midi_note_off_triggers_release() {
        // RED: MIDI note off should trigger envelope release
        use naughty_and_tender::voice::VoiceManager;

        let mut vm = VoiceManager::new(SAMPLE_RATE, 16);

        vm.note_on(60, 1.0);

        // Let it sustain
        for _ in 0..10 {
            let mut buffer = vec![0.0; 128];
            vm.process(&mut buffer);
        }

        // Trigger release
        vm.note_off(60);

        // Collect audio during release
        let mut release_samples = Vec::new();
        for _ in 0..50 {
            let mut buffer = vec![0.0; 128];
            vm.process(&mut buffer);
            release_samples.extend_from_slice(&buffer);
        }

        // Audio should gradually decrease
        let first_half_rms = calculate_rms(&release_samples[0..1000]);
        let second_half_rms =
            calculate_rms(&release_samples[release_samples.len() - 1000..]);

        assert!(
            first_half_rms > second_half_rms,
            "Audio should decay during release: {} vs {}",
            first_half_rms,
            second_half_rms
        );
    }

    #[test]
    fn test_polyphony_produces_mixed_output() {
        // RED: Multiple simultaneous notes should mix correctly
        use naughty_and_tender::voice::VoiceManager;

        let mut vm = VoiceManager::new(SAMPLE_RATE, 16);

        // Play single note
        vm.note_on(60, 0.5);

        // Process past attack phase to reach stable amplitude
        let mut warmup = vec![0.0; 1024];
        vm.process(&mut warmup);

        let mut buffer_single = vec![0.0; 512];
        vm.process(&mut buffer_single);

        let rms_single = calculate_rms(&buffer_single);

        // Reset and play chord
        vm.reset();
        vm.note_on(60, 0.5); // C
        vm.note_on(64, 0.5); // E
        vm.note_on(67, 0.5); // G

        // Process past attack phase to reach stable amplitude
        let mut warmup_chord = vec![0.0; 1024];
        vm.process(&mut warmup_chord);

        let mut buffer_chord = vec![0.0; 512];
        vm.process(&mut buffer_chord);

        let rms_chord = calculate_rms(&buffer_chord);

        // Chord should be louder (more energy from multiple voices)
        // Note: Due to phase relationships, expect modest increase (not 3x)
        assert!(
            rms_chord > rms_single * 1.05,
            "Chord should be louder than single note: {} vs {}",
            rms_chord,
            rms_single
        );
    }

    #[test]
    fn test_different_notes_produce_different_frequencies() {
        // RED: Different MIDI notes should produce different frequencies
        use naughty_and_tender::voice::VoiceManager;

        let mut vm1 = VoiceManager::new(SAMPLE_RATE, 16);
        let mut vm2 = VoiceManager::new(SAMPLE_RATE, 16);

        // Play C4
        vm1.note_on(60, 1.0);
        let mut samples_c4 = vec![0.0; 44100];
        vm1.process(&mut samples_c4);

        // Play C5 (octave higher)
        vm2.note_on(72, 1.0);
        let mut samples_c5 = vec![0.0; 44100];
        vm2.process(&mut samples_c5);

        // Count zero crossings
        let crossings_c4 = count_zero_crossings(&samples_c4);
        let crossings_c5 = count_zero_crossings(&samples_c5);

        // C5 should have twice as many zero crossings as C4 (octave = 2x frequency)
        let ratio = crossings_c5 as f32 / crossings_c4 as f32;
        assert!(
            (ratio - 2.0).abs() < 0.1,
            "C5 should be octave above C4, ratio: {}",
            ratio
        );
    }

    #[test]
    fn test_velocity_affects_output_amplitude() {
        // RED: Velocity should control note loudness
        use naughty_and_tender::voice::VoiceManager;

        let mut vm_loud = VoiceManager::new(SAMPLE_RATE, 16);
        let mut vm_soft = VoiceManager::new(SAMPLE_RATE, 16);

        vm_loud.note_on(60, 1.0); // Full velocity
        vm_soft.note_on(60, 0.5); // Half velocity

        // Process past attack phase
        for _ in 0..10 {
            let mut buf1 = vec![0.0; 128];
            let mut buf2 = vec![0.0; 128];
            vm_loud.process(&mut buf1);
            vm_soft.process(&mut buf2);
        }

        let mut buffer_loud = vec![0.0; 256];
        let mut buffer_soft = vec![0.0; 256];
        vm_loud.process(&mut buffer_loud);
        vm_soft.process(&mut buffer_soft);

        let rms_loud = calculate_rms(&buffer_loud);
        let rms_soft = calculate_rms(&buffer_soft);

        assert!(
            rms_loud > rms_soft * 1.3,
            "Higher velocity should be louder: {} vs {}",
            rms_loud,
            rms_soft
        );
    }

    #[test]
    fn test_audio_returns_to_silence_after_release() {
        // RED: After release completes, output should be silent
        use naughty_and_tender::voice::VoiceManager;

        let mut vm = VoiceManager::new(SAMPLE_RATE, 16);

        vm.note_on(60, 1.0);
        vm.note_off(60);

        // Process through release (assuming release time < 1 second)
        for _ in 0..100 {
            let mut buffer = vec![0.0; 128];
            vm.process(&mut buffer);
        }

        // Should be silent now
        let mut buffer = vec![0.0; 256];
        vm.process(&mut buffer);

        let rms = calculate_rms(&buffer);
        assert!(rms < 0.001, "Should be silent after release, got RMS {}", rms);
    }

    #[test]
    fn test_voice_stealing_maintains_polyphony_limit() {
        // RED: Voice stealing should prevent exceeding max voices
        use naughty_and_tender::voice::VoiceManager;

        let max_voices = 4;
        let mut vm = VoiceManager::new(SAMPLE_RATE, max_voices);

        // Play more notes than max voices
        for note in 60..70 {
            vm.note_on(note, 1.0);

            // Process a bit
            let mut buffer = vec![0.0; 64];
            vm.process(&mut buffer);
        }

        // Should never exceed max voices
        assert!(
            vm.active_voice_count() <= max_voices,
            "Should not exceed {} voices",
            max_voices
        );

        // Should still produce audio
        let mut buffer = vec![0.0; 256];
        vm.process(&mut buffer);

        let has_audio = buffer.iter().any(|&s| s.abs() > 0.01);
        assert!(has_audio, "Should still produce audio after voice stealing");
    }

    #[test]
    fn test_envelope_shapes_amplitude_over_time() {
        // RED: Envelope should control amplitude through ADSR phases
        use naughty_and_tender::voice::Voice;

        let mut voice = Voice::new(SAMPLE_RATE);

        // Set specific envelope times
        // attack: 50ms, decay: 50ms, sustain: 50%, release: 50ms
        voice.set_envelope_attack_ms(50.0);
        voice.set_envelope_decay_ms(50.0);
        voice.set_envelope_sustain_level(0.5);
        voice.set_envelope_release_ms(50.0);

        voice.note_on(60, 1.0);

        // Attack phase (0 to 50ms): should increase in amplitude (use abs for magnitude)
        let attack_start = voice.process().abs();
        for _ in 0..(SAMPLE_RATE * 0.025) as usize {
            voice.process();
        }
        let attack_mid = voice.process().abs();
        for _ in 0..(SAMPLE_RATE * 0.025) as usize {
            voice.process();
        }
        let attack_end = voice.process().abs();

        assert!(attack_mid > attack_start, "Attack amplitude should increase");
        assert!(attack_end > attack_mid, "Attack amplitude should continue increasing");

        // Decay phase (50 to 100ms): should decrease to sustain level
        for _ in 0..(SAMPLE_RATE * 0.05) as usize {
            voice.process();
        }
        let decay_end = voice.process().abs();

        // Should be close to sustain level * peak
        assert!(
            (decay_end - 0.5).abs() < 0.2,
            "Should reach sustain level ~0.5, got {}",
            decay_end
        );

        // Sustain phase: should hold steady (check RMS to account for oscillation)
        let sustain_samples1: Vec<f32> = (0..100).map(|_| voice.process()).collect();
        let sustain_rms1 = calculate_rms(&sustain_samples1);

        for _ in 0..1000 {
            voice.process();
        }

        let sustain_samples2: Vec<f32> = (0..100).map(|_| voice.process()).collect();
        let sustain_rms2 = calculate_rms(&sustain_samples2);

        assert!(
            (sustain_rms1 - sustain_rms2).abs() < 0.05,
            "Sustain RMS should hold steady: {} vs {}",
            sustain_rms1,
            sustain_rms2
        );
    }

    #[test]
    fn test_oscillator_waveform_selection() {
        // RED: Should be able to select different waveforms
        use naughty_and_tender::voice::Voice;
        use naughty_and_tender::oscillators::WaveformType;

        let mut voice_sine = Voice::new(SAMPLE_RATE);
        let mut voice_saw = Voice::new(SAMPLE_RATE);

        voice_sine.set_waveform(WaveformType::Sine);
        voice_saw.set_waveform(WaveformType::Sawtooth);

        voice_sine.note_on(60, 1.0);
        voice_saw.note_on(60, 1.0);

        // Process samples
        let samples_sine: Vec<f32> = (0..1000).map(|_| voice_sine.process()).collect();
        let samples_saw: Vec<f32> = (0..1000).map(|_| voice_saw.process()).collect();

        // Waveforms should be different
        let diff: f32 = samples_sine
            .iter()
            .zip(samples_saw.iter())
            .map(|(s1, s2)| (s1 - s2).abs())
            .sum();

        assert!(
            diff > 100.0,
            "Different waveforms should produce different output"
        );
    }

    #[test]
    fn test_no_audio_glitches_on_parameter_changes() {
        // RED: Changing parameters shouldn't cause clicks or glitches
        use naughty_and_tender::voice::Voice;

        let mut voice = Voice::new(SAMPLE_RATE);
        voice.note_on(60, 1.0);

        // Process some samples
        let mut samples = Vec::new();
        for _ in 0..100 {
            samples.push(voice.process());
        }

        // Change envelope parameters mid-note
        voice.set_envelope_decay_ms(200.0);
        voice.set_envelope_sustain_level(0.3);

        // Continue processing
        for _ in 0..100 {
            samples.push(voice.process());
        }

        // Check for discontinuities (clicks)
        let max_jump = samples
            .windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .fold(0.0f32, f32::max);

        // No single-sample jump should be > 0.5 (arbitrary threshold for "click")
        assert!(
            max_jump < 0.5,
            "Parameter changes should not cause clicks, max jump: {}",
            max_jump
        );
    }

    #[test]
    fn test_real_time_safety_no_panics() {
        // RED: Audio processing should never panic
        use naughty_and_tender::voice::VoiceManager;

        let mut vm = VoiceManager::new(SAMPLE_RATE, 16);

        // Stress test with random-ish events
        for i in 0..1000 {
            let note = 48 + (i % 36) as u8; // Wide note range

            if i % 3 == 0 {
                vm.note_on(note, (i % 10) as f32 / 10.0);
            }
            if i % 5 == 0 {
                vm.note_off(note);
            }

            let mut buffer = vec![0.0; 128];
            vm.process(&mut buffer);

            // Verify all samples are finite
            for &sample in &buffer {
                assert!(
                    sample.is_finite(),
                    "All samples should be finite (no NaN/inf)"
                );
            }
        }
    }

    // Helper functions
    fn calculate_rms(samples: &[f32]) -> f32 {
        let sum_squares: f32 = samples.iter().map(|s| s * s).sum();
        (sum_squares / samples.len() as f32).sqrt()
    }

    fn count_zero_crossings(samples: &[f32]) -> usize {
        samples
            .windows(2)
            .filter(|w| (w[0] < 0.0 && w[1] >= 0.0) || (w[0] >= 0.0 && w[1] < 0.0))
            .count()
    }
}
