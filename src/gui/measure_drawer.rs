use raylib::prelude::*;
use vec_tree::VecTree;

use crate::gui::{Drawable, DrawingPar, HEIGHT_SPACING_RATIO};
use crate::gui::gui_circuit::{GuiCircuitElement, GuiMeasure, GuiMeasureData, HoverData};
use crate::gui::gui_drawer::GuiDrawer;

const GOLDEN_RATIO: f32 = 1.618033988749894;

impl Drawable for GuiMeasure {
    fn layout(&self, nb_qbits:u8, parameter: &DrawingPar, _tree: &VecTree<GuiCircuitElement>) -> f32 {
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


    fn draw<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, nb_qbits:u8, parameter: &DrawingPar, _tree: &VecTree<GuiCircuitElement>) -> Option<HoverData> {
        let transformed_outline = drawer.transform_rectangle(&self.gui_data.borrow().outline);
        let mouse_pos = drawer.get_world_mouse_position();

        drawer.draw_all_registers(nb_qbits, parameter, self.gui_data.borrow().width);

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
