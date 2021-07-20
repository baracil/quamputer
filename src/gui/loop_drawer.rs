use crate::gui::{Drawable, DrawingPar};
use raylib::drawing::RaylibDraw;
use crate::gui::gui_circuit::{GuiLoop, GuiCircuitElement, GuiLoopData};
use crate::gui::gui_drawer::GuiDrawer;
use raylib::color::Color;
use vec_tree::VecTree;

impl Drawable for GuiLoop {

    fn layout(&self, parameter: &DrawingPar, tree: &VecTree<GuiCircuitElement>) -> f32 {
        let children = self.index.map(|i| {tree.children(i)});

        let circuit_width:f32 = match children {
            Some(iter) => {
                iter.map(|i| tree.get(i))
                    .filter(|m| m.is_some())
                    .map(|o| o.unwrap())
                    .map(|l| l.layout(parameter,tree))
                    .sum()
            },
            None => 0.0
        };

        let margin = if self.raw_circuit {0.0} else {parameter.margin};

        let width = circuit_width + margin*2.0;

        let mut data = GuiLoopData::default();
        data.margin = margin;
        data.width = width;
        data.outline.x = margin;
        data.outline.y = -parameter.register_spacing;
        data.outline.height = parameter.full_circuit_height();
        data.outline.width = circuit_width;
        data.outline_background = Color::new(128,128,128,255);

        self.gui_data.replace(data);

        width
    }

    fn draw<T:RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, parameter:&DrawingPar, tree: &VecTree<GuiCircuitElement>) {
        let rect = self.gui_data.borrow().outline.clone();
        let raw_circuit = self.raw_circuit;

        if !raw_circuit {
            drawer.draw_rectangle_rec(&rect, self.gui_data.borrow().outline_background);
            drawer.draw_rectangle_lines_ex(&rect, parameter.register_thickness as i32, parameter.foreground_color);
        }

        drawer.draw_all_registers(parameter,self.gui_data.borrow().width);


        let children = self.index.map(|i| tree.children(i));
        if children.is_none() {
            return
        }

        let children = children.unwrap();

        drawer.push_offset();
        drawer.shift_by(self.gui_data.borrow().margin);
        drawer.push_offset();

        for idx in children {
            let element = tree.get(idx);
            match element {
                Some(e) => {
                    e.draw(drawer,parameter,tree);
                    drawer.shift_by(e.width());
                }
                None => {}
            }
        }

        drawer.pop_offset();
        drawer.pop_offset()

    }
}
