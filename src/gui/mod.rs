use raylib::prelude::*;
use rsgui::font::FontInfo;
use vec_tree::VecTree;

use crate::gui::gui_circuit::{GuiCircuitElement, HoverData};
use crate::gui::gui_drawer::GuiDrawer;

mod circuit_drawer;
mod gate_drawer;
mod loop_drawer;
mod measure_drawer;
pub mod gui_circuit;
pub mod camera_manager;
pub mod gui_drawer;
pub mod mouse_position;


const HEIGHT_SPACING_RATIO: f32 = 0.6;

#[derive(Clone)]
pub struct DrawingPar {
    pub font: FontInfo,
    pub register_spacing: f32,
    pub register_thickness: f32,
    pub margin: f32,
    pub hover_color: Color,
    pub foreground_color: Color,
    pub background_color: Color,
}


impl DrawingPar {
    pub fn scale(&self, factor: f32) -> Self {
        DrawingPar {
            hover_color: self.hover_color,
            font: FontInfo { font: self.font.font.clone(), size: self.font.size * factor },
            register_spacing: self.register_spacing * factor,
            register_thickness: self.register_thickness * factor,
            margin: self.margin * factor,
            foreground_color: self.foreground_color,
            background_color: self.background_color,
        }
    }

    pub fn qbit_y_offset(&self, qbit_idx: u8) -> f32 {
        (qbit_idx as f32) * self.register_spacing
    }

    pub fn full_circuit_height(&self, nb_qbits:u8) -> f32 {
        return (nb_qbits as f32 + 1.0) * self.register_spacing;
    }
}


pub trait Drawable {
    /// Layout its content and return the width it will use
    fn layout(&self, nb_qbits:u8, parameter: &DrawingPar, tree: &VecTree<GuiCircuitElement>) -> f32;
    fn draw<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, nb_qbits:u8, parameter: &DrawingPar, tree: &VecTree<GuiCircuitElement>) -> Option<HoverData>;
}

impl Drawable for GuiCircuitElement {
    fn layout(&self, nb_qbits:u8, parameter: &DrawingPar, tree: &VecTree<GuiCircuitElement>) -> f32 {
        match self {
            GuiCircuitElement::GuiLoop(p) => p.layout(nb_qbits, parameter, tree),
            GuiCircuitElement::GuiGate(p) => p.layout(nb_qbits, parameter, tree),
            GuiCircuitElement::GuiMeasure(p) => p.layout(nb_qbits, parameter, tree)
        }
    }

    fn draw<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, nb_qbits:u8, parameter: &DrawingPar, tree: &VecTree<GuiCircuitElement>) -> Option<HoverData>{
        match self {
            GuiCircuitElement::GuiLoop(p) => p.draw(drawer, nb_qbits,parameter, tree),
            GuiCircuitElement::GuiGate(p) => p.draw(drawer,nb_qbits, parameter, tree),
            GuiCircuitElement::GuiMeasure(p) => p.draw(drawer, nb_qbits ,parameter, tree),
        }
    }
}

