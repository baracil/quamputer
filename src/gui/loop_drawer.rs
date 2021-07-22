use raylib::color::Color;
use raylib::drawing::RaylibDraw;
use vec_tree::VecTree;

use crate::gui::{Drawable, DrawingPar};
use crate::gui::gui_circuit::{GuiCircuitElement, GuiLoop, GuiLoopData, HoverData};
use crate::gui::gui_drawer::GuiDrawer;

impl Drawable for GuiLoop {

    fn layout(&self, nb_qbits: u8, parameter: &DrawingPar, tree: &VecTree<GuiCircuitElement>) -> f32 {
        let children = self.index.map(|i| { tree.children(i) });

        let circuit_width: f32 = if let Some(iter) = children {
            iter.filter_map(|i| tree.get(i))
                .map(|l| l.layout(nb_qbits, parameter, tree))
                .sum()
        } else {
            0.0
        };

        let margin = if self.raw_circuit { 0.0 } else { parameter.margin };
        let width = circuit_width + margin * 2.0;

        let mut data = GuiLoopData::default();
        data.margin = margin;
        data.width = width;
        data.outline.x = margin;
        data.outline.y = -parameter.register_spacing;
        data.outline.height = parameter.full_circuit_height(nb_qbits);
        data.outline.width = circuit_width;
        data.outline_background = Color::new(128, 128, 128, 255);

        self.gui_data.replace(data);

        width
    }

    fn draw<T: RaylibDraw>(&self, drawer: &mut GuiDrawer<T>, nb_qbits: u8, parameter: &DrawingPar, tree: &VecTree<GuiCircuitElement>) -> Option<HoverData> {

        if !self.raw_circuit {
            drawer.draw_rectangle_rec(&self.gui_data.borrow().outline, self.gui_data.borrow().outline_background);
            drawer.draw_rectangle_lines_ex(&self.gui_data.borrow().outline, parameter.register_thickness as i32, parameter.foreground_color);
        }

        drawer.draw_all_registers(nb_qbits, parameter, self.gui_data.borrow().width);

        let children = self.index.map(|i| tree.children(i));
        if children.is_none() {
            return None;
        }

        let children = children.unwrap();

        drawer.push_offset();
        drawer.shift_by(self.gui_data.borrow().margin);

        let mut hoover_result = None;
        for idx in children {
            let element = tree.get(idx);
            if let Some(e) = element {
                let child_hoover = e.draw(drawer, nb_qbits, parameter, tree);
                hoover_result = hoover_result.or(child_hoover);
                drawer.shift_by(e.width());
            }
        }

        drawer.pop_offset();


        if hoover_result.is_none() && !self.raw_circuit {
            let transformed_outline = drawer.transform_rectangle(&self.gui_data.borrow().outline);
            let mouse_position = drawer.mouse_info.world_pos;
            let hover = transformed_outline.check_collision_point_rec(mouse_position);

            if hover {
                drawer.draw_rectangle_lines_ex(&self.gui_data.borrow().outline, parameter.register_thickness as i32, parameter.hover_color);
                return self.index.map(|index| { HoverData::for_loop(index) });
            }
        }

        hoover_result
    }
}
