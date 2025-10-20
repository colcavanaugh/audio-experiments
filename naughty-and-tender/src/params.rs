//! Plugin parameters for Naughty and Tender
//!
//! This module defines all the plugin parameters that can be automated
//! and controlled by the host or GUI.

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

    // Oscillator parameters
    /// Waveform type (0=Sine, 1=Sawtooth, 2=Square, 3=Triangle)
    #[id = "waveform"]
    pub waveform: IntParam,

    // ADSR Envelope parameters
    /// Attack time in milliseconds
    #[id = "attack"]
    pub attack_ms: FloatParam,

    /// Decay time in milliseconds
    #[id = "decay"]
    pub decay_ms: FloatParam,

    /// Sustain level (0.0 - 1.0)
    #[id = "sustain"]
    pub sustain_level: FloatParam,

    /// Release time in milliseconds
    #[id = "release"]
    pub release_ms: FloatParam,
}

impl Default for NaughtyAndTenderParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(600, 500),

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

            // Oscillator parameters
            waveform: IntParam::new(
                "Waveform",
                0, // Default to Sine
                IntRange::Linear { min: 0, max: 3 },
            )
            .with_value_to_string(Arc::new(|value| {
                match value {
                    0 => "Sine".to_string(),
                    1 => "Sawtooth".to_string(),
                    2 => "Square".to_string(),
                    3 => "Triangle".to_string(),
                    _ => "Unknown".to_string(),
                }
            }))
            .with_string_to_value(Arc::new(|string| {
                match string {
                    "Sine" => Some(0),
                    "Sawtooth" => Some(1),
                    "Square" => Some(2),
                    "Triangle" => Some(3),
                    _ => None,
                }
            })),

            // ADSR Envelope parameters
            attack_ms: FloatParam::new(
                "Attack",
                10.0,
                FloatRange::Skewed {
                    min: 0.1,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit(" ms")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),

            decay_ms: FloatParam::new(
                "Decay",
                100.0,
                FloatRange::Skewed {
                    min: 0.1,
                    max: 2000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit(" ms")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),

            sustain_level: FloatParam::new(
                "Sustain",
                0.7,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit("")
            .with_value_to_string(formatters::v2s_f32_percentage(0))
            .with_string_to_value(formatters::s2v_f32_percentage()),

            release_ms: FloatParam::new(
                "Release",
                300.0,
                FloatRange::Skewed {
                    min: 0.1,
                    max: 5000.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit(" ms")
            .with_value_to_string(formatters::v2s_f32_rounded(1)),
        }
    }
}
