mod circuit_drawer;
mod gate_drawer;
mod loop_drawer;
mod measure_drawer;


use raylib::prelude::*;
use crate::operation::{QuantumOperation, CircuitPar, LoopPar, GatePar, MeasurePar};
use std::panic::panic_any;
use rs_gui::font::FontInfo;

const HEIGHT_SPACING_RATIO:f32 = 0.8;


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
}


pub trait Drawable {
    fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter:&DrawingPar) -> Vector2 ;
}

impl Drawable for QuantumOperation {
    fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter:&DrawingPar) -> Vector2 {
        match self {
            QuantumOperation::Circuit(p) => p.draw(drawer,pos,parameter),
            QuantumOperation::Loop(p) => p.draw(drawer,pos,parameter),
            QuantumOperation::Gate(p) => p.draw(drawer,pos,parameter),
            QuantumOperation::Measure(p) => p.draw(drawer,pos,parameter),
        }
    }
}

pub(crate) fn draw_all_registers(drawer: &mut impl RaylibDraw, pos: Vector2, parameter:&DrawingPar, width:f32) {
    let mut pos_start = pos.clone();
    let mut pos_end = pos.clone();
    pos_end.x = pos_start.x+width;

    for i in 0..parameter.nb_qbits {
        drawer.draw_line_ex(pos_start, pos_end, parameter.register_thickness, parameter.foreground_color);
        pos_start.y+=parameter.register_spacing;
        pos_end.y+=parameter.register_spacing;
    }
}
