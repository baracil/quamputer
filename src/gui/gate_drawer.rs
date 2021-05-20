use crate::gui::{Drawable, DrawingPar, draw_all_registers, HEIGHT_SPACING_RATIO};
use raylib::drawing::RaylibDraw;
use raylib::math::Vector2;
use crate::gate::GateWithoutControl;
use crate::gui::gui_circuit::{GuiGate, GuiGateData};
use std::panic::panic_any;
use rs_gui::size::Size;

impl Drawable for GuiGate {

    fn layout(&mut self, parameter: &DrawingPar) -> f32 {
        let gate_size = self.gate.width(parameter);
        let gate_y_center = self.gate.y_middle(parameter);
        let text =self.gate.text();

        self.gui_data.text = text;
        match &self.gui_data.text {
            None => {
                self.gui_data.outline.width = 0.0;
                self.gui_data.outline.height = 0.0;
                self.gui_data.text_size = Size::new(0.0,0.0);
                self.gui_data.text_position = Vector2::default();
            }
            Some(t) => {
                let size = parameter.font.measure_text(t,0.0);
                self.gui_data.outline.x = parameter.margin;
                self.gui_data.outline.y = gate_y_center-gate_size*0.5;
                self.gui_data.outline.width = gate_size;
                self.gui_data.outline.height = gate_size;
                self.gui_data.text_size = size;
                self.gui_data.text_position.x = self.gui_data.outline.x + (self.gui_data.outline.width - size.width())*0.5;
                self.gui_data.text_position.y = self.gui_data.outline.y + (self.gui_data.outline.height - size.height())*0.5;
            }
        }

        let width = 2.0*parameter.margin + gate_size;
        self.gui_data.width = width;
        self.gui_data.center.x = parameter.margin + gate_size *0.5;
        self.gui_data.center.y = gate_y_center;
        self.gui_data.gate_size = gate_size;
        width
    }

    fn draw(&self, drawer: &mut impl RaylibDraw, pos:Vector2, parameter:&DrawingPar, flipped:bool) {
        let width = self.gui_data.width;

        draw_all_registers(drawer,pos,parameter,width,flipped);

        self.draw_control_qbits(drawer,parameter,&pos,&self.control_bits,flipped);


        self.gate.draw(drawer,pos,parameter,&self.gui_data,flipped);
    }


}

impl GuiGate {

    fn draw_control_qbits(&self, drawer: &mut impl RaylibDraw, parameter:&DrawingPar, pos:&Vector2, control_bits:&[u8], flipped:bool) {
        let mut center = *pos+self.gui_data.center;
        let mut cpos_end = center.clone();
        let radius = parameter.register_spacing*0.06;

        parameter.flip_vector(&mut center,flipped);

        for control_bit in control_bits {
            cpos_end.y = pos.y+parameter.qbit_y_offset(*control_bit);

            parameter.flip_vector(&mut cpos_end,flipped);

            drawer.draw_line_ex(center,cpos_end,parameter.register_thickness, parameter.foreground_color);
            drawer.draw_circle_v(cpos_end,radius,parameter.foreground_color);
        }



    }

}


impl GateWithoutControl {

    pub fn width(&self, parameter: &DrawingPar) -> f32 {
        let factor:f32 = match self {
            GateWithoutControl::Not(_) => 0.5,
            GateWithoutControl::Swap(_, _) => 0.5,
            GateWithoutControl::X(_) => 1.0,
            GateWithoutControl::Y(_) => 1.0,
            GateWithoutControl::Z(_) => 1.0,
            GateWithoutControl::Hadamard(_) => 1.0
        };
        return parameter.register_spacing*factor*HEIGHT_SPACING_RATIO
    }

    pub fn text(&self) -> Option<String>{
         match self {
             GateWithoutControl::Not(_) => None,
             GateWithoutControl::X(_) => Some(String::from("X")),
             GateWithoutControl::Y(_) => Some(String::from("Y")),
             GateWithoutControl::Z(_) => Some(String::from("Z")),
             GateWithoutControl::Swap(_, _) => None,
             GateWithoutControl::Hadamard(_) => Some(String::from("H"))
         }
    }

