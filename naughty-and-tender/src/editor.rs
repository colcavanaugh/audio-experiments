//! Editor/GUI for Naughty and Tender
//!
//! This module provides the plugin's user interface using egui via nih-plug.

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
        |_, ()| {},
        move |egui_ctx, setter, _state| {
            egui::CentralPanel::default().show(egui_ctx, |ui| {
                ui.heading("Naughty and Tender");
                ui.add_space(10.0);

                ui.label("MIDI Synthesizer - Phase 2: Synthesis Active!");
                ui.add_space(20.0);

                // Oscillator section
                ui.group(|ui| {
                    ui.heading("Oscillator");
                    ui.add_space(5.0);

                    ui.label("Waveform");
                    ui.add(widgets::ParamSlider::for_param(&params.waveform, setter));
                });

                ui.add_space(15.0);

                // ADSR Envelope section
                ui.group(|ui| {
                    ui.heading("Envelope (ADSR)");
                    ui.add_space(5.0);

                    ui.label("Attack");
                    ui.add(widgets::ParamSlider::for_param(&params.attack_ms, setter));

                    ui.add_space(5.0);

                    ui.label("Decay");
                    ui.add(widgets::ParamSlider::for_param(&params.decay_ms, setter));

                    ui.add_space(5.0);

                    ui.label("Sustain");
                    ui.add(widgets::ParamSlider::for_param(&params.sustain_level, setter));

                    ui.add_space(5.0);

                    ui.label("Release");
                    ui.add(widgets::ParamSlider::for_param(&params.release_ms, setter));
                });

                ui.add_space(15.0);

                // Master section
                ui.group(|ui| {
                    ui.heading("Master");
                    ui.add_space(5.0);

                    ui.label("Gain");
                    ui.add(widgets::ParamSlider::for_param(&params.gain, setter));

                    ui.add_space(5.0);

                    ui.label("Active Voices");
                    ui.add(widgets::ParamSlider::for_param(
                        &params.voice_count,
                        setter,
                    ));
                });

                ui.add_space(15.0);

                // Status information
                ui.group(|ui| {
                    ui.label("Status");
                    ui.add_space(5.0);

                    ui.label("✅ Plugin loaded successfully");
                    ui.label("✅ MIDI synthesis active");
                    ui.label("✅ Polyphonic voice management (16 voices)");
                    ui.label("✅ 4 waveforms available");
                    ui.label("✅ Full ADSR envelope control");
                });
            });
        },
    )
}
