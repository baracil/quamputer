use raylib::prelude::*;
use vec_tree::VecTree;

use crate::gui::{Drawable, HEIGHT_SPACING_RATIO, Style};
use crate::gui::gui_circuit::{DrawableParameter, GuiCircuitElement, GuiMeasure, GuiMeasureData, HoverData};
use crate::gui::gui_drawer::GuiDrawer;

const GOLDEN_RATIO: f32 = 1.618033988749894;

impl Drawable for GuiMeasure {
    fn layout(&self, parameter: &DrawableParameter) -> f32 {
        let gate_height = parameter.register_spacing * HEIGHT_SPACING_RATIO;
        let gate_width = gate_height * GOLDEN_RATIO;

        let width = gate_width + parameter.margin * 2.0;

        let mut data = GuiMeasureData::default();

        data.width = width;
        data.outline.x = parameter.margin;
        data.outline.y = parameter.qbit_y_offset(self.target) - gate_height * 0.5;
        data.outline.width = gate_width;
        data.outline.height = gate_height;

        self.gui_data.replace(data);

        width
    }


    fn draw<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter: &DrawableParameter) -> Option<HoverData> {
        let transformed_outline = drawer.transform_rectangle(&self.gui_data.borrow().outline);
        let mouse_pos = drawer.mouse_info.world_pos;

        drawer.draw_all_registers(parameter, self.gui_data.borrow().width);

        drawer.draw_rectangle_rec(&self.gui_data.borrow().outline, Color::BLACK);
        drawer.draw_rectangle_lines_ex(&self.gui_data.borrow().outline, parameter.register_thickness as i32, parameter.foreground_color);

        let hover = transformed_outline.check_collision_point_rec(mouse_pos);

        if hover {
            drawer.draw_rectangle_lines_ex(&self.gui_data.borrow().outline, parameter.register_thickness as i32,parameter.hover_color);
        }

        if hover {
            return self.index.map(|index| { HoverData::for_measure(index)});
        }
        None
    }
}