    pub fn y_middle(&self, parameter: &DrawingPar) -> f32 {
        match self {
            GateWithoutControl::Not(t) => parameter.qbit_y_offset(*t),
            GateWithoutControl::X(t) => parameter.qbit_y_offset(*t),
            GateWithoutControl::Y(t) => parameter.qbit_y_offset(*t),
            GateWithoutControl::Z(t) => parameter.qbit_y_offset(*t),
            GateWithoutControl::Swap(t1, t2) => (parameter.qbit_y_offset(*t1) + parameter.qbit_y_offset(*t2))*0.5,
            GateWithoutControl::Hadamard(t) => parameter.qbit_y_offset(*t),
        }
    }

    fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar, gui_data:&GuiGateData, flipped:bool) {
        match self {
            GateWithoutControl::X(_) => draw_gate_with_text(drawer, pos, parameter, gui_data,flipped),
            GateWithoutControl::Y(_) => draw_gate_with_text(drawer, pos, parameter, gui_data,flipped),
            GateWithoutControl::Z(_) => draw_gate_with_text(drawer, pos, parameter, gui_data,flipped),
            GateWithoutControl::Hadamard(_) => draw_gate_with_text(drawer, pos, parameter, gui_data,flipped),

            GateWithoutControl::Not(_) => draw_not_gate(drawer,pos,parameter,gui_data, flipped),
            GateWithoutControl::Swap(target1, target2) => draw_swap_gate(drawer,pos,parameter,gui_data, target1,target2, flipped),
        }
    }
}


fn draw_gate_with_text(drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar, gui_data:&GuiGateData, flipped:bool) {
    let mut gate = gui_data.outline.clone();
    gate.x += pos.x;
    gate.y += pos.y;

    parameter.flip_rectangle(&mut gate,flipped);

    drawer.draw_rectangle_rec(gate,parameter.background_color);
    drawer.draw_rectangle_lines_ex(gate,parameter.register_thickness as i32, parameter.foreground_color);



    if let Some(text) = &gui_data.text {
        let mut position = pos+gui_data.text_position;
        let center_y = parameter.flip_y(position.y + gui_data.text_size.height()*0.5, flipped) - gui_data.text_size.height()*0.5;
        position.y = center_y;
        parameter.font.draw_text(drawer,text,&position,0.0,parameter.foreground_color);
    }
}

fn draw_not_gate(drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar, gui_data:&GuiGateData, flipped:bool) {
    let circle_radius = gui_data.gate_size*0.5;
    let mut center = pos+gui_data.center;
    parameter.flip_vector(&mut center,flipped);

    drawer.draw_circle_sector_lines(center,circle_radius,0,360,32,parameter.foreground_color);

    let mut pos1 = Vector2::new(center.x-circle_radius,center.y);
    let mut pos2 = Vector2::new(center.x+circle_radius,center.y);
    drawer.draw_line_ex(pos1,pos2,parameter.register_thickness, parameter.foreground_color);
    pos1.x = center.x;
    pos2.x = center.x;
    pos1.y -= circle_radius;
    pos2.y += circle_radius;
    drawer.draw_line_ex(pos1,pos2,parameter.register_thickness, parameter.foreground_color);

}

fn draw_swap_gate(drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar, gui_data:&GuiGateData, target1: &u8, target2: &u8, flipped:bool) {
    let target_y_pos1 = parameter.flip_y(pos.y + parameter.qbit_y_offset(*target1),flipped);
    let target_y_pos2 = parameter.flip_y(pos.y + parameter.qbit_y_offset(*target2),flipped);
    let size = gui_data.gate_size*0.5;


    let pos1 = Vector2::new(pos.x + parameter.margin+size, target_y_pos1);
    let pos2 = Vector2::new(pos.x + parameter.margin+size, target_y_pos2);

    drawer.draw_line_ex(pos1,pos2,parameter.register_thickness, parameter.foreground_color);

    draw_swap_cross(drawer,pos1,size,parameter);
    draw_swap_cross(drawer,pos2,size,parameter);

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
