use crate::gui::{Drawable, DrawingPar, draw_all_registers, HEIGHT_SPACING_RATIO};
use crate::operation::Measure;
use raylib::prelude::*;
use crate::gui::gui_circuit::GuiMeasure;
use std::panic::panic_any;

const GOLDEN_RATIO:f32 = 1.618033988749894;

impl Drawable for GuiMeasure {
    fn layout(&mut self, parameter: &DrawingPar) -> f32 {
        let gate_height = parameter.register_spacing * HEIGHT_SPACING_RATIO ;
        let gate_width = gate_height*GOLDEN_RATIO;

        self.gui_data.width = gate_width + parameter.margin*2.0;
        self.gui_data.outline.x = parameter.margin;
        self.gui_data.outline.y = parameter.qbit_y_offset(self.target) - gate_height*0.5;
        self.gui_data.outline.width = gate_width;
        self.gui_data.outline.height = gate_height;


        self.gui_data.width
    }


    fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter:&DrawingPar) {
        draw_all_registers(drawer,pos,parameter,self.gui_data.width);

        let mut outline = self.gui_data.outline.clone();
        outline.x += pos.x;
        outline.y += pos.y;
        drawer.draw_rectangle_rec(outline,Color::BLACK);
        drawer.draw_rectangle_lines_ex(outline, parameter.register_thickness as i32, parameter.foreground_color);


    }
}
