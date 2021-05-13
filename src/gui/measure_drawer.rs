use crate::gui::{Drawable, DrawingPar, draw_all_registers, HEIGHT_SPACING_RATIO};
use crate::operation::MeasurePar;
use raylib::prelude::*;

const GOLDEN_RATIO:f32 = 1.618033988749894;

impl Drawable for MeasurePar {
    fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter:&DrawingPar) -> Vector2 {
        let qbit_target = self.target;
        let gate_height = parameter.register_spacing * HEIGHT_SPACING_RATIO ;
        let gate_width = gate_height*GOLDEN_RATIO;

        let width = parameter.margin + gate_width;

        draw_all_registers(drawer,pos,parameter,width);
        let target_register_y = pos.y + (qbit_target as f32) *parameter.register_spacing;
        let gate = Rectangle::new(pos.x + parameter.margin, target_register_y - gate_height*0.5,gate_width,gate_height);
        drawer.draw_rectangle_rec(gate,Color::BLACK);
        drawer.draw_rectangle_lines_ex(gate, parameter.register_thickness as i32, parameter.foreground_color);


        Vector2::new(pos.x+width,pos.y)
    }
}
