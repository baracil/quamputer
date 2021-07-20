use crate::gui::{Drawable, DrawingPar, draw_all_registers, HEIGHT_SPACING_RATIO};
use raylib::prelude::*;
use crate::gui::gui_circuit::{GuiMeasure, GuiCircuitElement, GuiMeasureData};
use crate::gui::gui_drawer::GuiDrawer;
use vec_tree::VecTree;

const GOLDEN_RATIO:f32 = 1.618033988749894;

impl Drawable for GuiMeasure {
    fn layout(&self, parameter: &DrawingPar, tree: &VecTree<GuiCircuitElement>) -> f32 {
        let gate_height = parameter.register_spacing * HEIGHT_SPACING_RATIO ;
        let gate_width = gate_height*GOLDEN_RATIO;

        let width = gate_width + parameter.margin*2.0;

        let mut data = GuiMeasureData::default();

        data.width = width;
        data.outline.x = parameter.margin;
        data.outline.y = parameter.qbit_y_offset(self.target) - gate_height*0.5;
        data.outline.width = gate_width;
        data.outline.height = gate_height;

        self.gui_data.replace(data);

        width
    }


    fn draw<T:RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter:&DrawingPar, tree: &VecTree<GuiCircuitElement>) {
        drawer.draw_all_registers(parameter,self.gui_data.borrow().width);

        drawer.draw_rectangle_rec(&self.gui_data.borrow().outline,Color::BLACK);
        drawer.draw_rectangle_lines_ex(&self.gui_data.borrow().outline, parameter.register_thickness as i32, parameter.foreground_color);


    }
}
