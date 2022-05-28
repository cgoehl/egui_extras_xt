use std::f32::consts::TAU;

use egui::{self, Response, Ui, Widget};
use emath::Vec2;
use epaint::{Shape, Stroke};

use crate::common::{normalized_angle_unsigned_excl, Orientation, WidgetShape, Winding, WrapMode};

// ----------------------------------------------------------------------------

/// Combined into one function (rather than two) to make it easier
/// for the borrow checker.
type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<f32>) -> f32>;

fn get(get_set_value: &mut GetSetValue<'_>) -> f32 {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: f32) {
    (get_set_value)(Some(value));
}

// ----------------------------------------------------------------------------

#[non_exhaustive]
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum AngleKnobPreset {
    AdobePhotoshop,
    AdobePremierePro,
    Gimp,
    GoogleChromeDevTools,
    Krita,
    LibreOffice,
    QtWidgets,
    // Software without knob widgets:
    // - Blender (no knobs but transform gizmo suggests Top/Clockwise/None)
    // - Inkscape
    // - Kdenlive
    // - MyPaint (no knobs but canvas rotation suggests Right/Clockwise/Signed)
}

impl AngleKnobPreset {
    fn properties(&self) -> (Orientation, Winding, WrapMode) {
        match *self {
            AngleKnobPreset::AdobePhotoshop => (
                Orientation::Right,
                Winding::Counterclockwise,
                WrapMode::Signed,
            ),
            AngleKnobPreset::AdobePremierePro => {
                (Orientation::Top, Winding::Clockwise, WrapMode::None)
            }
            AngleKnobPreset::Gimp => (
                Orientation::Right,
                Winding::Counterclockwise,
                WrapMode::Unsigned,
            ),
            AngleKnobPreset::GoogleChromeDevTools => {
                (Orientation::Top, Winding::Clockwise, WrapMode::Unsigned)
            }
            AngleKnobPreset::Krita => (
                Orientation::Right,
                Winding::Counterclockwise,
                WrapMode::Signed,
            ),
            AngleKnobPreset::LibreOffice => (
                Orientation::Right,
                Winding::Counterclockwise,
                WrapMode::Unsigned,
            ),
            AngleKnobPreset::QtWidgets => {
                (Orientation::Bottom, Winding::Clockwise, WrapMode::Unsigned)
            }
        }
    }
}

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct AngleKnob<'a> {
    get_set_value: GetSetValue<'a>,
    diameter: f32,
    orientation: Orientation,
    winding: Winding,
    shape: WidgetShape<'a>,
    wrap: WrapMode,
    min: Option<f32>,
    max: Option<f32>,
    snap: Option<f32>,
    shift_snap: Option<f32>,
    show_axes: bool,
    axis_count: usize,
}

impl<'a> AngleKnob<'a> {
    pub fn new(value: &'a mut f32) -> Self {
        Self::from_get_set(move |v: Option<f32>| {
            if let Some(v) = v {
                *value = v;
            }
            *value
        })
    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<f32>) -> f32) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
            diameter: 32.0,
            orientation: Orientation::Top,
            winding: Winding::Clockwise,
            shape: WidgetShape::Circle,
            wrap: WrapMode::Unsigned,
            min: None,
            max: None,
            snap: None,
            shift_snap: Some(TAU / 24.0),
            show_axes: true,
            axis_count: 4,
        }
    }

    pub fn preset(mut self, preset: AngleKnobPreset) -> Self {
        (self.orientation, self.winding, self.wrap) = preset.properties();
        self
    }

    pub fn diameter(mut self, diameter: impl Into<f32>) -> Self {
        self.diameter = diameter.into();
        self
    }

    pub fn winding(mut self, winding: Winding) -> Self {
        self.winding = winding;
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn shape(mut self, shape: WidgetShape<'a>) -> Self {
        self.shape = shape;
        self
    }

    pub fn wrap(mut self, wrap: WrapMode) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn min(mut self, min: Option<f32>) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: Option<f32>) -> Self {
        self.max = max;
        self
    }

    pub fn snap(mut self, snap: Option<f32>) -> Self {
        self.snap = snap;
        self
    }

    pub fn shift_snap(mut self, shift_snap: Option<f32>) -> Self {
        self.shift_snap = shift_snap;
        self
    }

    pub fn show_axes(mut self, show_axes: bool) -> Self {
        self.show_axes = show_axes;
        self
    }

    pub fn axis_count(mut self, axis_count: usize) -> Self {
        self.axis_count = axis_count.into();
        self
    }
}

