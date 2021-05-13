mod circuit_drawer;
mod gate_drawer;
mod loop_drawer;
mod measure_drawer;
pub mod gui_circuit;


use raylib::prelude::*;
use crate::operation::{CircuitElement};
use rs_gui::font::FontInfo;
use std::iter::Sum;
use std::ops::Add;
use crate::gui::gui_circuit::GuiCircuitElement;

const HEIGHT_SPACING_RATIO:f32 = 0.6;


pub struct DrawingPar {
    pub font: FontInfo,
    pub nb_qbits:u8,
    pub register_spacing:f32,
    pub register_thickness:f32,
    pub margin:f32,
    pub foreground_color:Color,
    pub background_color:Color
}

impl DrawingPar {
    pub fn qbit_y_offset(&self, qbit_idx:u8) -> f32 {
        (qbit_idx as f32) * self.register_spacing
    }

    pub fn full_circuit_height(&self) -> f32 {
        return (self.nb_qbits as f32 + 1.0)*self.register_spacing;
    }
}


pub trait Drawable {
    fn layout(&mut self,parameter:&DrawingPar) -> f32;
    fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter:&DrawingPar) ;
}

impl Drawable for GuiCircuitElement {

    fn layout(&mut self, parameter: &DrawingPar) -> f32 {
        match self {
            GuiCircuitElement::GuiLoop(p) => p.layout(parameter),
            GuiCircuitElement::GuiGate(p) => p.layout(parameter),
            GuiCircuitElement::GuiMeasure(p) => p.layout(parameter)
        }
    }

    fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter:&DrawingPar) {
        match self {
            GuiCircuitElement::GuiLoop(p) => p.draw(drawer, pos, parameter),
            GuiCircuitElement::GuiGate(p) => p.draw(drawer, pos, parameter),
            GuiCircuitElement::GuiMeasure(p) => p.draw(drawer, pos, parameter),
        }
    }


}

pub(crate) fn draw_all_registers(drawer: &mut impl RaylibDraw, pos: Vector2, parameter:&DrawingPar, width:f32) {
    let mut pos_start = pos.clone();
    let mut pos_end = pos.clone();
    pos_end.x = pos_start.x+width;

    for _i in 0..parameter.nb_qbits {
        drawer.draw_line_ex(pos_start, pos_end, parameter.register_thickness, parameter.foreground_color);
        pos_start.y+=parameter.register_spacing;
        pos_end.y+=parameter.register_spacing;
    }
}
