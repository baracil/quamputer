mod circuit_drawer;
mod gate_drawer;
mod loop_drawer;
mod measure_drawer;
pub mod gui_circuit;
pub mod camera_manager;
pub mod gui_drawer;


use raylib::prelude::*;
use rsgui::font::FontInfo;
use crate::gui::gui_circuit::GuiCircuitElement;
use crate::gui::gui_drawer::GuiDrawer;

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

    pub fn scale(&self,factor:f32) -> Self {
        DrawingPar{
            font:FontInfo{font:self.font.font.clone(),size:self.font.size*factor},
            nb_qbits:self.nb_qbits,
            register_spacing:self.register_spacing*factor,
            register_thickness:self.register_thickness*factor,
            margin:self.margin*factor,
            foreground_color:self.foreground_color,
            background_color:self.background_color,
        }
    }

    pub fn qbit_y_offset(&self, qbit_idx:u8) -> f32 {
        (qbit_idx as f32) * self.register_spacing
    }

    pub fn full_circuit_height(&self) -> f32 {
        return (self.nb_qbits as f32 + 1.0)*self.register_spacing;
    }

    // pub fn flip_rectangle(&self, rect: &mut Rectangle, flipped:bool) {
    //     if flipped {
    //         rect.y = self.full_circuit_height()-(rect.y+rect.height);
    //     }
    // }
    //
    // pub fn flip_vector(&self, vect:&mut Vector2, flipped:bool) {
    //     if flipped {
    //         vect.y = self.full_circuit_height()-vect.y;
    //     }
    //  }
    //
    // pub fn flip_y(&self, y:f32, flipped:bool) -> f32 {
    //     if flipped {self.full_circuit_height() - y} else {y}
    // }
}


pub trait Drawable {
    fn layout(&mut self,parameter:&DrawingPar) -> f32;
    fn draw<T:RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter:&DrawingPar) ;
}

impl Drawable for GuiCircuitElement {

    fn layout(&mut self, parameter: &DrawingPar) -> f32 {
        match self {
            GuiCircuitElement::GuiLoop(p) => p.layout(parameter),
            GuiCircuitElement::GuiGate(p) => p.layout(parameter),
            GuiCircuitElement::GuiMeasure(p) => p.layout(parameter)
        }
    }

    fn draw<T:RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter:&DrawingPar) {
        match self {
            GuiCircuitElement::GuiLoop(p) => p.draw(drawer, parameter),
            GuiCircuitElement::GuiGate(p) => p.draw(drawer, parameter),
            GuiCircuitElement::GuiMeasure(p) => p.draw(drawer, parameter),
        }
    }

}

pub(crate) fn draw_all_registers(drawer: &mut impl RaylibDraw, pos: Vector2, parameter:&DrawingPar, width:f32, flipped:bool) {
    let mut pos_start = pos.clone();
    let mut pos_end = pos.clone();
    pos_end.x = pos_start.x+width;

    for i in 0..parameter.nb_qbits {
        pos_start.y = pos.y + (i as f32) * parameter.register_spacing;
        pos_end.y = pos.y + (i as f32) * parameter.register_spacing;

        drawer.draw_line_ex(pos_start, pos_end, parameter.register_thickness, parameter.foreground_color);


        // parameter.flip_vector(&mut pos_start,flipped);
        // parameter.flip_vector(&mut pos_end,flipped);
    }
}
