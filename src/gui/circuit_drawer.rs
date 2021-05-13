use crate::gui::{Drawable, DrawingPar};
use raylib::prelude::*;
use crate::circuit::Circuit;
use crate::gui::gui_circuit::GuiCircuit;

impl Drawable for GuiCircuit {
    fn layout(&mut self, parameter: &DrawingPar) -> f32 {
        let width = self.elements.iter_mut()
            .map(|o| o.layout(parameter))
            .sum();
        self.gui_data.width = width;
        width
    }

    fn draw(&self, drawer: &mut impl RaylibDraw, pos: Vector2, parameter: &DrawingPar) {
        let mut pos = pos.clone();
        for operation in self.elements.iter() {
            operation.draw(drawer, pos, parameter);
            pos.x += operation.gui_data().width
        };
    }
}
