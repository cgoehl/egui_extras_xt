use std::ops::RangeInclusive;

use eframe::egui::{DragValue, Grid, Ui};
use egui_extras_xt::common::{Orientation, WidgetShape, Winding};
use egui_extras_xt::knobs::EncoderKnob;
use egui_extras_xt::ui::drag_rangeinclusive::DragRangeInclusive;
use egui_extras_xt::ui::optional_value_widget::OptionalValueWidget;
use egui_extras_xt::ui::widgets_from_iter::SelectableValueFromIter;
use strum::IntoEnumIterator;

use crate::pages::ui::{widget_orientation_ui, widget_shape_ui};
use crate::pages::PageImpl;

pub struct EncoderKnobPage {
    value: f32,
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

impl Default for EncoderKnobPage {
    fn default() -> EncoderKnobPage {
        EncoderKnobPage {
            value: 0.0,
            interactive: true,
            diameter: 32.0,
            drag_length: 0.008,
            winding: Winding::Clockwise,
            thickness: 0.66,
            shape: WidgetShape::Circle,
            animated: true,
						show_axes: true,
						axis_count: 10,
        }
    }
}

impl PageImpl for EncoderKnobPage {
    fn ui(&mut self, ui: &mut Ui) {
        ui.add(
            EncoderKnob::new(&mut self.value)
                .interactive(self.interactive)
                .diameter(self.diameter)
                .drag_length(self.drag_length)
                .winding(self.winding)
                .thickness(self.thickness)
                .shape(self.shape.clone())
                .animated(self.animated)
								.show_axes(self.show_axes)
								.axis_count(self.axis_count)
                
        );
        ui.separator();

        Grid::new("encoder_knob_properties")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .striped(true)
            .show(ui, |ui| {
                ui.label("Value");
                ui.add(DragValue::new(&mut self.value));
                ui.end_row();

                ui.label("Interactive");
                ui.checkbox(&mut self.interactive, "");
                ui.end_row();

                ui.label("Diameter");
                ui.add(DragValue::new(&mut self.diameter));
                ui.end_row();

                ui.label("Drag length");
                ui.add(DragValue::new(&mut self.drag_length).min_decimals(1).speed(0.001));
                ui.end_row();

                ui.label("Winding");
                ui.horizontal(|ui| {
                    ui.selectable_value_from_iter(&mut self.winding, Winding::iter());
                });
                ui.end_row();

                ui.label("Thickness");
                ui.add(DragValue::new(&mut self.thickness));
                ui.end_row();

                ui.label("Shape");
                widget_shape_ui(ui, &mut self.shape);
                ui.end_row();

                ui.label("Animated");
                ui.checkbox(&mut self.animated, "");
                ui.end_row();

								ui.label("Show axes");
                ui.checkbox(&mut self.show_axes, "");
                ui.end_row();

                ui.label("Axis count");
                ui.add(DragValue::new(&mut self.axis_count));
                ui.end_row();
            });
    }
}
