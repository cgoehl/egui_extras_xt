use std::f32::consts::TAU;

use eframe::egui::{self, global_dark_light_mode_switch};
use itertools::Itertools;

mod common;
use common::{KnobDirection, KnobMode, KnobOrientation};

mod angle_knob;
use angle_knob::AngleKnob;

mod audio_knob;
use audio_knob::{AudioKnob, AudioKnobShape};

mod compass_knob;
use compass_knob::CompassKnob;

struct MyApp {
    // Common properties
    common_orientation: KnobOrientation,
    common_direction: KnobDirection,
    common_mode: KnobMode,
    common_snap_angle: Option<f32>,
    common_shift_snap_angle: Option<f32>,
    common_minimum_angle: Option<f32>,
    common_maximum_angle: Option<f32>,

    // AngleKnob
    angle_knob_value: f32,

    // AudioKnob
    audio_knob_value: f32,
    audio_knob_spread: f32,
    audio_knob_thickness: f32,

    // CompassKnob
    compass_knob_value: f32,
    compass_knob_spread: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            // Common properties
            common_orientation: KnobOrientation::Top,
            common_direction: KnobDirection::Clockwise,
            common_mode: KnobMode::Signed,
            common_snap_angle: None,
            common_shift_snap_angle: None,
            common_minimum_angle: None,
            common_maximum_angle: None,

            // AngleKnob
            angle_knob_value: TAU / 18.0,

            // AudioKnob
            audio_knob_value: 0.75,
            audio_knob_spread: 1.0,
            audio_knob_thickness: 0.66,

