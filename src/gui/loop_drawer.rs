use crate::gui::{Drawable, DrawingPar, draw_all_registers};
use raylib::drawing::RaylibDraw;
use raylib::math::{Vector2};
use crate::gui::gui_circuit::GuiLoop;
use crate::gui::gui_drawer::GuiDrawer;
use std::panic::panic_any;

impl Drawable for GuiLoop {

    fn layout(&mut self, parameter: &DrawingPar) -> f32 {
        let circuit_width = self.circuit.layout(parameter);
        self.gui_data.width = circuit_width +parameter.margin*2.0;
        self.gui_data.outline.x = parameter.margin;
        self.gui_data.outline.y = -parameter.register_spacing;
        self.gui_data.outline.height = parameter.full_circuit_height();
        self.gui_data.outline.width = circuit_width;
        self.gui_data.outline_background = parameter.foreground_color.fade(0.5);

        self.gui_data.width
    }

    fn draw<T:RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter:&DrawingPar) {
        let mut rect = self.gui_data.outline.clone();
        drawer.draw_rectangle_rec(&rect, self.gui_data.outline_background);
        drawer.draw_rectangle_lines_ex(&rect, parameter.register_thickness as i32, parameter.foreground_color);

        drawer.draw_all_registers(parameter,self.gui_data.width);


        drawer.push_offset();
        drawer.shift_by(parameter.margin);
        self.circuit.draw(drawer, parameter);
        drawer.pop_offset()

    }
}
