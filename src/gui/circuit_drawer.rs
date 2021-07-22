use raylib::prelude::*;

use crate::gui::{Drawable};
use crate::gui::gui_circuit::{GuiCircuitElement, GuiRoot, HoverData};
use crate::gui::gui_drawer::GuiDrawer;
use crate::gui::mouse_information::MouseInformation;

impl GuiRoot {
    pub fn layout(&mut self) {
        if let Some(root) = self.parameter.get_root() {
            root.layout(&self.parameter);
        }
    }

    pub fn draw<T: RaylibDraw>(&self, rl2d: &mut RaylibMode2D<T>, mouse_information:&MouseInformation) -> Option<HoverData> {
        let mut drawer = GuiDrawer::default(rl2d, mouse_information, self.position);

        self.parameter.get_root()
            .and_then(|root| { root.draw(&mut drawer, &self.parameter)})
    }

}


