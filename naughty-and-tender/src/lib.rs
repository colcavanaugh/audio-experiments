//! Naughty and Tender - A MIDI Synthesizer Plugin
//!
//! First audio plugin - designed for parameter exploration and hands-on learning.
//!
//! This plugin serves as both a foundation and a playground for synthesis techniques,
//! prioritizing understanding over preset perfection.

#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use nih_plug::prelude::*;
use std::sync::Arc;

mod editor;
mod params;

// Phase 2 modules - will be implemented to make tests pass
pub mod envelope;
pub mod oscillators;
pub mod voice;

use params::NaughtyAndTenderParams;
use voice::VoiceManager;

/// The main plugin struct
pub struct NaughtyAndTender {
    params: Arc<NaughtyAndTenderParams>,
    sample_rate: f32,
    voice_manager: Option<VoiceManager>,
}

impl Default for NaughtyAndTender {
    fn default() -> Self {
        Self {
            params: Arc::new(NaughtyAndTenderParams::default()),
            sample_rate: 44100.0,
            voice_manager: None, // Will be initialized in initialize()
        }
    }
}

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

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Initialize voice manager with 16 voices
        const NUM_VOICES: usize = 16;

        self.sample_rate = buffer_config.sample_rate;
        self.voice_manager = Some(VoiceManager::new(self.sample_rate, NUM_VOICES));

        nih_log!("Naughty and Tender initialized");
        nih_log!("Sample rate: {}", self.sample_rate);
        nih_log!("Max buffer size: {}", buffer_config.max_buffer_size);
        nih_log!("Voice manager initialized with {} voices", NUM_VOICES);

        true
    }

    fn reset(&mut self) {
        nih_log!("Plugin reset");

        // Reset voice manager
        if let Some(vm) = &mut self.voice_manager {
            vm.reset();
        }
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // Get voice manager (return if not initialized)
        let Some(voice_manager) = &mut self.voice_manager else {
            // Not initialized yet - output silence
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
        use oscillators::WaveformType;
        let waveform = match waveform_int {
            0 => WaveformType::Sine,
            1 => WaveformType::Sawtooth,
            2 => WaveformType::Square,
            3 => WaveformType::Triangle,
            _ => WaveformType::Sine, // Default fallback
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
                #[allow(clippy::cast_possible_truncation)] // Audio buffer size never exceeds u32
                if event.timing() > sample_idx as u32 {
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
                        // Convert velocity from 0-1 range
                        voice_manager.note_on(note, velocity);
                    }
                    NoteEvent::NoteOff {
                        timing: _,
                        voice_id: _,
                        channel: _,
                        note,
                        velocity: _,
                    } => {
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

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }
}

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
