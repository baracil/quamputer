use crate::gui::{Drawable, DrawingPar};
use raylib::prelude::*;
use crate::gui::gui_circuit::{GuiRoot, GuiCircuitElement};
use crate::gui::gui_drawer::GuiDrawer;

impl GuiRoot {
    pub fn layout(&mut self, parameter: &DrawingPar) {
        let root_index = self.tree.get_root_index();

        if root_index.is_none() {
            self.width = 0;
            self.height = 0;
            return;
        }
        let root_index = root_index.unwrap();
        let root = self.tree.get(root_index).unwrap();

        let width = root.layout(parameter, &self.tree);
        let height = parameter.full_circuit_height();
        self.width = width.max(0.0).round() as u32;
        self.height = height.max(0.0).round() as u32;
    }

    pub fn draw<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter: &DrawingPar) {
        match self.get_root() {
            Some(r) => r.draw(drawer, parameter, &self.tree),
            None => {}
        }
    }


    fn get_root(&self) -> Option<&GuiCircuitElement> {
        self.tree.get_root_index().and_then(|i| { self.tree.get(i) })
    }

}