impl<'a> Widget for AngleKnob<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.diameter);
        let (rect, mut response) =
            ui.allocate_exact_size(desired_size, egui::Sense::click_and_drag());

        let rotation_matrix = self.orientation.rot2();

        if response.clicked() || response.dragged() {
            let prev_value = get(&mut self.get_set_value);
            let mut new_value = (rotation_matrix.inverse()
                * (response.interact_pointer_pos().unwrap() - rect.center()))
            .angle()
                * self.winding.to_float();

            if let Some(snap_angle) = if ui.input().modifiers.shift_only() {
                self.shift_snap
            } else {
                self.snap
            } {
                assert!(
                    snap_angle > 0.0,
                    "non-positive snap angles are not supported"
                );
                new_value = (new_value / snap_angle).round() * snap_angle;
            }

            if self.wrap == WrapMode::Unsigned {
                new_value = normalized_angle_unsigned_excl(new_value);
            }

            if self.wrap == WrapMode::None {
                let prev_turns = (prev_value / TAU).round();
                new_value += prev_turns * TAU;

                if new_value - prev_value > (TAU / 2.0) {
                    new_value -= TAU;
                } else if new_value - prev_value < -(TAU / 2.0) {
                    new_value += TAU;
                }
            }

            if let Some(min) = self.min {
                new_value = new_value.max(min);
            }

            if let Some(max) = self.max {
                new_value = new_value.min(max);
            }

            set(&mut self.get_set_value, new_value);
            response.mark_changed();
        }

        if ui.is_rect_visible(rect) {
            let visuals = ui.style().interact(&response).clone();
            let radius = self.diameter / 2.0;

            let value = get(&mut self.get_set_value);

            let angle_to_shape_outline = |angle: f32| {
                rotation_matrix
                    * Vec2::angled(angle * self.winding.to_float())
                    * (self.shape.eval(angle * self.winding.to_float()) * radius)
            };

            self.shape.paint_shape(
                ui,
                rect.center(),
                radius,
                visuals.bg_fill,
                visuals.fg_stroke,
                self.orientation.rot2(),
            );

            {
                let paint_axis = |axis_angle| {
                    ui.painter().add(Shape::dashed_line(
                        &[
                            rect.center(),
                            rect.center() + angle_to_shape_outline(axis_angle),
                        ],
                        ui.visuals().window_stroke(), // TODO: Semantically correct color
                        1.0,
                        1.0,
                    ));
                };

                if self.show_axes {
                    for axis in 0..self.axis_count {
                        paint_axis(axis as f32 * (TAU / (self.axis_count as f32)));
                    }
                }
            }

            {
                let paint_stop = |stop_position: f32| {
                    let stop_stroke = {
                        let stop_alpha = 1.0
                            - ((stop_position - value).abs() / (TAU * 0.75))
                                .clamp(0.0, 1.0)
                                .powf(5.0);

                        // TODO: Semantically correct color
                        Stroke::new(
                            visuals.fg_stroke.width,
                            visuals.fg_stroke.color.linear_multiply(stop_alpha),
                        )
                    };

                    ui.painter().line_segment(
                        [
                            rect.center(),
                            rect.center() + angle_to_shape_outline(stop_position),
                        ],
                        stop_stroke,
                    );
                };

                if let Some(min) = self.min {
                    paint_stop(min);
                }

                if let Some(max) = self.max {
                    paint_stop(max);
                }
            }

            {
                ui.painter().line_segment(
                    [rect.center(), rect.center() + angle_to_shape_outline(value)],
                    visuals.fg_stroke, // TODO: Semantically correct color
                );

                ui.painter().circle(
                    rect.center(),
                    self.diameter / 24.0,
                    visuals.text_color(), // TODO: Semantically correct color
                    visuals.fg_stroke,    // TODO: Semantically correct color
                );

                ui.painter().circle(
                    rect.center() + angle_to_shape_outline(value),
                    self.diameter / 24.0,
                    visuals.text_color(), // TODO: Semantically correct color
                    visuals.fg_stroke,    // TODO: Semantically correct color
                );
            }
        }

        response
    }
}
