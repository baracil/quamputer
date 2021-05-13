use crate::gui::{Drawable, DrawingPar, draw_all_registers, HEIGHT_SPACING_RATIO, Width};
use crate::operation::Gate;
use raylib::drawing::RaylibDraw;
use raylib::math::Vector2;
use crate::gate::GateWithoutControl;
use raylib::prelude::Rectangle;

impl Drawable for Gate {
    fn layout(&mut self, parameter: &DrawingPar) -> Width {
        let gate_width = self.gate.gate_width(parameter);
        let width = parameter.margin + gate_width.0;
        Width(width)
    }

    fn draw(&self, drawer: &mut impl RaylibDraw, pos:Vector2, parameter:&DrawingPar) -> Vector2 {
        let gate_width = self.gate.gate_width(parameter);
        let width = gate_width.0 + parameter.margin;

        draw_all_registers(drawer,pos,parameter,width);

//        draw_control_qbits(drawer,pos,parameter,gate_width, self.gate.qbit_target(),&self.control_bits);


        self.gate.draw(drawer,pos,parameter)
    }
}

fn draw_control_qbits(drawer: &mut impl RaylibDraw, pos:Vector2, parameter:&DrawingPar, gate_width:Width, target:u8,control_bits:&[u8]) {
    let cpos_start = Vector2::new(pos.x + parameter.margin + gate_width.0*0.5, pos.y + parameter.qbit_y_offset(target)) ;
    let mut cpos_end = cpos_start.clone();
    let radius = parameter.register_spacing*0.06;

    for control_bit in control_bits {
        cpos_end.y = parameter.qbit_y_offset(*control_bit);
        drawer.draw_line_ex(cpos_start,cpos_end,parameter.register_thickness, parameter.foreground_color);

        drawer.draw_circle_v(cpos_end,radius,parameter.foreground_color);
    }



}

impl GateWithoutControl {

    pub fn gate_width(&self, parameter: &DrawingPar) -> Width {
        let factor:f32 = match self {
            GateWithoutControl::Not(_) => 0.5,
            GateWithoutControl::Swap(_, _) => 0.5,
            GateWithoutControl::X(_) => 1.0,
            GateWithoutControl::Y(_) => 1.0,
            GateWithoutControl::Z(_) => 1.0,
            GateWithoutControl::Hadamard(_) => 1.0
        };
        return Width(parameter.register_spacing*factor*HEIGHT_SPACING_RATIO)
    }

    fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar) -> Vector2 {
        match self {
            GateWithoutControl::X(target) => draw_gate_with_letter(drawer,pos,parameter,target,"X"),
            GateWithoutControl::Y(target) => draw_gate_with_letter(drawer,pos,parameter,target,"Y"),
            GateWithoutControl::Z(target) => draw_gate_with_letter(drawer,pos,parameter,target,"Z"),
            GateWithoutControl::Hadamard(target) => draw_gate_with_letter(drawer,pos,parameter,target,"H"),

            GateWithoutControl::Not(target) => draw_not_gate(drawer,pos,parameter,target),
            GateWithoutControl::Swap(target1, target2) => draw_swap_gate(drawer,pos,parameter,target1,target2),
        }
    }
}


fn draw_gate_with_letter(drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar, target: &u8, letter: &str) -> Vector2 {
    let target_y_pos = pos.y + parameter.qbit_y_offset(*target);
    let gate_size = parameter.register_spacing*HEIGHT_SPACING_RATIO;

    let gate = Rectangle::new(pos.x+parameter.margin, target_y_pos-gate_size*0.5,gate_size,gate_size);

    drawer.draw_rectangle_rec(gate,parameter.background_color);
    drawer.draw_rectangle_lines_ex(gate,parameter.register_thickness as i32, parameter.foreground_color);
    //draw letter
    let text_size = parameter.font.measure_text(letter,0.0);

    let position = Vector2::new(
        gate.x+(gate.width - text_size.width())*0.5,
        gate.y+(gate.height -text_size.height())*0.5
    );

    parameter.font.draw_text(drawer,letter,&position,0.0,parameter.foreground_color);


    Vector2::new(pos.x + gate_size+parameter.margin, pos.y)
}

fn draw_not_gate(drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar, target: &u8) -> Vector2 {
    let target_y_pos = pos.y + (*target as f32) * parameter.register_spacing;
    let circle_radius = parameter.register_spacing*0.5*HEIGHT_SPACING_RATIO*0.5;

    let center = Vector2::new(pos.x+parameter.margin+circle_radius, target_y_pos);

    drawer.draw_circle_v(center,circle_radius,parameter.background_color);
    drawer.draw_circle_sector_lines(center,circle_radius,0,360,32,parameter.foreground_color);

    let mut pos1 = Vector2::new(center.x-circle_radius,center.y);
    let mut pos2 = Vector2::new(center.x+circle_radius,center.y);
    drawer.draw_line_ex(pos1,pos2,parameter.register_thickness, parameter.foreground_color);
    pos1.x = center.x;
    pos2.x = center.x;
    pos1.y -= circle_radius;
    pos2.y += circle_radius;
    drawer.draw_line_ex(pos1,pos2,parameter.register_thickness, parameter.foreground_color);


    Vector2::new(pos.x+ parameter.margin+circle_radius*2.0, pos.y)

}

fn draw_swap_gate(drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar, target1: &u8, target2: &u8) -> Vector2 {
    let target_y_pos1 = pos.y + (*target1 as f32) * parameter.register_spacing;
    let target_y_pos2 = pos.y + (*target2 as f32) * parameter.register_spacing;
    let size = parameter.register_spacing*0.5*HEIGHT_SPACING_RATIO*0.5;


    let pos1 = Vector2::new(pos.x + parameter.margin+size, target_y_pos1);
    let pos2 = Vector2::new(pos.x + parameter.margin+size, target_y_pos2);

    drawer.draw_line_ex(pos1,pos2,parameter.register_thickness, parameter.foreground_color);

    draw_swap_cross(drawer,pos1,size,parameter);
    draw_swap_cross(drawer,pos2,size,parameter);

    Vector2::new(pos.x + parameter.margin+size*2.0, pos.y)
}

fn draw_swap_cross(drawer: &mut impl RaylibDraw, center: Vector2, size:f32, parameter: &DrawingPar) {
    let size = size*0.5;
    let mut point1 = center.clone();
    let mut point2 = center.clone();
    point1.x -= size;
    point1.y -= size;
    point2.x += size;
    point2.y += size;
    drawer.draw_line_ex(point1,point2, parameter.register_thickness, parameter.foreground_color);

    point1.y += 2.0*size;
    point2.y -= 2.0*size;
    drawer.draw_line_ex(point1,point2, parameter.register_thickness, parameter.foreground_color);

}
