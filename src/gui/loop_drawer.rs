use raylib::color::Color;
use raylib::drawing::RaylibDraw;

use crate::gui::{Drawable};
use crate::gui::gui_circuit::{DrawableParameter, GuiLoop, HoverData};
use crate::gui::gui_drawer::GuiDrawer;

impl Drawable for GuiLoop {
    fn layout(&mut self, parameter: &DrawableParameter) -> f32 {
        let circuit_width: f32 = self.circuit.layout(parameter);

        let margin = parameter.margin;
        let width = circuit_width + margin * 2.0;

        self.gui_data.margin = margin;
        self.gui_data.width = width;
        self.gui_data.outline.x = margin;
        self.gui_data.outline.y = -parameter.register_spacing;
        self.gui_data.outline.height = parameter.full_circuit_height(parameter.nb_qbits);
        self.gui_data.outline.width = circuit_width;
        self.gui_data.outline_background = Color::new(128, 128, 128, 255);

        width
    }

    fn draw<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter: &DrawableParameter) -> Option<HoverData> {
        drawer.draw_rectangle_rec(&self.gui_data.outline, self.gui_data.outline_background);
        drawer.draw_rectangle_lines_ex(&self.gui_data.outline, parameter.register_thickness as i32, parameter.foreground_color);
        drawer.draw_all_registers(parameter, self.gui_data.width);

        drawer.push_offset();
        drawer.shift_by(self.gui_data.margin);
        let hoover_result = self.circuit.draw(drawer, parameter);
        drawer.pop_offset();


        if hoover_result.is_none() {
            let transformed_outline = drawer.transform_rectangle(&self.gui_data.outline);
            let mouse_position = drawer.mouse_info.world_pos;
            let hover = transformed_outline.check_collision_point_rec(mouse_position);

            if hover {
                drawer.draw_rectangle_lines_ex(&self.gui_data.outline, parameter.register_thickness as i32, parameter.hover_color);
                return Some(HoverData::for_loop(self.id));
            }
        }

        hoover_result
    }
}
