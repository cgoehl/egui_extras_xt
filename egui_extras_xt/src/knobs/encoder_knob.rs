use std::f32::consts::TAU;
use std::ops::RangeInclusive;

use ecolor::Color32;
use egui::{self, Response, Sense, Ui, Widget, Shape};
use emath::{remap_clamp, Vec2, Rot2};

use crate::common::{Orientation, WidgetShape, Winding};

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

#[must_use = "You should put this widget in an ui with `ui.add(widget);`"]
pub struct EncoderKnob<'a> {
    get_set_value: GetSetValue<'a>,
    interactive: bool,
    diameter: f32,
    drag_length: f32,
    winding: Winding,
    thickness: f32,
    shape: WidgetShape,
    animated: bool,
    show_axes: bool,
    axis_count: usize,
}

impl<'a> EncoderKnob<'a> {
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
            interactive: true,
            diameter: 32.0,
            drag_length: 1.0,
            winding: Winding::Clockwise,
            thickness: 0.66,
            shape: WidgetShape::Circle,
            animated: true,
            show_axes: true,
            axis_count: 10,
        }
    }

    pub fn interactive(mut self, interactive: bool) -> Self {
        self.interactive = interactive;
        self
    }

    pub fn diameter(mut self, diameter: impl Into<f32>) -> Self {
        self.diameter = diameter.into();
        self
    }

    pub fn drag_length(mut self, drag_length: impl Into<f32>) -> Self {
        self.drag_length = drag_length.into();
        self
    }

    pub fn winding(mut self, winding: Winding) -> Self {
        self.winding = winding;
        self
    }

    pub fn thickness(mut self, thickness: impl Into<f32>) -> Self {
        self.thickness = thickness.into();
        self
    }

    pub fn shape(mut self, shape: WidgetShape) -> Self {
        self.shape = shape;
        self
    }

    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    pub fn show_axes(mut self, show_axes: bool) -> Self {
        self.show_axes = show_axes;
        self
    }

    pub fn axis_count(mut self, axis_count: usize) -> Self {
        self.axis_count = axis_count;
        self
    }
}

impl<'a> Widget for EncoderKnob<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let desired_size = Vec2::splat(self.diameter);
        let rotation_matrix = Rot2::default();

        let (rect, mut response) = ui.allocate_exact_size(
            desired_size,
            if self.interactive {
                Sense::click_and_drag()
            } else {
                Sense::hover()
            },
        );

        if response.dragged() {
            let drag_delta = rotation_matrix.inverse() * response.drag_delta();

            let mut new_value = get(&mut self.get_set_value);

            let delta = drag_delta.x + drag_delta.y * self.winding.to_float();
            new_value += delta * (self.diameter * self.drag_length);

            set(&mut self.get_set_value, new_value);
            response.mark_changed();
        }

        if response.drag_released() {
            if self.animated {
                ui.ctx().clear_animations();
                ui.ctx().animate_value_with_time(
                    response.id,
                    get(&mut self.get_set_value),
                    ui.style().animation_time,
                );
            }
        }

        if ui.is_rect_visible(rect) {
            let visuals = *ui.style().interact(&response);

            let value = if self.animated && !response.dragged() {
                ui.ctx()
                    .animate_value_with_time(response.id, get(&mut self.get_set_value), 0.1)
            } else {
                get(&mut self.get_set_value)
            };

            ui.painter().circle(
                rect.center(),
                self.diameter / 3.0,
                visuals.text_color(), // TODO: Semantically correct color
                visuals.fg_stroke,    // TODO: Semantically correct color
            );

            {
                let angle_to_shape_outline = |angle: f32| {
                  rotation_matrix
                      * Vec2::angled(angle * self.winding.to_float())
                      * (self.shape.eval(angle * self.winding.to_float()) * self.diameter / 2.0)
                };

                let paint_axis = |axis_angle| {
                    ui.painter().add(Shape::line(
                        vec![
                            rect.center(),
                            rect.center() + angle_to_shape_outline(axis_angle),
                        ],
                        visuals.fg_stroke
                    ));
                };

                if self.show_axes {
                    for axis in 0..self.axis_count {
                        paint_axis((axis as f32 * (TAU / (self.axis_count as f32)) + value));
                    }
                }
            }

            ui.painter().circle(
              rect.center(),
              self.diameter / 2.0,
              Color32::TRANSPARENT, // TODO: Semantically correct color
              visuals.fg_stroke,    // TODO: Semantically correct color
          );
          
        }

        response
    }
}
