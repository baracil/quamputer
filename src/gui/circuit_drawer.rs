use raylib::prelude::*;

use crate::gui::{Drawable};
use crate::gui::gui_circuit::{GuiCircuitElement, GuiRoot, HoverData};
use crate::gui::gui_drawer::GuiDrawer;
use crate::gui::mouse_information::MouseInformation;

impl GuiRoot {
    pub fn layout(&mut self) {
        self.circuit.layout(&self.parameter);
    }

    pub fn draw<T: RaylibDraw>(&self, rl2d: &mut RaylibMode2D<T>, mouse_information:&MouseInformation) -> Option<HoverData> {
        let mut drawer = GuiDrawer::default(rl2d, mouse_information, self.position);

        self.circuit.draw(&mut drawer,&self.parameter)
    }

}


