use raylib::prelude::*;

use crate::gui::{Drawable, DrawingPar};
use crate::gui::gui_circuit::{GuiCircuitElement, GuiRoot};
use crate::gui::gui_drawer::GuiDrawer;

impl GuiRoot {
    pub fn layout(&mut self) {
        let root_index = self.tree.get_root_index();

        if root_index.is_none() {
            return;
        }
        let root_index = root_index.unwrap();
        let root = self.tree.get(root_index).unwrap();

        root.layout(self.nb_qbits, &self.parameter, &self.tree);
    }

    pub fn draw<T: RaylibDraw>(&self, rl2d: &mut RaylibMode2D<T>) {
        let mut drawer = GuiDrawer::default(rl2d,self.position);
        match self.get_root() {
            Some(r) => r.draw(&mut drawer, self.nb_qbits , &self.parameter, &self.tree),
            None => {}
        }
    }


    fn get_root(&self) -> Option<&GuiCircuitElement> {
        self.tree.get_root_index().and_then(|i| { self.tree.get(i) })
    }
}


