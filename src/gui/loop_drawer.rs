use crate::gui::{Drawable, DrawingPar};
use crate::operation::Loop;
use raylib::drawing::RaylibDraw;
use raylib::math::{Vector2, Rectangle};

impl Drawable for Loop {

    fn draw(&self, drawer: &mut impl RaylibDraw, pos:Vector2, parameter:&DrawingPar) -> Vector2 {
        let mut pos_final = {
            let mut pos_start = pos.clone();
            pos_start.x = parameter.margin;
            self.circuit.draw(drawer, pos_start, parameter)
        };

        pos_final.x +=  parameter.margin;

        let loop_width = pos_final.x - pos.x - parameter.margin;
        let loop_height = ((parameter.nb_qbits+1) as f32) * parameter.register_spacing;

        let rect  = Rectangle::new(pos.x+parameter.margin,pos.y-parameter.register_spacing,loop_width,loop_height);

        drawer.draw_rectangle_lines_ex(rect, parameter.register_thickness as i32, parameter.foreground_color);


        {
            let mut pos1 = pos.clone();
            let mut pos2 = pos.clone();
            for _nb_qbit in 0..parameter.nb_qbits {
                pos1.x = pos.x;
                pos2.x = pos.x+parameter.margin;
                drawer.draw_line_ex(pos1,pos2,parameter.register_thickness,parameter.foreground_color);

                pos1.x = pos_final.x;
                pos2.x = pos1.x - parameter.margin;
                drawer.draw_line_ex(pos1,pos2,parameter.register_thickness,parameter.foreground_color);

                pos1.y += parameter.register_spacing;
                pos2.y += parameter.register_spacing;
            }
        }



        pos_final
    }
}