            // CompassKnob
            compass_knob_value: 0.0,
            compass_knob_spread: TAU / 2.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                global_dark_light_mode_switch(ui);
                ui.heading("Knobs");
            });

            ui.separator();

            ui.heading("Common properties");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.common_orientation, KnobOrientation::Top, "⬆ Top");
                ui.selectable_value(
                    &mut self.common_orientation,
                    KnobOrientation::Right,
                    "➡ Right",
                );
                ui.selectable_value(
                    &mut self.common_orientation,
                    KnobOrientation::Bottom,
                    "⬇ Bottom",
                );
                ui.selectable_value(
                    &mut self.common_orientation,
                    KnobOrientation::Left,
                    "⬅ Left",
                );

                {
                    let mut is_custom_orientation =
                        matches!(self.common_orientation, KnobOrientation::Custom(..));

                    ui.selectable_value(&mut is_custom_orientation, true, "✏ Custom(..)");

                    if is_custom_orientation
                        && !matches!(self.common_orientation, KnobOrientation::Custom(..))
                    {
                        self.common_orientation = KnobOrientation::Custom(0.0);
                    }

                    if let KnobOrientation::Custom(value) = &mut self.common_orientation {
                        ui.drag_angle(value);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.selectable_value(
                    &mut self.common_direction,
                    KnobDirection::Clockwise,
                    "⟳ Clockwise",
                );
                ui.selectable_value(
                    &mut self.common_direction,
                    KnobDirection::Counterclockwise,
                    "⟲ Counterclockwise",
                );
            });

            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.common_mode, KnobMode::Signed, "± Signed");

                ui.selectable_value(&mut self.common_mode, KnobMode::Unsigned, "+ Unsigned");

                ui.selectable_value(&mut self.common_mode, KnobMode::SpinAround, "🔃 SpinAround");
            });

            ui.horizontal(|ui| {
                {
                    let mut snap_enabled = self.common_snap_angle.is_some();
                    ui.toggle_value(&mut snap_enabled, "Snap angle");

                    self.common_snap_angle = match (snap_enabled, self.common_snap_angle) {
                        (true, None) => Some(TAU / 24.0),
                        (false, Some(_)) => None,
                        _ => self.common_snap_angle,
                    };

                    if let Some(value) = &mut self.common_snap_angle {
                        ui.drag_angle(value);
                        *value = value.max(TAU / 360.0);
                    }
                }

                {
                    let mut shift_snap_enabled = self.common_shift_snap_angle.is_some();
                    ui.toggle_value(&mut shift_snap_enabled, "Shift snap angle");

                    self.common_shift_snap_angle =
                        match (shift_snap_enabled, self.common_shift_snap_angle) {
                            (true, None) => Some(TAU / 24.0),
                            (false, Some(_)) => None,
                            _ => self.common_shift_snap_angle,
                        };

                    if let Some(value) = &mut self.common_shift_snap_angle {
                        ui.drag_angle(value);
                        *value = value.max(TAU / 360.0);
                    }
                }
            });

            ui.horizontal(|ui| {
                {
                    let mut minimum_enabled = self.common_minimum_angle.is_some();
                    ui.toggle_value(&mut minimum_enabled, "Minimum");

                    self.common_minimum_angle = match (minimum_enabled, self.common_minimum_angle) {
                        (true, None) => Some(-TAU),
                        (false, Some(_)) => None,
                        _ => self.common_minimum_angle,
                    };

                    if let Some(value) = &mut self.common_minimum_angle {
                        ui.drag_angle(value);
                    }
                }

                {
                    let mut maximum_enabled = self.common_maximum_angle.is_some();
                    ui.toggle_value(&mut maximum_enabled, "Maximum");

                    self.common_maximum_angle = match (maximum_enabled, self.common_maximum_angle) {
                        (true, None) => Some(TAU),
                        (false, Some(_)) => None,
                        _ => self.common_maximum_angle,
                    };

                    if let Some(value) = &mut self.common_maximum_angle {
                        ui.drag_angle(value);
                    }
                }
            });

            ui.add_space(8.0);
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("AudioKnob");
                ui.add_space(8.0);
                ui.add(egui::Slider::new(&mut self.audio_knob_value, -1.0..=1.0));
                ui.add(egui::Slider::new(&mut self.audio_knob_spread, 0.0..=1.0));
                ui.add(egui::Slider::new(&mut self.audio_knob_thickness, 0.0..=1.0));

                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    for (audio_knob_range, audio_knob_size) in [0.0..=1.0, -1.0..=1.0]
                        .into_iter()
                        .cartesian_product([64.0, 32.0])
                    {
                        ui.add(
                            AudioKnob::new(&mut self.audio_knob_value, audio_knob_range)
                                .diameter(audio_knob_size)
                                .orientation(self.common_orientation)
                                .direction(self.common_direction)
                                .spread(self.audio_knob_spread)
                                .thickness(self.audio_knob_thickness)
                                .shape(AudioKnobShape::Squircle(4.0)),
                        );
                    }
                });

                ui.add_space(8.0);
                ui.separator();

                ui.heading("AngleKnob");
                ui.add_space(8.0);

                ui.drag_angle(&mut self.angle_knob_value);
                ui.add_space(8.0);

                ui.horizontal(|ui| {
                    for angle_knob_size in [64.0, 32.0] {
                        ui.add(
                            AngleKnob::new(&mut self.angle_knob_value)
                                .diameter(angle_knob_size)
                                .orientation(self.common_orientation)
                                .direction(self.common_direction)
                                .mode(self.common_mode)
                                .min(self.common_minimum_angle)
                                .max(self.common_maximum_angle)
                                .snap_angle(self.common_snap_angle)
                                .shift_snap_angle(self.common_shift_snap_angle),
                        );
                    }
                });

                ui.add_space(8.0);
                ui.separator();

                ui.heading("CompassKnob");
                ui.add_space(8.0);

                ui.drag_angle(&mut self.compass_knob_value);
                ui.drag_angle(&mut self.compass_knob_spread);
                ui.add_space(8.0);

                ui.add(
                    CompassKnob::new(&mut self.compass_knob_value)
                        .mode(self.common_mode)
                        .width(256.0)
                        .height(48.0)
                        .spread(self.compass_knob_spread)
                        .labels(["N", "E", "S", "W"])
                        .snap_angle(self.common_snap_angle)
                        .shift_snap_angle(self.common_shift_snap_angle)
                        .min(self.common_minimum_angle)
                        .max(self.common_maximum_angle)
                        .animated(true),
                );

                ui.add_space(8.0);
                ui.separator();
            });
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();

    eframe::run_native("Knobs", options, Box::new(|_cc| Box::new(MyApp::default())));
}
